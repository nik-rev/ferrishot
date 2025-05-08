complete -c ferrishot -s r -l region -d 'Open with a region pre-selected' -r -f
complete -c ferrishot -s a -l accept-on-select -d 'Accept on first selection' -r -f -a "copy\t'Copy image to the clipboard'
save\t'Save image to a file'
upload\t'Upload image to the internet'"
complete -c ferrishot -s d -l delay -d 'Wait this long before launch' -r -f
complete -c ferrishot -s s -l save-path -d 'Save image to path' -r -F
complete -c ferrishot -s C -l config-file -d 'Use the provided config file' -r -F
complete -c ferrishot -l log-level -d 'Choose a miniumum level at which to log' -r
complete -c ferrishot -l log-file -d 'Path to the log file' -r -F
complete -c ferrishot -l log-filter -d 'Filter for specific Rust module or crate, instead of showing logs from all crates' -r -f
complete -c ferrishot -s l -l last-region -d 'Use last region'
complete -c ferrishot -s D -l dump-default-config -d 'Write the default config to /home/e/.config/ferrishot.kdl'
complete -c ferrishot -s S -l silent -d 'Run in silent mode'
complete -c ferrishot -s j -l json -d 'Print in JSON format'
complete -c ferrishot -l log-stderr -d 'Log to standard error instead of file'
complete -c ferrishot -l debug -d 'Launch in debug mode (F12)'
complete -c ferrishot -s h -l help -d 'Print help (see more with \'--help\')'
complete -c ferrishot -s V -l version -d 'Print version'
