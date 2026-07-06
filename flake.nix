{
  description = "Rust Development Environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          name = "my-rust-project";
          src = ./.;
          buildInputs = with pkgs; [
            openssl
          ];
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
          cargoHash = "sha256-pZGm1Av0IEBKARczz7exkNYsZb8LzaMrV7J32a2fFG4="; #32 byte hash, Replace with the actual hash after first build
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
      });
}
