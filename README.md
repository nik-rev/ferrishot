# ferrishot

An easy to use, and cross-platform screenshot app written in Rust.

https://github.com/user-attachments/assets/a7a69202-597b-4f25-816f-f84ce85c6313

## Features

Currently, this project is under heavy development. The current goal is to reach feature parity with [flameshot](https://github.com/flameshot-org/flameshot) before the 1.0 release and then go beyond.

- Select a region on the screen by left clicking and drag
- Resize region by dragging on any of the sides
- Move the region around by dragging in the center
- `Esc` closes the app
- `Ctrl c` copy region to clipboard
- `Ctrl s` save region to path

## Roadmap

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
