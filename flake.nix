#     ____ _               _  ___  ____
#    / ___| |__   __ _  __| |/ _ \/ ___|
#   | |   | '_ \ / _` |/ _` | | | \___ \
#   | |___| | | | (_| | (_| | |_| |___) |
#    \____|_| |_|\__,_|\__,_|\___/|____/
#    https://github.com/NewDawn0/ChadOS
# 
#   @Author: NewDawn0
#   @Contributors: -
#   @License: MIT
#   
#   File: flake.nix
#   Desc: Flake configuration

{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };
  outputs = { self, flake-utils, rust-overlay, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        buildTools = with pkgs; [ cargo cargo-bootimage cargo-make qemu ];
        rustToolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
          toolchain.default.override {
            extensions = [ "rust-src" "llvm-tools-preview" ];
            targets = [ "x86_64-unknown-none" ];
          });
        shell = with pkgs;
          mkShell {
            buildInputs = with pkgs; [ coreutils eza gnugrep thefuck ];
            nativeBuildInputs = buildTools ++ [ rustToolchain ];
            RUST_BACKTRACE = 1;
            shellHook = ''
              eval $(${pkgs.thefuck}/bin/thefuck --alias fuck)
              alias grep="${pkgs.gnugrep}/bin/grep --color=auto"
              alias ls=eza
            '';
          };
      in with pkgs; {
        defaultPackage = stdenv.mkDerivation rec {
          pname = "ChadOS";
          version = "0.1.0";
          src = ./.;
          CARGO_MANIFEST_DIR = ./.;
          nativeBuildInputs = buildTools ++ [ rustToolchain ];
          buildPhase = ''
            export RUST_BACKTRACE=1
            cargo make clean 
            cargo make build
          '';
          installPhase = ''
            mkdir -p $out/bin
            cp ChadOS.img $out/lib
          '';
          meta = with lib; {
            description = "A really based OS";
            longDescription = ''
              Chad or ChadOS is a simple x86_64 Operating system build mostly in Rus.
            '';
            homepage = "https://github.com/NewDawn0/ChadOS";
            license = licenses.mit;
            maintainers = with maintainers; [ NewDawn0 ];
            platforms = platforms.all;
          };
        };
        devShells.default = shell;
      });
}
