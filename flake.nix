{
  description = "Rust Development Environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      rec {
        packages.lock = pkgs.writeShellApplication {
          name = "rustlings-lock";
          runtimeInputs = [ pkgs.nix ];
          text = ''
            set -euo pipefail

            mkdir -p .nix-gc-roots

            nix build .#devShells.${system}.default --out-link .nix-gc-roots/default-devshell

            # Rustlings package build can fail while exercises are incomplete.
            if nix build .#packages.${system}.default --out-link .nix-gc-roots/default-package; then
              echo "Package GC root updated in .nix-gc-roots/default-package"
            else
              echo "Skipped package GC root: packages.default failed to build"
            fi

            echo "GC roots updated in .nix-gc-roots/"
          '';
        };

        apps.lock = utils.lib.mkApp {
          drv = packages.lock;
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          name = "my-rust-project";
          src = ./.;
          buildInputs = with pkgs; [
            openssl
          ];
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
          # cargoHash = "sha256-GMuoovINEaIAETRbLhtImLFXdFjLE5b77xtXaVx+jIc="; #32 byte hash, Replace with the actual hash after first build
          # we can use cargo lock file for hash
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };

        devShells.default = pkgs.mkShell {
          name = "rust-dev-shell";
          # Use nativeBuildInputs for tools and packages compiled for the host architecture
          nativeBuildInputs = with pkgs; [
            # Rust Toolchain
            cargo
            rustc
            rustfmt
            clippy
            rust-analyzer

            # Build tools
            pkg-config
          ];

          # Use buildInputs for libraries that your Rust binaries link against
          buildInputs = with pkgs; [
            openssl
          ];

          # Native Nix declaration for environment variables (replaces shellHook)
          env = {
            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

            # Ensures pkg-config can dynamically find OpenSSL on your system path
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          };
        };
      }
    );
}
