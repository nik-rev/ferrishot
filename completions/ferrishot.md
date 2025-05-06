# Command-Line Help for `ferrishot`

This document contains the help content for the `ferrishot` command-line program.

**Command Overview:**

* [`ferrishot`↴](#ferrishot)

## `ferrishot`

A cross-platform desktop screenshot app

**Usage:** `ferrishot [OPTIONS]`

###### **Arguments:**

* `<FILE>` — Instead of taking a screenshot of the desktop, open this image instead

###### **Options:**

* `-r`, `--region <WxH+X+Y>` — Open with a region pre-selected

   Format: `<width>x<height>+<top-left-x>+<top-left-y>`
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
* `-C`, `--config-file <file.kdl>` — Use the provided config file

  Default value: `/home/e/.config/ferrishot.kdl`
* `-S`, `--silent` — Run in silent mode. Do not print anything
* `-j`, `--json` — Print in JSON format



