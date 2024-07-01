{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
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
          pkgs = import inputs.nixpkgs { inherit system; };
          crane = inputs.crane.mkLib pkgs;
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
          coffee-break = crane.buildPackage (commonArgs // { inherit cargoArtifacts; });
        in
        {
          checks = {
            inherit coffee-break;
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
            checks = self.checks.${system};
            packages = with pkgs; [ rust-analyzer ];
          };

        }
      )
    // {
      overlays.default = final: _: { inherit (self.packages.${final.system}) coffee-break; };
    };
}
