#{
#  inputs = {
#    naersk.url = "github:nix-community/naersk/master";
#    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
#    utils.url = "github:numtide/flake-utils";
#  };
#
#  outputs = { self, nixpkgs, utils, naersk }:
#    utils.lib.eachDefaultSystem (system:
#      let
#        pkgs = import nixpkgs { inherit system; };
#        naersk-lib = pkgs.callPackage naersk { };
#      in
#      {
#        defaultPackage = naersk-lib.buildPackage ./.;
#        devShell = with pkgs; mkShell {
#          buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy ];
#          RUST_SRC_PATH = rustPlatform.rustLibSrc;
#          LOCALE_ARCHIVE=/usr/lib/locale/locale-archive;
#        };
#      });
#}
{
  inputs = {
      nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
      flake-utils.url = "github:numtide/flake-utils";
      rust-overlay.url = "github:oxalica/rust-overlay";
    };
  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.stable.latest.default;
      in {
        devShell = pkgs.mkShell {
          buildInputs =
            [ (rustVersion.override { extensions = [ "rust-src" ]; }) ];
        };
      });
}
