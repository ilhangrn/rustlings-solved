 let
   nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-26.05";
   pkgs = import nixpkgs { config = {}; overlays = []; };
 in

 pkgs.mkShellNoCC {
   packages = with pkgs; [
     cowsay
     lolcat
     cargo
     rustc
     rust-analyzer
     rustfmt
   ];

   GREETING = "Hello, IG_NIX!";
   LANGUAGE = "EN";

  shellHook = ''
        export LOCALE_ARCHIVE="${pkgs.glibcLocales}/lib/locale/locale-archive";
            echo $GREETING | cowsay | lolcat;
            rustc --version;
  '';
 }