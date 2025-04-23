{
  description = "Koeda Cup 2025";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ flake-parts, ... }:
  flake-parts.lib.mkFlake { inherit inputs; } {
    systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
    perSystem = { pkgs, system, ... }: {
      _module.args.pkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [
          inputs.fenix.overlays.default
        ];
      };

      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = "lsm";
        version = "0.1.0";

        src = ./.;

        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        nativeBuidInputs = with pkgs; [
          fenix.stable.rustc
          fenix.stable.cargo
        ];

        buildInputs = [];
      };

      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          (fenix.stable.withComponents [
            "rustc"
            "cargo"
            "rust-analyzer"
          ])
        ];
        shellHook = ''
          exec fish
        '';
      };
    };
  };
}
