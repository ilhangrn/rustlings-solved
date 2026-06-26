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
