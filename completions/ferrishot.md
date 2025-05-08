# Command-Line Help for `ferrishot`

This document contains the help content for the `ferrishot` command-line program.

**Command Overview:**

* [`ferrishot`↴](#ferrishot)

## `ferrishot`

A powerful screenshot app

**Usage:** `ferrishot [OPTIONS]`

###### **Arguments:**

* `<FILE>` — Instead of taking a screenshot of the desktop, open this image instead

###### **Options:**

* `-r`, `--region <WxH+X+Y>` — Open with a region pre-selected

   Format: `<width>x<height>+<top-left-x>+<top-left-y>`

   Each value can be absolute.
   - 550 for `x` means top-left corner starts after 550px
   - 100 for `height` means it will be 100px tall

   Each can also be relative to the height (for `y` and `height`) or width (for `width` and `x`)
   - 0.2 for `width` means it region takes up 20% of the width of the image.
   - 0.5 for `y` means the top-left corner will be at the vertical center

   The format can also end with 1 or 2 percentages, which shifts the region relative to the region's size
   - If `width` is `250`, end region with `+30%` to move right by 75px or `-40%` to move left by 100px
   - Supplying 2 percentage at the end like `+30%-10%`, the 1st affects x-offset and the 2nd affects y-offset

   With the above syntax, you can create all the regions you want.
   - `100x1.0+0.5+0-50%`: Create a 100px wide, full height, horizontally centered region
   - `1.0x1.0+0+0`: Create a region that spans the full screen. You can use alias `full` for this
* `-l`, `--last-region` — Use last region
* `-a`, `--accept-on-select <ACTION>` — Accept capture and perform the action as soon as a selection is made

   If holding `ctrl` while you are releasing the left mouse button on the first selection,
   the behavior is cancelled

   It's quite useful to run ferrishot, select a region and have it instantly be copied to the
   clipboard for example. In 90% of situations you won't want to do much post-processing of
   the region and this makes that experience twice as fast. You can always opt-out with `ctrl`

   Using this option with `--region` or `--last-region` will run ferrishot in 'headless mode',
   without making a new window.

  Possible values:
  - `copy`:
    Copy image to the clipboard
  - `save`:
    Save image to a file
  - `upload`:
    Upload image to the internet

* `-d`, `--delay <MILLISECONDS>` — Wait this long before launch
* `-s`, `--save-path <PATH>` — Instead of opening a file picker to save the screenshot, save it to this path instead
* `-D`, `--dump-default-config` — Write contents of the default config to /home/e/.config/ferrishot.kdl
* `-C`, `--config-file <FILE.KDL>` — Use the provided config file

  Default value: `/home/e/.config/ferrishot.kdl`
* `-S`, `--silent` — Run in silent mode. Do not print anything
* `-j`, `--json` — Print in JSON format



