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

            # Backlight write path: one udev rule putting the sysfs nodes in
            # the video group (writable by it) — the same pattern the fleet
            # already uses for the customer-display hidraw nodes. This
            # replaces the old root-chmod oneshot, which granted group-root
            # writes the agent user could never use (its only group was
            # wheel) — brightness writes failed EACCES silently. Note:
            # brightness only, never bl_power (on the TCxWave kiosks that
            # node cuts power to a rail the touch digitiser shares).
            services.udev.extraRules = lib.mkIf isKiosk ''
              SUBSYSTEM=="backlight", ACTION=="add", RUN+="${pkgs.coreutils}/bin/chgrp video /sys/class/backlight/%k/brightness", RUN+="${pkgs.coreutils}/bin/chmod g+w /sys/class/backlight/%k/brightness"
            '';
          };
        };
    };
}
