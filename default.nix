{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage rec {
	pname = "rustlings";
	version = "6.5.0";

	src = ./.;

	cargoLock = {
		lockFile = ./Cargo.lock;
	};

	nativeBuildInputs = with pkgs; [
		pkg-config
		installShellFiles
	];

	# Rustlings includes many binaries/exercises; tests are not required for package builds.
	doCheck = false;

	meta = with pkgs.lib; {
		description = "Small exercises to get you used to reading and writing Rust code";
		homepage = "https://github.com/rust-lang/rustlings";
		license = licenses.mit;
		maintainers = [ ];
		platforms = platforms.all;
		mainProgram = "rustlings";
	};
}
