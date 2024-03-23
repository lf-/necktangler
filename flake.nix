{
  description = "hydra branch curser";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ self.overlays.default ];
          };
        in
        {
          packages = rec {
            necktangler = pkgs.necktangler;
            default = necktangler;
          };
          checks = self.packages.${system};

          # for debugging
          # inherit pkgs;

          devShells.default = pkgs.necktangler.overrideAttrs (
            old: {
              # make rust-analyzer work
              RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;

              nativeBuildInputs = old.nativeBuildInputs or [] ++ [
                pkgs.rust-analyzer
              ];
            }
          );
        }
      )
    // {
      overlays.default = (final: prev: {
        necktangler = final.callPackage ./package.nix { };
      });
    };
}
