<div align="center">
  <p>
    <h1>
      <a href="https://github.com/nik-rev/ferrishot">
        <img height="64px" width="64px" src="assets/icons/Ferrishot.svg" />
      </a>
      <br />
      ferrishot
    </h1>
    <h4>Screenshot app written in Rust, inspired by <a href="https://github.com/flameshot-org/flameshot">flameshot<a />.</h4>
  </p>
</div>

## Showcase

<https://github.com/user-attachments/assets/ebbbfe85-b81e-4f26-9453-545dd1b2ce38>

## Features

Run by writing `ferrishot` on the command line.

- Select a region on the screen by left clicking and dragging
- Resize the region by dragging on any of the sides or corners and dragging
- Move the region around by dragging in the center
- `Enter` copies screenshot region to clipboard
- `Ctrl s` saves screenshot region as a file
- `F11` selects the entire monitor
- Instantly copy region to clipboard with `--instant` flag
- Holding `Shift` while resizing or dragging will resize or move the selection 10 times slower
- Size indicator allows setting an absolute width and height for the screenshot
- `Esc` exits

This project is under heavy development, and we have a lot of plans. A list of planned features can be found in [`TODO.md`](./TODO.md).

## Platform Support

- [x] Windows
- [x] MacOS
- [x] Linux (X11)
- [x] Linux (Wayland)

## Installation

### Homebrew

```sh
brew install nik-rev/tap/ferrishot
```

### PowerShell

```sh
powershell -ExecutionPolicy Bypass -c "irm https://github.com/nik-rev/ferrishot/releases/latest/download/ferrishot-installer.ps1 | iex"
```

### Shell

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/nik-rev/ferrishot/releases/latest/download/ferrishot-installer.sh | sh
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

If you use Linux, see [`CONTRIBUTING.md`](./CONTRIBUTING.md) for details on which dependencies you will need.

```sh
cargo install ferrishot
```
