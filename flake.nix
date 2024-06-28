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

      # work around https://github.com/nextest-rs/nextest/issues/267
      # this needs to exist in both the devShell and preCheck phase!
      darwinNextestHack = pkgs.lib.optionalString pkgs.stdenv.isDarwin ''
        export DYLD_FALLBACK_LIBRARY_PATH=$(${ourRustVersion}/bin/rustc --print sysroot)/lib
      '';

      # NOTE (aseipp): on Linux, go ahead and use mold by default to improve
      # link times a bit; mostly useful for debug build speed, but will help
      # over time if we ever get more dependencies, too
      useMoldLinker = pkgs.stdenv.isLinux;

      # these are needed in both devShell and buildInputs
      linuxNativeDeps = with pkgs; lib.optionals stdenv.isLinux [
        mold-wrapped
      ];

    in {
      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          ourRustVersion

          cargo-bloat protobuf
          grpcurl tokio-console sqlite
          pkg-config fuse
        ] ++ darwinDeps ++ linuxNativeDeps;
        shellHook = ''
          export RUST_BACKTRACE=1
        '' + pkgs.lib.optionalString useMoldLinker ''
          export RUSTFLAGS="--cfg tokio_unstable -C link-arg=-fuse-ld=mold"
        '' + ''
          export RUSTFLAGS="-Zthreads=0 $RUSTFLAGS -C link-arg=-Wl,--compress-debug-sections=zstd"
        '' + darwinNextestHack;
      };
    });
}
