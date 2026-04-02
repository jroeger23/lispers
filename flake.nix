{
  description = "Rust-Nix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crate2nix = {
      url = "github:nix-community/crate2nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {
    flake-parts,
    rust-overlay,
    crate2nix,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} (top: {
      systems = [
        "x86_64-linux"
      ];

      perSystem = {
        self',
        pkgs,
        system,
        ...
      }: let
        # Use rust-bin to generate the toolchain from rust-toolchain.toml
        rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        buildRustCrateForPkgs = pkgs:
          pkgs.buildRustCrate.override {
            rustc = rust-toolchain; # Use rustc from toolchain
            cargo = rust-toolchain; # Use cargo from toolchain
            defaultCrateOverrides =
              pkgs.defaultCrateOverrides
              // {
                # Fix rav1e build.rs:278 error when no CARGO_ENCODED_RUSTFLAGS is set
                rav1e = attrs: {
                  CARGO_ENCODED_RUSTFLAGS = "";
                };
                # Fix thread 'main' (222) panicked at build.rs:250:45:
                av-scenechange = attrs: {
                  CARGO_ENCODED_RUSTFLAGS = "";
                };
                # Bindgen fix
                ffmpeg-sys-next = attrs: {
                  buildInputs = [pkgs.ffmpeg_4];
                  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
                  nativeBuildInputs = [pkgs.pkg-config];
                  BINDGEN_EXTRA_CLANG_ARGS = [
                    "--sysroot=${pkgs.glibc.dev}"
                  ];
                };
                ffmpeg-next = attrs: {
                  features = ["codec" "format" "ffmpeg4" "ffmpeg_4_0" "ffmpeg_4_1" "ffmpeg_4_2" "ffmpeg_4_3" "ffmpeg_4_4" "ff_api_vaapi" "software-scaling" "software-resampling"];
                };
              };
          };

        # Cargo.nix for IFD
        generatedCargoNix = crate2nix.tools.${system}.generatedCargoNix {
          name = "rustnix";
          src = ./.;
        };

        cargoNix = import generatedCargoNix {
          inherit pkgs buildRustCrateForPkgs;
        };
      in {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [rust-overlay.overlays.default];
        };

        packages.lispers = cargoNix.workspaceMembers.lispers.build.overrideAttrs (attrs: {
          preConfigure = ''
            export LISPERS_OUT_DIR="$out"
            export LISPERS_DONT_COPY_SCENES=1
          '';
          postInstall = ''
            cp -r $src/scenes $out/scenes
          '';
        });
        packages.default = self'.packages.lispers;
        apps = {
          lisp_demo = {
            type = "app";
            program = "${self'.packages.lispers}/bin/lisp_demo";
          };
          repl = {
            type = "app";
            program = "${self'.packages.lispers}/bin/repl";
          };
          rt_demo = {
            type = "app";
            program = "${self'.packages.lispers}/bin/rt_demo";
          };
          rt_lisp_demo = {
            type = "app";
            program = "${self'.packages.lispers}/bin/rt_lisp_demo";
          };
          rt_interp = {
            type = "app";
            program = "${self'.packages.lispers}/bin/rt_interp";
          };
          default = self'.apps.rt_demo_lisp;
        };

        devShells.default = pkgs.mkShell {
          shellHook = ''
            export LISPERS_USE_LOCAL_SCENES=1
          '';
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          nativeBuildInputs = [rust-toolchain pkgs.pkg-config pkgs.ffmpeg_4];
          BINDGEN_EXTRA_CLANG_ARGS = [
            "--sysroot=${pkgs.glibc.dev}"
          ];
        };
      };
    });
}
