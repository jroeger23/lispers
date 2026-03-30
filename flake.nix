{
  description = "Rust-Nix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    crate2nix = {
      url = "github:nix-community/crate2nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-trusted-public-keys = "eigenvalue.cachix.org-1:ykerQDDa55PGxU25CETy9wF6uVDpadGGXYrFNJA3TUs=";
    extra-substituters = "https://eigenvalue.cachix.org";
    allow-import-from-derivation = true;
  };

  outputs = {
    crate2nix,
    flake-utils,
    nixpkgs,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        # Overlay pkgs with rust-bin
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

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
      in rec {
        apps = rec {
          lisp_demo = {
            type = "app";
            program = "${packages.default}/bin/lisp_demo";
          };
          repl = {
            type = "app";
            program = "${packages.default}/bin/repl";
          };
          rt_demo = {
            type = "app";
            program = "${packages.default}/bin/rt_demo";
          };
          rt_demo_lisp = {
            type = "app";
            program = "${packages.default}/bin/rt_lisp_demo";
          };
          rt_interp = {
            type = "app";
            program = "${packages.default}/bin/rt_interp";
          };
          default = rt_demo_lisp;
        };
        packages = rec {
          lispers = cargoNix.workspaceMembers.lispers.build;
          default = lispers;
        };
        devShell = pkgs.mkShell {
          buildInputs = [rust-toolchain pkgs.cargo-edit pkgs.pkg-config pkgs.ffmpeg.dev pkgs.ffmpeg];
        };
      }
    );
}
