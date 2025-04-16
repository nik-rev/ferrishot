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
