{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    # For installing non-standard rustc versions
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      };

      ourRustVersion = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.complete);

      # these are needed in both devShell and buildInputs
      darwinDeps = with pkgs; lib.optionals stdenv.isDarwin [
        darwin.apple_sdk.frameworks.Security
        darwin.apple_sdk.frameworks.SystemConfiguration
        libiconv
      ];

      # these are needed in both devShell and buildInputs
      linuxDeps = with pkgs; [
        mold-wrapped
      ];
    in {
      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          ourRustVersion

          cargo-bloat protobuf
          sqlite pkg-config fuse
        ] ++ darwinDeps ++ linuxDeps;

        shellHook = with pkgs; ''
          export RUST_BACKTRACE=1
          export RUSTFLAGS="--cfg tokio_unstable -Zthreads=0"

        '' + lib.optionalString stdenv.isLinux ''
          export RUSTFLAGS+=" -C link-arg=-fuse-ld=mold -C link-arg=-Wl,--compress-debug-sections=zstd"

        '' + lib.optionalString stdenv.isDarwin ''
          # work around https://github.com/nextest-rs/nextest/issues/267
          export DYLD_FALLBACK_LIBRARY_PATH=$(${ourRustVersion}/bin/rustc --print sysroot)/lib
          # use new fast macOS linker
          export RUSTFLAGS+=" -C link-arg=-fuse-ld=/usr/bin/ld -C link-arg=-ld_new"
        '';
      };
    });
}
