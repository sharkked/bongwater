{
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
        pkgs,
        system,
        ...
      }: let
        rustc = (pkgs.rust-bin.stable.latest.default).override {
          extensions = ["rust-src"];
        };

        buildDeps = with pkgs; [pkg-config];
        devDeps = with pkgs; [git];

        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

        mkRustPackage =
          (pkgs.makeRustPlatform {
            inherit rustc;
            cargo = rustc;
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

        packages.default = mkRustPackage;

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = buildDeps ++ devDeps ++ [rustc];
          shellHook = ''
            export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
          '';
        };
      };
    };
}
