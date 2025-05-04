# v0.3.0 - 4 May 2025

- Added support for touch inputs
- Ferrishot can now be configured! The config format uses [KDL](https://kdl.dev/). Here is an excerpt from the default config:

```kdl
// Show the size indicator
size-indicator #true
// Show icons around the selection
selection-icons #true

keys {
  // Leave the app
  exit key=<esc>

  // Copies selected region to clipboard, exiting
  copy-to-clipboard mod=ctrl key=c
  copy-to-clipboard key=<enter>
}

theme {
  // color of the frame around the selection
  selection-frame 0xab_61_37

  // background color of the region that is not selected
  non-selected-region 0x00_00_00 opacity=0.5
}
```

- Image upload. Press `ctrl + u` to upload your screenshot to the internet. You'll get a preview of the image, link to it and a QR Code.
- Vim keybindings! Ferrishot is fully controllable via keyboard, if you want to do that
  - `h | j | k | l` to move by 1 pixel the given side
  - `H | J | K | L` to extend by 1 pixel the given side
  - `ctrl + h | j | k | l` shrinks by 1 pixel the given side
  - Use `alt` in combination with any of the above keys to do the movement of `125px` instead of `1px`
  - There is also `gg` to move to top-left corner, `G` for bottom-right corner and more. You can view a cheatsheet in-app by pressing `?`.
  - All the keybindings can be overridden with a custom config. You are also able to define custom ones, if you want it to be `50px` instead of `125px` for all of them simply do `ferrishot --dump-default-config` and then edit the values `125 -> 50`.
- Super-select mode, press `t` and:
  - The screen will be divided into 25 regions each assigned a letter. Press any of the letters, then:
  - The chosen region will be further divided into 25 more regions. This will repeat yet again for a 3rd time.
  - Allows you to place your cursor into 1 of 15,625 positions on the screen within 4 keystrokes. This will select the top-left corner
  - Repeat this with the bottom-right corner by using `b`. Now you have selected your screenshot without using a mouse, in just 8 keystrokes!
- More powerful command line interface

  - `--delay` wait some time before taking a screenshot
  - `--region` open program with a custom region
  - `--save-path` choose a file to save the image to, instead of opening file picker (when using `ctrl + s`)
  - `--accept-on-select` instantly save, upload or copy image to clipboard as soon as you create the first selection.

    This can be cancelled with `ctrl`. It's handy since often you want to instantly copy the selection to clipboard and not do anything fancy.

    So you can therefore set a shell alias like `alias ferrishot=ferrishot --accept-on-select=copy` and that may be satisfying to you 90% of the time.

    In some cases you'd like to save the image, so just hold ctrl to disable this behaviour when releasing the left-mouse button.

    Using this option with `--region` will run ferrishot in "headless mode", without opening a window (as the region created with `--region` _will_ be the first region)

# v0.2.0 - 16 April 2025

- Right-click will snap the closest corner to the current cursor position
- There are now buttons available for these actions:
  - `Enter`: Copy to Clipboard
  - `F11`: Select entire monitor
  - `Ctrl + S`: Save Screenshot
  - `Esc`: Exit
- Added an indicator for the width and height of selection in the bottom right corner. This indicator can be edited to set the selection to a concrete size!
- Holding `Shift` while resizing or moving the selection will now do it 10 times slower, to allow being very accurate.
- Improved security on linux as per review from a reddit user: <https://www.reddit.com/r/rust/comments/1jxwd86/comment/mmvfzhy/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button>

# v0.1.0 - 12 April 2025

The initial release comes with the following features:

- Select a region on the screen by left clicking and drag
- Resize region by dragging on any of the sides
- Move the region around by dragging in the center
- `Esc` closes the app
- `Enter` or `Ctrl c` copy region to clipboard
- `Ctrl s` save region as an image to path
- Instantly copy region to clipboard with `--instant` flag
