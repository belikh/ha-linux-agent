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

        ha-linux-agent = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            pname = "ha-linux-agent";
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
          system = pkgs.stdenv.hostPlatform.system;
          format = pkgs.formats.toml { };
        in
        {
          options.services.ha-linux-agent = {
            enable = lib.mkEnableOption "ha-linux-agent, the Home Assistant companion daemon";

            package = lib.mkOption {
              type = lib.types.package;
              default = self.packages.${system}.default;
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
          };

          config = lib.mkIf cfg.enable {
            systemd.user.services.ha-linux-agent = {
              description = "Home Assistant companion daemon";
              wantedBy = [ "default.target" ];
              after = [ "network-online.target" ];
              serviceConfig = {
                ExecStart = "${cfg.package}/bin/ha-linux-agent ${format.generate "ha-linux-agent-config.toml" cfg.settings}";
                Restart = "on-failure";
                RestartSec = 5;
              };
            };
          };
        };
    };
}
