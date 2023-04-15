{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      };

    rust-version = pkgs.rust-bin.stable.latest.default;
    my-rust-bin = rust-version.override {
      extensions = [ "rust-analyzer" "rust-src" ];
    };

    in {
      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          cargo-bloat protobuf my-rust-bin
          grpcurl tokio-console
        ];
      };
    });
}
