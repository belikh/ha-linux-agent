use async_trait::async_trait;
use ha_agent_core::config::HardwareBackendConfig;
use ha_agent_core::model::{CommandDescriptor, SensorDescriptor, SensorState};
use ha_agent_core::{CommandBackend, SensorBackend};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{info, warn};

pub struct HardwareBackend {
    config: HardwareBackendConfig,
    /// Root of the sysfs tree this backend reads — `/sys` in production,
    /// a synthetic tempfile tree in tests (std::fs is a module, not a
    /// mockable trait, so path injection is the canonical seam).
    sysfs_root: PathBuf,
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
        Self::with_root(config, PathBuf::from("/sys"))
    }

    /// Constructor with an explicit sysfs root — the test seam. Every path
    /// the backend touches derives from `root`, so a synthetic tree exercises
    /// the exact production parsing.
    pub fn with_root(config: HardwareBackendConfig, root: PathBuf) -> Self {
        // 1. Detect backlight device
        let backlight_path = if let Some(ref dev) = config.backlight_device {
            let p = root.join("class/backlight").join(dev);
            if p.exists() {
                Some(p)
            } else {
                warn!("configured backlight device '{}' not found", dev);
                None
            }
        } else {
            // Auto-detect first entry in <root>/class/backlight
            fs::read_dir(root.join("class/backlight"))
                .ok()
                .and_then(|mut entries| entries.find_map(|entry| entry.ok().map(|e| e.path())))
        };

        if let Some(ref path) = backlight_path {
            info!("hardware backend: detected backlight device at {:?}", path);
        }

        // 2. Detect cpu temperature sensor (coretemp hwmon)
        let mut temp_input_path = None;
        if let Ok(entries) = fs::read_dir(root.join("class/hwmon")) {
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
            let tz = root.join("class/thermal/thermal_zone0/temp");
            if tz.exists() {
                temp_input_path = Some(tz);
            }
        }
        if let Some(ref path) = temp_input_path {
            info!("hardware backend: detected temperature input at {:?}", path);
        }

        // 3. Detect available scaling governors
        let available_governors = fs::read_to_string(
            root.join("devices/system/cpu/cpu0/cpufreq/scaling_available_governors"),
        )
        .map(|s| s.split_whitespace().map(|x| x.to_string()).collect())
        .unwrap_or_else(|_| vec!["powersave".to_string(), "performance".to_string()]);

        // 4. Detect available EPP values
        let available_epps = fs::read_to_string(
            root.join("devices/system/cpu/cpu0/cpufreq/energy_performance_available_preferences"),
        )
        .map(|s| s.split_whitespace().map(|x| x.to_string()).collect())
        .unwrap_or_else(|_| {
            vec![
                "default".to_string(),
                "performance".to_string(),
                "balance_performance".to_string(),
                "balance_power".to_string(),
                "power".to_string(),
            ]
        });

        Self {
            config,
            sysfs_root: root,
            backlight_path,
            temp_input_path,
            available_governors,
            available_epps,
        }
    }


    /// Path helper for the cpu0 cpufreq node the capability checks and
    /// reads share.
    fn cpufreq_path(&self, leaf: &str) -> PathBuf {
        self.sysfs_root
            .join("devices/system/cpu/cpu0/cpufreq")
            .join(leaf)
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
        let cpus = fs::read_dir(self.sysfs_root.join("devices/system/cpu"))?;
        for entry in cpus.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with("cpu") && name.chars().nth(3).is_some_and(|c| c.is_ascii_digit())
            {
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
        if self.config.backlight && self.backlight_path.is_some() {
            sensors.push(
                SensorDescriptor::sensor("backlight_brightness", "Display Brightness")
                    .with_unit("%")
                    .with_icon("mdi:brightness-6"),
            );
        }

        if self.config.cpu_governor
            && self.cpufreq_path("scaling_governor").exists()
        {
            sensors.push(
                SensorDescriptor::sensor("cpu_governor", "CPU Governor")
                    .with_icon("mdi:speedometer"),
            );
        }

        if self.config.cpu_epp && self.cpufreq_path("energy_performance_preference").exists() {
            sensors.push(
                SensorDescriptor::sensor(
                    "cpu_energy_performance_preference",
                    "CPU Energy Preference",
                )
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
                    let temp = if temp_raw > 1000.0 {
                        temp_raw / 1000.0
                    } else {
                        temp_raw
                    };
                    states.push(SensorState::new(
                        "cpu_temperature",
                        (temp * 10.0).round() / 10.0,
                    ));
                }
            }
        }

        if self.config.backlight {
            if let Some(ref path) = self.backlight_path {
                let bright_file = path.join("brightness");
                let max_file = path.join("max_brightness");
                if let (Ok(bright_str), Ok(max_str)) = (
                    self.read_sys_file(&bright_file),
                    self.read_sys_file(&max_file),
                ) {
                    if let (Ok(bright), Ok(max)) =
                        (bright_str.parse::<f64>(), max_str.parse::<f64>())
                    {
                        if max > 0.0 {
                            let pct = (bright * 100.0 / max).round();
                            states.push(SensorState::new("backlight_brightness", pct));
                        }
                    }
                }
            }
        }

        if self.config.cpu_governor {
            let path = self.cpufreq_path("scaling_governor");
            if let Ok(gov) = self.read_sys_file(&path) {
                states.push(SensorState::new("cpu_governor", gov));
            }
        }

        if self.config.cpu_epp {
            let path = self.cpufreq_path("energy_performance_preference");
            if let Ok(epp) = self.read_sys_file(&path) {
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

        if self.config.backlight && self.backlight_path.is_some() {
            cmds.push(
                CommandDescriptor::number(
                    "backlight_brightness",
                    "Set Display Brightness",
                    0.0,
                    100.0,
                )
                .with_icon("mdi:brightness-6"),
            );
        }

        if self.config.cpu_governor
            && self.cpufreq_path("scaling_governor").exists()
        {
            cmds.push(
                CommandDescriptor::select(
                    "cpu_governor",
                    "Set CPU Governor",
                    self.available_governors.clone(),
                )
                .with_icon("mdi:speedometer"),
            );
        }

        if self.config.cpu_epp && self.cpufreq_path("energy_performance_preference").exists() {
            cmds.push(
                CommandDescriptor::select(
                    "cpu_energy_performance_preference",
                    "Set CPU Energy Preference",
                    self.available_epps.clone(),
                )
                .with_icon("mdi:leaf"),
            );
        }

        cmds
    }

    async fn handle(&self, command_id: &str, payload: &str) -> anyhow::Result<()> {
        match command_id {
            "backlight_brightness" => {
                let backlight_path = self
                    .backlight_path
                    .as_ref()
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

#[cfg(test)]
mod tests {
    use super::*;
    use ha_agent_core::config::HardwareBackendConfig;
    use std::fs;

    /// Build a synthetic sysfs tree exercising every surface the backend
    /// probes: a backlight device, a coretemp hwmon node, and cpu0 cpufreq
    /// with real governor/EPP option lists.
    fn synthetic_sysfs() -> tempfile::TempDir {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();

        let bl = root.join("class/backlight/intel_backlight");
        fs::create_dir_all(&bl).unwrap();
        fs::write(bl.join("brightness"), "500").unwrap();
        fs::write(bl.join("max_brightness"), "1000").unwrap();

        let hwmon = root.join("class/hwmon/hwmon0");
        fs::create_dir_all(&hwmon).unwrap();
        fs::write(hwmon.join("name"), "coretemp").unwrap();
        fs::write(hwmon.join("temp1_input"), "45230").unwrap();

        let cpufreq = root.join("devices/system/cpu/cpu0/cpufreq");
        fs::create_dir_all(&cpufreq).unwrap();
        fs::write(
            cpufreq.join("scaling_available_governors"),
            "powersave performance\n",
        )
        .unwrap();
        fs::write(
            cpufreq.join("energy_performance_available_preferences"),
            "default performance balance_performance balance_power power\n",
        )
        .unwrap();
        fs::write(cpufreq.join("scaling_governor"), "performance").unwrap();
        fs::write(cpufreq.join("energy_performance_preference"), "performance").unwrap();

        // A second cpu dir proving write_cpu_files' cpu<N> filter skips
        // non-cpu entries (e.g. "cpuidle", "cpufreq" the directory).
        fs::create_dir_all(root.join("devices/system/cpu/cpuidle")).unwrap();
        dir
    }

    fn test_config() -> HardwareBackendConfig {
        HardwareBackendConfig {
            enable: true,
            backlight_device: None,
            backlight: true,
            cpu_governor: true,
            cpu_epp: true,
        }
    }

    #[test]
    fn synthetic_tree_yields_all_sensor_ids() {
        let dir = synthetic_sysfs();
        let backend = HardwareBackend::with_root(test_config(), dir.path().to_path_buf());
        let ids: Vec<String> = backend.sensors().iter().map(|s| s.id.clone()).collect();
        assert!(ids.contains(&"cpu_temperature".to_string()), "{ids:?}");
        assert!(ids.contains(&"backlight_brightness".to_string()), "{ids:?}");
        assert!(ids.contains(&"cpu_governor".to_string()), "{ids:?}");
        assert!(
            ids.contains(&"cpu_energy_performance_preference".to_string()),
            "{ids:?}"
        );
    }

    #[test]
    fn options_are_sysfs_enumerated_not_hardcoded() {
        // Regression guard: when sysfs carries the option lists, they come
        // from the tree (sorted, verbatim) — the hardcoded vecs are a
        // read-failure fallback only.
        let dir = synthetic_sysfs();
        let backend = HardwareBackend::with_root(test_config(), dir.path().to_path_buf());
        let cmds = backend.commands();
        let governor = cmds.iter().find(|c| c.id == "cpu_governor").expect("governor select");
        assert_eq!(
            governor.options.as_ref().expect("options"),
            &vec!["powersave".to_string(), "performance".to_string()]
        );
    }

    #[tokio::test]
    async fn poll_parses_millidegrees_and_backlight_percent() {
        let dir = synthetic_sysfs();
        let backend = HardwareBackend::with_root(test_config(), dir.path().to_path_buf());
        let states = backend.poll().await;
        let temp = states.iter().find(|s| s.id == "cpu_temperature").expect("temp");
        assert_eq!(temp.value, serde_json::json!(45.2)); // 45230 millidegrees
        let bl = states
            .iter()
            .find(|s| s.id == "backlight_brightness")
            .expect("backlight");
        assert_eq!(bl.value, serde_json::json!(50.0)); // 500/1000 as f64 percent
    }

    #[tokio::test]
    async fn empty_tree_publishes_nothing_and_does_not_panic() {
        let dir = tempfile::tempdir().unwrap();
        let backend = HardwareBackend::with_root(test_config(), dir.path().to_path_buf());
        assert!(backend.sensors().is_empty());
        let states = backend.poll().await;
        assert!(states.is_empty());
        // Commands against absent nodes: writes must fail with an error,
        // never panic.
        let err = backend
            .handle("backlight_brightness", "42")
            .await
            .expect_err("no backlight");
        assert!(!err.to_string().is_empty());
    }

    #[tokio::test]
    async fn brightness_write_rejected_outside_options_but_ok_inside() {
        let dir = synthetic_sysfs();
        let root = dir.path().to_path_buf();
        // Make the node writable by the test user (real sysfs permission
        // modelling is the udev rule's job, not the backend's).
        let bl = root.join("class/backlight/intel_backlight");
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(bl.join("brightness"), fs::Permissions::from_mode(0o644)).unwrap();

        let backend = HardwareBackend::with_root(test_config(), root);
        backend
            .handle("backlight_brightness", "100") // 100% of 1000 = 1000 raw
            .await
            .expect("in-range write succeeds");
        assert_eq!(
            fs::read_to_string(dir.path().join("class/backlight/intel_backlight/brightness"))
                .unwrap(),
            "1000"
        );
        backend
            .handle("cpu_governor", "turbo-ultra") // not in options
            .await
            .expect_err("unknown governor rejected");
    }
}
