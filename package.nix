{
  lib,
  rustPlatform,
  libxkbcommon,
  xorg,
  vulkan-loader,
  wayland,
}:
let
  buildInputs = [
    libxkbcommon
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    xorg.libxcb
    vulkan-loader
    wayland
  ];
  libraryPath = lib.makeLibraryPath buildInputs;
in
rustPlatform.buildRustPackage {
  pname = "fretboard";
  version = "0.1.0";
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
  inherit buildInputs;
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
}
