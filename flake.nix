{
  description = "Advent of Code 2021 solutions";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };


  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs { inherit system overlays; };
        in
        with pkgs; {
          devShell = mkShell {
            buildInputs = [
              gnuplot
              rust-bin.stable.latest.default
            ] ++ lib.optionals stdenv.isDarwin
              (with darwin.apple_sdk.frameworks; [ CoreFoundation CoreServices ]);
          };
        }
      );
}
