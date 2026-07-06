use async_trait::async_trait;
use ha_agent_core::config::HardwareBackendConfig;
use ha_agent_core::model::{CommandDescriptor, SensorDescriptor, SensorState};
use ha_agent_core::{CommandBackend, SensorBackend};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{info, warn};

pub struct HardwareBackend {
    config: HardwareBackendConfig,
    backlight_path: Option<PathBuf>,
    temp_input_path: Option<PathBuf>,
    available_governors: Vec<String>,
    available_epps: Vec<String>,
}

impl HardwareBackend {
    pub fn detect() -> bool {
        // Always return true since at least some sysfs interface or temp sensor will be present on any Linux system.
        true
    }

    pub fn new(config: HardwareBackendConfig) -> Self {
        // 1. Detect backlight device
        let backlight_path = if let Some(ref dev) = config.backlight_device {
            let p = PathBuf::from("/sys/class/backlight").join(dev);
            if p.exists() {
                Some(p)
            } else {
                warn!("configured backlight device '{}' not found", dev);
                None
            }
        } else {
            // Auto-detect first entry in /sys/class/backlight
            fs::read_dir("/sys/class/backlight")
                .ok()
                .and_then(|mut entries| {
                    entries.find_map(|entry| {
                        entry.ok().map(|e| e.path())
                    })
                })
        };

        if let Some(ref path) = backlight_path {
            info!("hardware backend: detected backlight device at {:?}", path);
        }

        // 2. Detect cpu temperature sensor (coretemp hwmon)
        let mut temp_input_path = None;
        if let Ok(entries) = fs::read_dir("/sys/class/hwmon") {
            for entry in entries.flatten() {
                let path = entry.path();
                let name_file = path.join("name");
                if let Ok(name) = fs::read_to_string(&name_file) {
                    if name.trim() == "coretemp" {
                        // Look for temp1_input (usually package id 0)
                        let t1 = path.join("temp1_input");
                        if t1.exists() {
                            temp_input_path = Some(t1);
                            break;
                        }
                    }
                }
            }
        }
        if temp_input_path.is_none() {
            // Fallback: look for any thermal zone temp
            let tz = PathBuf::from("/sys/class/thermal/thermal_zone0/temp");
            if tz.exists() {
                temp_input_path = Some(tz);
            }
        }
        if let Some(ref path) = temp_input_path {
            info!("hardware backend: detected temperature input at {:?}", path);
        }

        // 3. Detect available scaling governors
        let available_governors = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors")
            .map(|s| s.split_whitespace().map(|x| x.to_string()).collect())
            .unwrap_or_else(|_| vec!["powersave".to_string(), "performance".to_string()]);

        // 4. Detect available EPP values
        let available_epps = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/energy_performance_available_preferences")
            .map(|s| s.split_whitespace().map(|x| x.to_string()).collect())
            .unwrap_or_else(|_| vec![
                "default".to_string(),
                "performance".to_string(),
                "balance_performance".to_string(),
                "balance_power".to_string(),
                "power".to_string(),
            ]);

        Self {
            config,
            backlight_path,
            temp_input_path,
            available_governors,
            available_epps,
        }
    }

    fn read_sys_file(&self, path: &Path) -> anyhow::Result<String> {
        let content = fs::read_to_string(path)?;
        Ok(content.trim().to_string())
    }

    fn write_sys_file(&self, path: &Path, val: &str) -> anyhow::Result<()> {
        fs::write(path, val)?;
        Ok(())
    }

    fn write_cpu_files(&self, filename: &str, val: &str) -> anyhow::Result<()> {
        let cpus = fs::read_dir("/sys/devices/system/cpu")?;
        for entry in cpus.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with("cpu") && name.chars().nth(3).map_or(false, |c| c.is_ascii_digit()) {
                let target = entry.path().join("cpufreq").join(filename);
                if target.exists() {
                    if let Err(e) = self.write_sys_file(&target, val) {
                        warn!("failed to write {} to {:?}: {}", val, target, e);
                    }
                }
            }
        }
        Ok(())
    }
}

#[async_trait]
impl SensorBackend for HardwareBackend {
    fn id(&self) -> &str {
        "hardware"
    }

