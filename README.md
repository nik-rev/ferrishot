<div align="center">
  <p>
    <h1>
      <a href="https://github.com/flameshot-org/flameshot">
        <img height="64px" width="64px" src="logo.svg" />
      </a>
      <br />
      ferrishot
    </h1>
    <h4>Screenshot app written in Rust</h4>
  </p>
</div>

Currently, this project is under heavy development. The goal is to reach feature parity with [flameshot](https://github.com/flameshot-org/flameshot) before the 1.0 release and then go beyond

## Features

Run by writing `ferrishot` on the command line.

- Select a region on the screen by left clicking and drag
- Resize region by dragging on any of the sides
- Move the region around by dragging in the center
- `Esc` closes the app
- `Enter` or `Ctrl c` copy region to clipboard
- `Ctrl s` save region as an image to path
- Instantly copy region to clipboard with `--instant` flag

## Showcase

<https://github.com/user-attachments/assets/a7a69202-597b-4f25-816f-f84ce85c6313>

## Road map

- Height & width text indicator for region
- Ability to specify the selection absolutely (i.e., without mouse)
- Take screenshot in 3, 5, 10 seconds
- Draw shapes on the screen
  - Text
  - Line
  - Circle
  - Arrows
  - Square
  - Highlight
  - Numbered circles
  - Pen
- Change thickness of tools
- Pixelate region of the screen
- Undo and Redo actions for drawing
- CLI Application
- Config file
- Snap to edges / borders of visible objects on the screen

## Installation

### Homebrew

```sh
brew install nik-rev/tap/ferrishot
```

### PowerShell

```sh
powershell -ExecutionPolicy Bypass -c "irm https://github.com/nik-rev/countryfetch/releases/latest/download/ferrishot-installer.ps1 | iex"
```

### Shell

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/nik-rev/countryfetch/releases/latest/download/ferrishot-installer.sh | sh
```

### Nix

Add it to your `flake.nix`:

```nix
# add it to your inputs
inputs.ferrishot.url = "github:nik-rev/ferrishot/main";
# then use it in home-manager for example
inputs.ferrishot.packages.${pkgs.system}.default
```

### Cargo

See [`CONTRIBUTING.md`](./CONTRIBUTING.md) for details on which dependencies you will need.

```sh
cargo install ferrishot
```
