# Feature Road map

These are the features that I plan on adding to ferrishot! Not all of these may be added.

- Upload image to the web, and have access to its QR code so you can easily get it from mobile
- Snap to edges / borders of visible objects on the screen
- Read text on the screen (i.e. OCR)
- Config file to allow customizing the theme and tools
- Take screenshot N seconds from now
- Multi-monitor support (as in, you can launch the app on Monitor 2, and take a screenshot on Monitor 1. Right now you must launch it on Monitor 1 for that)

## Tools for Drawing

Tools will allow adding shapes to your screenshot.

- Numbered circle
- `p`: Pencil
- `d`: Line
- `s`: Square
- `r`: Rectangle
- `c`: Circle
- `m`: Marker
- `t`: Text

You will be able to use the scroll wheel to change thickness of the tools.

Tools should have customizability. You should be able to change the font size, weight, size.
You will be able to drag them around.

There is going to be undo (`Ctrl-z`) and redo `Ctrl-Z`

## Vim Keybindings

- `<number>X`: Set the width to this amount
- `<number>Y`: Set the height to this amount

Grand Movements

- `<`: Move selection to the far bottom left
- `>`: Move selection to the far top left
- `gg`: Move selection to the far top left
- `zt`: Move selection to the far top
- `zb`: Move selection to the far bottom
- `$`: Move selection to the far right
- `M`: Move selection to the middle of the screen
- `G`: Move selection to the far bottom right
- `0`: Move selection to the far left

Big movements

- `w`: Move selection to the top by a lot
- `W`: Extend selection up by a lot
- `C-w`: Contract selection up by a lot

- `e`: Move selection to the right by a lot
- `E`: Extend selection right by a lot
- `C-e`: Contract selection right by a lot

- `b`: Move selection to the left by a lot
- `B`: Extend selection left by a lot
- `C-b`: Contract selection left by a lot

- `n`: Move selection to the bottom by a lot
- `N`: Extend selection bottom by a lot
- `C-n`: Contract selection bottom by a lot

Little movements

- `j`: Move down by a bit
- `J`: Extend down by a bit
- `C-j`: Contract down by a bit

- `k`: Move up by a bit
- `K`: Extend up by a bit
- `C-k`: Contract up by a bit

- `l`: Move right by a bit
- `L`: Extend right by a bit
- `C-l`: Contract right by a bit

- `h`: Move left by a bit
- `H`: Extend left by a bit
- `C-h`: Contract left by a bit

All commands support numbers which will also execute them N amount of times.
