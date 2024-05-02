{
  description = "An implementation of Tic-tac-toe and Ultimate Tic-tac-toe in Rust with egui";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    flake-parts.url = "github:hercules-ci/flake-parts";

    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.pre-commit-hooks.flakeModule
      ];

      systems = ["x86_64-linux" "aarch64-linux"];
      perSystem = {
        config,
        system,
        ...
      }: let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [(import inputs.rust-overlay)];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default;

        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;
        src = pkgs.lib.cleanSourceWith {
          src = ./.;
          filter = path: type:
            (pkgs.lib.hasSuffix "\.html" path)
            || (pkgs.lib.hasSuffix "\.ico" path)
            || (craneLib.filterCargoSources path type);
        };

        commonArgsNativeBuildInputs = with pkgs; [
          cmake
          pkg-config
        ];

        commonArgsBuildInputs = with pkgs; [
          expat.dev
          fontconfig.dev
          freetype.dev
        ];

        graphicalBuildInputs = with pkgs; [
          libGL
          libxkbcommon
          xorg.libX11
          xorg.libxcb
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          wayland
        ];

        commonArgs = {
          pname = "tic-tac-toe";
          inherit src;
          strictDeps = true;
          doCheck = false;

          # We set these here because we need to compile fonts and system
          # library stuff for cargoArtifacts, which gets built before any
          # of the packages
          nativeBuildInputs = commonArgsNativeBuildInputs;
          buildInputs = commonArgsBuildInputs;
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        individualCrateArgs =
          commonArgs
          // {
            inherit cargoArtifacts;
            inherit (craneLib.crateNameFromCargoToml {inherit src;}) version;
          };
      in rec {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs =
            [
              (rustToolchain.override {
                extensions = ["rust-analyzer" "rust-src" "rust-std"];
              })
              pkgs.cargo-nextest
            ]
            ++ graphicalBuildInputs;
          shellHook = ''
            ${config.pre-commit.installationScript}
          '';
        };

        # See https://flake.parts/options/pre-commit-hooks-nix and
        # https://github.com/cachix/git-hooks.nix/blob/master/modules/hooks.nix
        # for all the available hooks and options
        pre-commit.settings.hooks = {
          check-added-large-files.enable = true;
          check-merge-conflicts.enable = true;
          check-toml.enable = true;
          check-vcs-permalinks.enable = true;
          check-yaml.enable = true;
          end-of-file-fixer.enable = true;
          trim-trailing-whitespace.enable = true;

          rustfmt = {
            enable = true;
            packageOverrides = {
              cargo = rustToolchain;
              rustfmt = rustToolchain;
            };
          };
        };

        checks = {
          inherit (packages) native web doc;

          clippy = craneLib.cargoClippy (commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            });

          fmt = craneLib.cargoFmt {
            inherit src;
          };

          nextest = craneLib.cargoNextest (commonArgs
            // {
              inherit cargoArtifacts;
              partitions = 1;
              partitionType = "count";
            });
        };

        packages = {
          native = craneLib.buildPackage (individualCrateArgs
            // {
              nativeBuildInputs = commonArgsNativeBuildInputs ++ [pkgs.makeWrapper];
              buildInputs = commonArgsBuildInputs ++ graphicalBuildInputs;
              meta.mainProgram = "tictactoe";
              postInstall = ''
                wrapProgram $out/bin/tictactoe \
                  --prefix LD_LIBRARY_PATH : "${pkgs.lib.makeLibraryPath graphicalBuildInputs}"
              '';
            });

          # web = let
          #   rustToolchainWasm = rustToolchain.override {
          #     targets = ["wasm32-unknown-unknown"];
          #   };
          #   craneLibTrunk =
          #     ((inputs.crane.mkLib pkgs).overrideToolchain rustToolchainWasm)
          #     .overrideScope (_: _: {inherit (pkgs) wasm-bindgen-cli;});
          # in
          #   craneLibTrunk.buildTrunkPackage (individualCrateArgs
          #     // {
          #       trunkIndexPath = "index.html";
          #       CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
          #       inherit (pkgs) wasm-bindgen-cli;
          #     });

          doc = craneLib.cargoDoc (commonArgs
            // {
              inherit cargoArtifacts;
              cargoDocExtraArgs = "--no-deps --document-private-items --workspace";
              RUSTDOCFLAGS = "--deny warnings";
            });
        };
      };
    };
}
