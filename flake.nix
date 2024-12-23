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
      rust-dev = (
        pkgs.rust-bin.selectLatestNightlyWith (
          toolchain:
          toolchain.minimal.override {
            extensions = [
              "rust-analyzer"
              "rust-src"
              "rustfmt"
            ];
          }
        )
      );
      libraries = [
        pkgs.libxkbcommon
        pkgs.xorg.libX11
        pkgs.xorg.libXcursor
        pkgs.xorg.libXi
        pkgs.xorg.libxcb
        pkgs.vulkan-loader
      ];
      libraryPath = pkgs.lib.makeLibraryPath libraries;
    in
    {
      nixosModules.default =
        { ... }:
        {
          config = {
            environment.systemPackages = [ inputs.self.packages.${system}.default ];
          };
        };
      packages.${system}.default =
        let
          lib = pkgs.lib;
        in
        pkgs.rustPlatform.buildRustPackage {
          pname = "fretboard";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          buildInputs = libraries;
          postFixup = ''
            library_path=$(patchelf --print-rpath $out/bin/fretboard)
            patchelf --set-rpath "$library_path:${libraryPath}" $out/bin/fretboard
          '';
          meta = {
            description = "A flexible fretboard visualization tool";
            homepage = "https://github.com/rossnomann/fretboard";
            license = lib.licenses.mit;
            mainProgram = "fretboard";
          };
        };
      devShells.${system}.default =
        let

        in
        pkgs.mkShell {
          RUST_SRC_PATH = "${rust-dev}/lib/rustlib/src/rust/library";
          buildInputs = libraries ++ [
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
