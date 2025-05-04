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

Run by typing `ferrishot` on the command line.

### Basic usage

- Select a region on the screen by left clicking and dragging.
- Resize the region by dragging on any of the sides and dragging
- Move the region around by dragging in the center

The selection is surrounded by buttons, each with their own keybinding. Most notably:

- `Enter` copies screenshot to clipboard
- `Ctrl u` uploads the screenshot to the internet, giving you the link and QR Code - so you can easily send it to your phone!
- `Ctrl s` saves screenshot to a file. You can choose any valid extension like `.png`, `.webp`, `.jpg`

### Size Indicator

In the bottom-right corner, is a small box showing the height and width of the selection.
You can manually edit it to set a specific size.

### Keyboard Control

Ferrishot can be fully keyboard controlled, with no mouse! You can select any region on the screen in just
8 keystrokes. Pick a top-left corner by typing `t`, and pick a bottom-right corner by typing `b`:

<https://github.com/user-attachments/assets/7b013780-4213-4770-bbb4-6c9c8a383eb3>

We also have vim motions! There is a cheatsheet available by pressing `?` to view the motions:

![cheatsheet](./assets/cheatsheet.webp)

You can see all of the keybindings declared in the default config file [`default.kdl`](./default.kdl)

### Config

Ferrishot is very customizable! You have _full_ control over the UI, color scheme and keybindings.

Create the default config file `ferrishot.kdl` by doing `ferrishot --dump-default-config`.

For reference, see the [default config file (`default.kdl`)](./default.kdl) which contains comments describing each option.

### Command-line interface

You can use `ferrishot` in scripts, too

#### ferrishot

A cross-platform desktop screenshot app

**Usage:** `ferrishot [OPTIONS]`

###### **Arguments:**

- `<FILE>` — Instead of taking a screenshot of the desktop, open this image instead

###### **Options:**

- `--region <WxH+X+Y>` — Screenshot region to select

  Format: `<width>x<height>+<top-left-x>+<top-left-y>`

- `--delay <MILLISECONDS>` — Wait this long before launching
- `--save-path <PATH>` — Instead of opening a file picker to save the screenshot, save it to this path instead
- `-a`, `--accept-on-select <ACTION>` — Accept capture as soon as a selection is made

  If holding `ctrl` while you are releasing the left mouse button on the
  first selection, the behaviour is cancelled

  Possible values:

  - `copy`:
    Copy the selected region to the clipboard
  - `save`:
    Save the selected region as a file
  - `upload`:
    Upload the selected region to the internet

- `--dump-default-config` — Write the default config to ~/.config/ferrishot.kdl
- `--config-file <file.kdl>` — Specifies the config file to use

  Default value: `~/.config/ferrishot.kdl`

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

### Arch AUR

```sh
yay -S ferrishot-bin
```

### Cargo

If you use Linux, see [`CONTRIBUTING.md`](./CONTRIBUTING.md) for details on which dependencies you will need.

```sh
cargo install ferrishot
```
