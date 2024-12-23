# Fretboard

A flexible fretboard visualization tool.

![preview](./resources/preview.png)

## Installation

### NixOS

`flake.nix`:
```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fretboard = {
      url = "github:rossnomann/fretboard";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs: {
    nixosConfigurations.default = inputs.nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        # inputs.fretboard.nixosModules.default
        # or add manually:
        (
          { ... }:
          {
            config = {
              environment.systemPackages = [ inputs.fretboard.packages.${system}.default ];
            };
          }
        )
      ];
  };
}
```

## Configuration

Default path: `$XDG_CONFIG_HOME/fretboard/config.toml`
Use `FRETBOARD_CONFIG_PATH` environment variable to override the path.

Example:

```toml
# default_frets = 24
# default_tuning = "Guitar (6) Standard" # name from a [[tuning]] list item
# note_format = "sharp"  # or flat
# Theme: catppuccin-frappe, catppuccin-latte, catppuccin-macchiato, catppuccin-mocha
# theme_name = "catppuccin-mocha"
[[tuning]]
name = "Guitar (6) Standard"
data = ["E2", "A2", "D3", "G3", "B3", "E4"]
[[tuning]]
name = "Guitar (6) D Standard"
data = ["D2", "G2", "C3", "F3", "A3", "D4"]
[[tuning]]
name = "Guitar (6) Drop C#"
data = ["Db2", "Ab2", "Db3", "Gb3", "Bb3", "Eb4"]
[[tuning]]
name = "Bass (4) Standard"
frets = 24
data = ["E1", "A1", "D2", "G2"]
[[tuning]]
name = "Ukulele"
frets = 15
data = ["G4", "C4", "E4", "A4"]
```

## LICENSE

The MIT License (MIT)