    fn sensors(&self) -> Vec<SensorDescriptor> {
        let mut sensors = Vec::new();

        if self.temp_input_path.is_some() {
            sensors.push(
                SensorDescriptor::sensor("cpu_temperature", "CPU Temperature")
                    .with_unit("°C")
                    .with_device_class("temperature")
                    .with_icon("mdi:thermometer"),
            );
        }

        // We report state of controls so Home Assistant can read them
        if self.backlight_path.is_some() {
            sensors.push(
                SensorDescriptor::sensor("backlight_brightness", "Display Brightness")
                    .with_unit("%")
                    .with_icon("mdi:brightness-6"),
            );
        }

        if self.config.cpu_governor && Path::new("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor").exists() {
            sensors.push(
                SensorDescriptor::sensor("cpu_governor", "CPU Governor")
                    .with_icon("mdi:speedometer"),
            );
        }

        if self.config.cpu_epp && Path::new("/sys/devices/system/cpu/cpu0/cpufreq/energy_performance_preference").exists() {
            sensors.push(
                SensorDescriptor::sensor("cpu_energy_performance_preference", "CPU Energy Preference")
                    .with_icon("mdi:leaf"),
            );
        }

        sensors
    }

    async fn poll(&self) -> Vec<SensorState> {
        let mut states = Vec::new();

        if let Some(ref path) = self.temp_input_path {
            if let Ok(raw) = self.read_sys_file(path) {
                if let Ok(temp_raw) = raw.parse::<f64>() {
                    // hwmon temp is usually in millidegrees, thermal_zone might be too.
                    // If it is > 1000, we divide by 1000.
                    let temp = if temp_raw > 1000.0 { temp_raw / 1000.0 } else { temp_raw };
                    states.push(SensorState::new("cpu_temperature", (temp * 10.0).round() / 10.0));
                }
            }
        }

        if let Some(ref path) = self.backlight_path {
            let bright_file = path.join("brightness");
            let max_file = path.join("max_brightness");
            if let (Ok(bright_str), Ok(max_str)) = (self.read_sys_file(&bright_file), self.read_sys_file(&max_file)) {
                if let (Ok(bright), Ok(max)) = (bright_str.parse::<f64>(), max_str.parse::<f64>()) {
                    if max > 0.0 {
                        let pct = (bright * 100.0 / max).round();
                        states.push(SensorState::new("backlight_brightness", pct));
                    }
                }
            }
        }

        if self.config.cpu_governor {
            let path = Path::new("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor");
            if let Ok(gov) = self.read_sys_file(path) {
                states.push(SensorState::new("cpu_governor", gov));
            }
        }

        if self.config.cpu_epp {
            let path = Path::new("/sys/devices/system/cpu/cpu0/cpufreq/energy_performance_preference");
            if let Ok(epp) = self.read_sys_file(path) {
                states.push(SensorState::new("cpu_energy_performance_preference", epp));
            }
        }

        states
    }
}

#[async_trait]
impl CommandBackend for HardwareBackend {
    fn id(&self) -> &str {
        "hardware"
    }

    fn commands(&self) -> Vec<CommandDescriptor> {
        let mut cmds = Vec::new();

        if self.backlight_path.is_some() {
            cmds.push(
                CommandDescriptor::number("backlight_brightness", "Set Display Brightness", 0.0, 100.0)
                    .with_icon("mdi:brightness-6")
            );
        }

        if self.config.cpu_governor && Path::new("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor").exists() {
            cmds.push(
                CommandDescriptor::select("cpu_governor", "Set CPU Governor", self.available_governors.clone())
                    .with_icon("mdi:speedometer")
            );
        }

        if self.config.cpu_epp && Path::new("/sys/devices/system/cpu/cpu0/cpufreq/energy_performance_preference").exists() {
            cmds.push(
                CommandDescriptor::select("cpu_energy_performance_preference", "Set CPU Energy Preference", self.available_epps.clone())
                    .with_icon("mdi:leaf")
            );
        }

        cmds
    }

    async fn handle(&self, command_id: &str, payload: &str) -> anyhow::Result<()> {
        match command_id {
            "backlight_brightness" => {
                let backlight_path = self.backlight_path.as_ref()
                    .ok_or_else(|| anyhow::anyhow!("no backlight device detected"))?;
                
                let pct = payload.trim().parse::<f64>()?;
                let max_str = self.read_sys_file(&backlight_path.join("max_brightness"))?;
                let max = max_str.parse::<f64>()?;
                
                let raw_val = ((pct * max) / 100.0).round() as u64;
                self.write_sys_file(&backlight_path.join("brightness"), &raw_val.to_string())?;
                Ok(())
            }
            "cpu_governor" => {
                let gov = payload.trim().to_string();
                if !self.available_governors.contains(&gov) {
                    return Err(anyhow::anyhow!("unsupported governor: {}", gov));
                }
                self.write_cpu_files("scaling_governor", &gov)?;
                Ok(())
            }
            "cpu_energy_performance_preference" => {
                let epp = payload.trim().to_string();
                if !self.available_epps.contains(&epp) {
                    return Err(anyhow::anyhow!("unsupported EPP: {}", epp));
                }
                self.write_cpu_files("energy_performance_preference", &epp)?;
                Ok(())
            }
            other => Err(anyhow::anyhow!("unknown hardware command: {other}")),
        }
    }
}
