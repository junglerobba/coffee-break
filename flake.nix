{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self, flake-utils, ... }@inputs:
    flake-utils.lib.eachSystem
      [
        "aarch64-darwin"
        "x86_64-darwin"
      ]
      (
        system:
        let
          pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ inputs.rust-overlay.overlays.default ];
          };
          rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          crane = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;
          src = crane.cleanCargoSource (crane.path ./.);
          buildInputs = with pkgs; [
            darwin.IOKit
            libiconv
          ];
          commonArgs = {
            inherit src buildInputs;
            strictDeps = true;
          };
          cargoArtifacts = crane.buildDepsOnly commonArgs;
          coffee-break = pkgs.callPackage ./default.nix {
            rustPlatform = pkgs.makeRustPlatform {
              cargo = rustToolchain;
              rustc = rustToolchain;
            };
          };
        in
        {
          checks = {
            clippy = crane.cargoClippy (
              commonArgs
              // {
                inherit cargoArtifacts;
                cargoClippyExtraArgs = "--all-targets -- --deny warnings";
              }
            );
            fmt = crane.cargoFmt { inherit src; };
          };
          packages = {
            inherit coffee-break;
            default = coffee-break;
          };
          apps.default = flake-utils.lib.mkApp { drv = coffee-break; };

          devShells.default = crane.devShell {
            inputsFrom = [ coffee-break ];
            packages = with pkgs; [
              rustToolchain
              rust-analyzer
            ];
          };

        }
      )
    // {
      overlays.default = final: _: { inherit (self.packages.${final.system}) coffee-break; };
    };
}
