{
  description = "Rust-Nix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay/6bf986d20552384209907fa0d5f3fa9a34d00995";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crate2nix.url = "github:nix-community/crate2nix";

    # Development

    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-trusted-public-keys = "eigenvalue.cachix.org-1:ykerQDDa55PGxU25CETy9wF6uVDpadGGXYrFNJA3TUs=";
    extra-substituters = "https://eigenvalue.cachix.org";
    allow-import-from-derivation = true;
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-parts,
    rust-overlay,
    crate2nix,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      imports = [
        ./nix/rust-overlay/flake-module.nix
        ./nix/devshell/flake-module.nix
      ];

      perSystem = {
        system,
        pkgs,
        lib,
        inputs',
        ...
      }: let
        cargoNix = inputs.crate2nix.tools.${system}.appliedCargoNix {
          name = "rustnix";
          src = ./.;
        };
      in rec {
        checks = {
          rustnix = cargoNix.rootCrate.build.override {
            runTests = true;
          };
        };

        apps = rec {
          demo = {
            type = "app";
            program = "${packages.default}/bin/demo";
          };
          repl = {
            type = "app";
            program = "${packages.default}/bin/repl";
          };
          default = demo;
        };

        packages = {
          rustnix = cargoNix.rootCrate.build;
          default = packages.rustnix;

          inherit (pkgs) rust-toolchain;

          rust-toolchain-versions = pkgs.writeScriptBin "rust-toolchain-versions" ''
            ${pkgs.rust-toolchain}/bin/cargo --version
            ${pkgs.rust-toolchain}/bin/rustc --version
          '';
        };
      };
    };
}
