{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs =
    inputs:
    let
      system = "x86_64-linux";
      overlays = [ inputs.rust-overlay.overlays.default ];
      pkgs = import inputs.nixpkgs { inherit system overlays; };
      rust-dev = pkgs.rust-bin.selectLatestNightlyWith (
        toolchain:
        toolchain.minimal.override {
          extensions = [
            "rust-analyzer"
            "rust-src"
            "rustfmt"
          ];
        }
      );
      defaultPackage = pkgs.callPackage ./package.nix { };
      libraryPath = pkgs.lib.makeLibraryPath defaultPackage.buildInputs;
    in
    {
      overlays.${system}.default = final: _: {
        fretboard = final.callPackage ./package.nix { };
      };
      packages.${system}.default = defaultPackage;
      devShells.${system}.default = pkgs.mkShell {
        RUST_SRC_PATH = "${rust-dev}/lib/rustlib/src/rust/library";
        buildInputs = defaultPackage.buildInputs ++ [
          pkgs.pkg-config
          (pkgs.lib.hiPrio (
            pkgs.rust-bin.stable.latest.minimal.override {
              extensions = [
                "rust-docs"
                "clippy"
              ];
            }
          ))
          rust-dev
        ];
        shellHook = ''
          export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${libraryPath}
          export CARGO_HOME="$PWD/.cargo"
          export PATH="$CARGO_HOME/bin:$PATH"
          export RUST_LOG=info
          mkdir -p .cargo
          echo '*' > .cargo/.gitignore
        '';
      };
    };
}
