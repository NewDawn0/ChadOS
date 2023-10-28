# ____ _               _  ___  ____
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
#   Desc: Nix dev-shell flake config
{
  description = "ChadOS source tree";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in with pkgs; {
        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config
            eza
            fd
            cargo-make
            (rust-bin.fromRustupToolchainFile ./.build/toolchain.toml)
          ];

          shellHook = ''
            alias ls=eza
            alias find=fd
          '';
        };
      });
}
