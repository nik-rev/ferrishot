module completions {

  def "nu-complete ferrishot accept_on_select" [] {
    [ "copy" "save" "upload" ]
  }

  # A powerful screenshot app
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
    --log-level: string       # Choose a miniumum level at which to log
    --log-stderr              # Log to standard error instead of file
    --log-file: path          # Path to the log file
    --log-filter: string      # Filter for specific Rust module or crate, instead of showing logs from all crates
    --debug                   # Launch in debug mode (F12)
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

}

export use completions *
