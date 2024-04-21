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

    my-rust-bin = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.complete);

    in {
      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          cargo-bloat protobuf my-rust-bin
          grpcurl tokio-console sqlite
          pkg-config fuse
        ];
      };
    });
}
