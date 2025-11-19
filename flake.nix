{
  description = "my lovely discord bot son that i hate";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      perSystem = {
        self',
        pkgs,
        system,
        ...
      }: let
        rustTarget = (pkgs.rust-bin.stable.latest.default).override {
          extensions = ["rust-src"];
        };

        buildDeps = with pkgs; [pkg-config];
        devDeps = with pkgs; [git];

        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

        mkRustPackage =
          (pkgs.makeRustPlatform {
            rustc = rustTarget;
            cargo = rustTarget;
          }).buildRustPackage {
            inherit (cargoToml.package) name version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = buildDeps;
          };
      in {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [(import inputs.rust-overlay)];
        };

        packages.default = self'.packages.bongwater;

        packages.bongwater = mkRustPackage;

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = buildDeps ++ devDeps ++ [rustTarget];
          shellHook = ''
            export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
          '';
        };
      };
    };
}
