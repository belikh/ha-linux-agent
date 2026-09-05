{
  description = "ha-linux-agent — a pluggable Home Assistant companion daemon for Linux";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      crane,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        craneLib = crane.mkLib pkgs;

        src = craneLib.cleanCargoSource ./.;

        commonArgs = {
          inherit src;
          strictDeps = true;
          buildInputs = [
            pkgs.dbus
          ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [ pkgs.pkg-config ];
          nativeBuildInputs = [ pkgs.pkg-config ];
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # doCheck = false: the supervisor regression tests need mosquitto on
        # PATH, which is provided only by the dedicated cargoTest check
        # below — one place owns test execution.
        ha-linux-agent = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            pname = "ha-linux-agent";
            doCheck = false;
          }
        );

        # The reliability gate: the workspace's tests — including the five
        # defect-named supervisor regression tests against a real mosquitto
        # subprocess. The Nix build sandbox brings up a private loopback
        # interface for every non-fixed-output derivation, so an
        # in-sandbox client-server pair needs no sandbox escape; mosquitto
        # rides nativeBuildInputs so the harness finds it on PATH.
        cargoTest = craneLib.cargoTest (
          commonArgs
          // {
            inherit cargoArtifacts;
            nativeBuildInputs = commonArgs.nativeBuildInputs ++ [ pkgs.mosquitto ];
          }
        );
      in
      {
        packages.default = ha-linux-agent;
        packages.ha-linux-agent = ha-linux-agent;

        checks = {
          inherit ha-linux-agent;
          clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            }
          );
          tests = cargoTest;
          # The report's Phase 2 gate: "the runNixOSTest gating the module
          # in both roles". The cargo tests cover the supervisor against a
          # real mosquitto subprocess; this test gates the NIXOS MODULE
          # itself — that the unit is a system service (not a user unit),
          # that both role branches evaluate into a valid closure, and
          # that the service starts and reaches active (running) with
          # WatchdogSec/StateDirectory/User as configured. The
          # environment-attrset bug that escaped to the fleet eval
          # (lib.optionals producing a list) is exactly the class this
          # gate catches before push.
          module-kiosk = pkgs.testers.runNixOSTest {
            name = "ha-linux-agent-module-kiosk";
            nodes.machine =
              { config, pkgs, ... }:
              {
                imports = [ self.nixosModules.default ];
                # A broker inside the VM: the agent must actually connect
                # (not just flail in a restart loop against a dead host)
                # for wait_for_unit to mean anything.
                services.mosquitto = {
                  enable = true;
                  listeners = [
                    {
                      port = 1883;
                      settings.allow_anonymous = true;
                      omitPasswordAuth = true;
                      # The NixOS module ALWAYS loads mosquitto_acl_file.so;
                      # an empty acl list default-denies every topic
                      # (connects succeed, PubAcks flow for publishes the
                      # broker silently discards, subscribes SubAck but
                      # deliver nothing — jupiter-os's mqtt.nix asserts on
                      # this exact trap). Open the test broker fully.
                      acl = [ "topic readwrite #" ];
                    }
                  ];
                };
                environment.systemPackages = [ pkgs.mosquitto ];
                # Test-only ordering: on real hosts the unit orders against
                # network-online.target and the broker is remote; in the VM
                # the broker is local, and the agent's first connect must
                # not race mosquitto's listener (observed: Connection
                # refused at t=20s, then a full backoff cycle).
                systemd.services.ha-linux-agent = {
                  after = [ "mosquitto.service" ];
                  wants = [ "mosquitto.service" ];
                };
                users.users.io = {
                  isNormalUser = true;
                  password = "";
                };
                services.ha-linux-agent = {
                  enable = true;
                  role = "kiosk";
                  settings = {
                    device.id = "testhost";
                    mqtt.host = "127.0.0.1";
                  };
                };
              };
            testScript = ''
              machine.start()
              machine.wait_for_unit("mosquitto.service")
              machine.wait_for_unit("ha-linux-agent.service")
              # The supervisor's contract: availability goes online within
              # a keepalive of connecting.
              machine.wait_until_succeeds(
                  "mosquitto_sub -h 127.0.0.1 -t 'ha-linux-agent/testhost/availability' -C 1 -W 30 | grep -F online"
              )
              # System manager, not user: the fleet-audit's user-unit
              # retirement, pinned.
              machine.succeed("test \"$(systemctl show ha-linux-agent -p User --value)\" = io")
              machine.succeed("test -n \"$(systemctl show ha-linux-agent -p WatchdogSec --value)\"")
              machine.fail("systemctl status ha-linux-agent --user 2>/dev/null")
            '';
          };
          module-server = pkgs.testers.runNixOSTest {
            name = "ha-linux-agent-module-server";
            nodes.machine =
              { config, pkgs, ... }:
              {
                imports = [ self.nixosModules.default ];
                services.mosquitto = {
                  enable = true;
                  listeners = [
                    {
                      port = 1883;
                      settings.allow_anonymous = true;
                      omitPasswordAuth = true;
                      # The NixOS module ALWAYS loads mosquitto_acl_file.so;
                      # an empty acl list default-denies every topic
                      # (connects succeed, PubAcks flow for publishes the
                      # broker silently discards, subscribes SubAck but
                      # deliver nothing — jupiter-os's mqtt.nix asserts on
                      # this exact trap). Open the test broker fully.
                      acl = [ "topic readwrite #" ];
                    }
                  ];
                };
                environment.systemPackages = [ pkgs.mosquitto ];
                # Test-only ordering (see the kiosk machine above).
                systemd.services.ha-linux-agent = {
                  after = [ "mosquitto.service" ];
                  wants = [ "mosquitto.service" ];
                };
                # No users.users.io declaration at all: the minimal/server
                # role path must still evaluate and the mkIf video-group
                # guard must not conjure a stub user.
                services.ha-linux-agent = {
                  enable = true;
                  role = "server";
                  user = "root";
                  settings = {
                    device.id = "testhost";
                    mqtt.host = "127.0.0.1";
                  };
                };
              };
            testScript = ''
              machine.start()
              machine.wait_for_unit("mosquitto.service")
              machine.wait_for_unit("ha-linux-agent.service")
              machine.wait_until_succeeds(
                  "mosquitto_sub -h 127.0.0.1 -t 'ha-linux-agent/testhost/availability' -C 1 -W 30 | grep -F online"
              )
              machine.succeed("test \"$(systemctl show ha-linux-agent -p User --value)\" = root")
              # Headless role: no session-bus Environment block materialises.
              machine.succeed("test -z \"$(systemctl show ha-linux-agent -p Environment --value)\"")
            '';
          };
        };

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};
          packages = [
            pkgs.rust-analyzer
            pkgs.mosquitto
          ];
        };

        apps.default = flake-utils.lib.mkApp { drv = ha-linux-agent; };
      }
    )
    // {
      nixosModules.default =
        {
          config,
          lib,
          pkgs,
          ...
        }:
        let
          cfg = config.services.ha-linux-agent;
          format = pkgs.formats.toml { };
          # The session-bus Environment block is a kiosk-role concern: it
          # only means something on a host whose user lingers with a live
          # per-user bus (the cage kiosks). Server and minimal hosts ship
          # without it — no session bus exists there to reach.
          isKiosk = cfg.role == "kiosk";
          # %U-style specifiers do not expand inside Environment=; resolve
          # the runtime dir for the configured user at eval time instead.
          agentUid = config.users.users.${cfg.user}.uid;
        in
        {
          options.services.ha-linux-agent = {
            enable = lib.mkEnableOption "ha-linux-agent, the Home Assistant companion daemon";

            package = lib.mkOption {
              type = lib.types.package;
              default = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
              description = "ha-linux-agent package to run.";
            };

            settings = lib.mkOption {
              type = format.type;
              default = { };
              description = ''
                ha-linux-agent config.toml contents, as Nix attrs. See
                packaging/config.example.toml for the full schema. At minimum
                set `mqtt.host`.
              '';
              example = lib.literalExpression ''
                {
                  mqtt.host = "10.1.1.20";
                  mqtt.username = "ha-linux-agent";
                  mqtt.password_file = "/run/secrets/mqtt_ha_linux_agent";
                }
              '';
            };

            role = lib.mkOption {
              type = lib.types.enum [
                "kiosk"
                "server"
                "minimal"
              ];
              default = "minimal";
              description = ''
                Host-class switch driving the unit shape:
                - kiosk: adds the session-bus Environment block
                  (XDG_RUNTIME_DIR + DBUS_SESSION_BUS_ADDRESS at
                  /run/user/<uid>/bus) so the agent reaches notification /
                  compositor sessions on a lingered desktop-ish host, plus
                  the udev video-group rule for backlight writes.
                - server: headless host — no session bus to reach, no
                  backlight to write; the generic backend degrades to
                  warn-and-disable where no session exists.
                - minimal: baseline unit only.
              '';
            };

            user = lib.mkOption {
              type = lib.types.str;
              default = "io";
              description = ''
                User the agent runs as. A static user, never DynamicUser:
                DynamicUser is incompatible with allocating a D-Bus service
                name and implies ProtectSystem=strict, which would block the
                agent's sysfs surface. The user must exist.
              '';
            };
          };

          config = lib.mkIf cfg.enable {
            # One system service per host — NOT a systemd --user unit. The
            # user-unit shape retired on the 2026 evidence: user units
            # cannot order against system targets, and since PR #517768
            # (May 2026) switch-to-configuration runs a full per-user switch
            # for any live user manager — restarting user targets mid-switch
            # and tearing sessions down. A system service restarts cleanly
            # on every `nixos-rebuild switch`, which the fleet's appliance
            # management requires.
            systemd.services.ha-linux-agent = {
              description = "Home Assistant companion daemon";
              wantedBy = [ "multi-user.target" ];
              after = [ "network-online.target" ];
              wants = [ "network-online.target" ];
              # Backends shell out to host tools — without an explicit
              # path, a system service gets only systemd's minimal default
              # (coreutils/findutils/grep/sed/systemd), so ZfsBackend's
              # `which zpool` detect() failed silently on ZFS-root hosts
              # (observed live on europa: pool sensors never registered).
              # systemd comes via the default set; zfs is conditional so
              # non-ZFS hosts don't drag the module in.
              path = [
                cfg.package
                pkgs.zfs
              ];
              # systemd.environment is an attrset — the kiosk block is
              # merged in with mkIf so non-kiosk hosts get {} not [].
              environment = lib.mkIf isKiosk {
                XDG_RUNTIME_DIR = "/run/user/${builtins.toString agentUid}";
                DBUS_SESSION_BUS_ADDRESS = "unix:path=/run/user/${builtins.toString agentUid}/bus";
              };
              serviceConfig = {
                ExecStart = "${cfg.package}/bin/ha-linux-agent ${format.generate "ha-linux-agent-config.toml" cfg.settings}";
                User = cfg.user;
                Restart = "on-failure";
                RestartSec = "5s";
                # sd_notify state: the last-discovery manifest lives here,
                # and WatchdogSec pairs with the agent's poll-loop pings —
                # a wedged loop stops pinging and costs one restart, not
                # the host's telemetry until the next reboot.
                StateDirectory = "ha-linux-agent";
                WatchdogSec = "15min";
              };
            };

            # Privilege model for the hardware backend's sysfs actuators, per
            # the fleet-audit finding: sysfs attributes are root:root 0644,
            # so an agent running as an unprivileged user gets EACCES on
            # every governor/EPP/backlight write — logged as a warn!,
            # invisible at the HA layer (the HA select exists, the command
            # arrives, the write dies silently; observed live on europa).
            # One udev rule per node class puts the ACTUATOR NODES ONLY
            # into the video group, via the canonical ArchWiki pattern
            # (chgrp/chmod on sysfs attribute files from a RUN+= hook —
            # sysfs supports setattr; the mode resets when the node is
            # re-created, which is why it rides udev events, not a oneshot).
            #   - backlight/brightness, never bl_power (on the TCxWave
            #     kiosks that node cuts power to a rail the touch
            #     digitiser shares). Covers the launcher backend's
            #     screen-power light too — same node is its write path.
            #   - cpufreq scaling_governor and, separately gated,
            #     energy_performance_preference — EPP is absent on
            #     acpi-cpufreq hosts (europa's Opteron), so its rule TESTs
            #     for the node before running; the boost knob joins the
            #     same pattern when Phase 3 ships it.
            # Keyed to enable, not role: any host exposing these actuators
            # needs the write path — a server's governor/EPP selects are
            # as dead as a kiosk's brightness when the rule is kiosk-only.
            services.udev.extraRules = ''
              SUBSYSTEM=="backlight", ACTION=="add|change", TEST=="brightness", RUN+="${pkgs.coreutils}/bin/chgrp video /sys/class/backlight/%k/brightness", RUN+="${pkgs.coreutils}/bin/chmod g+w /sys/class/backlight/%k/brightness"
              SUBSYSTEM=="cpu", ACTION=="add|change", TEST=="cpufreq/scaling_governor", RUN+="${pkgs.coreutils}/bin/chgrp video /sys/devices/system/cpu/%k/cpufreq/scaling_governor", RUN+="${pkgs.coreutils}/bin/chmod g+w /sys/devices/system/cpu/%k/cpufreq/scaling_governor"
              SUBSYSTEM=="cpu", ACTION=="add|change", TEST=="cpufreq/energy_performance_preference", RUN+="${pkgs.coreutils}/bin/chgrp video /sys/devices/system/cpu/%k/cpufreq/energy_performance_preference", RUN+="${pkgs.coreutils}/bin/chmod g+w /sys/devices/system/cpu/%k/cpufreq/energy_performance_preference"
            '';

            # udev applies rules to NEW events only — `nixos-rebuild switch`
            # reloads the ruleset but never re-triggers existing devices,
            # so nodes that appeared at boot keep root:root until a
            # reboot. cpufreq cores emit KOBJ_CHANGE on policy add, so a
            # synthetic change-trigger reaches every node the rules cover.
            # Boot makes this redundant (coldplug replays every event);
            # it exists to close the switch-gap on already-running hosts.
            systemd.services.ha-linux-agent-sysfs-retrigger = {
              description = "Re-trigger udev events for ha-linux-agent sysfs actuator nodes";
              after = [ "systemd-udevd.service" ];
              wantedBy = [ "multi-user.target" ];
              serviceConfig = {
                Type = "oneshot";
                ExecStart = "${config.systemd.package}/bin/udevadm trigger --subsystem-match=cpu --subsystem-match=backlight --action=change";
                RemainAfterExit = true;
              };
            };

            # A switch changes group membership and udev rules; the RUNNING
            # agent keeps its spawn-time supplementary groups until restart,
            # so a switch that adds io to video must also restart the agent
            # for the write path to actually open. The udev-retrigger unit
            # above re-applies rules to existing nodes on the same switch.
            systemd.services.ha-linux-agent.restartTriggers = [
              config.services.udev.extraRules
              config.users.users.${cfg.user}.extraGroups
            ];

            # The agent user must hold video membership for the rules above
            # to grant anything. Supplementary groups apply at ExecStart
            # spawn, so the post-switch service restart picks membership up
            # without a re-login. Unconditional (the standard NixOS module
            # pattern): merges into an existing users.users.<user>
            # declaration, and defines the user when a standalone consumer
            # hasn't — a hasAttr self-check here recurses (it forces
            # config.users.users while defining it).
            users.users.${cfg.user}.extraGroups = [ "video" ];
          };
        };
    };
}
