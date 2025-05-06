module completions {

  def "nu-complete ferrishot accept_on_select" [] {
    [ "copy" "save" "upload" ]
  }

  # A cross-platform desktop screenshot app
  export extern ferrishot [
    file?: path               # Instead of taking a screenshot of the desktop, open this image instead
    --region(-r): string      # Open with a region pre-selected
    --last-region(-l)         # Use last region
    --accept-on-select(-a): string@"nu-complete ferrishot accept_on_select" # Accept on first selection
    --delay(-d): string       # Wait this long before launch
    --save-path(-s): path     # Save image to path
    --dump-default-config(-D) # Write the default config to /home/e/.config/ferrishot.kdl
    --config-file(-C): path   # Use the provided config file
    --silent(-S)              # Run in silent mode
    --json(-j)                # Print in JSON format
    --log-level: string       # Choose a minumum level at which to log
    --log-stdout              # Log to stdout instead of file
    --log-file: path          # Path to the log file
    --debug                   # Launch ferrishot in debug mode (F12)
    --print-log-file-path     # Output the path to the log file
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

}

export use completions *
