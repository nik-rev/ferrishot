
use builtin;
use str;

set edit:completion:arg-completer[ferrishot] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'ferrishot'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'ferrishot'= {
            cand -r 'Open with a region pre-selected'
            cand --region 'Open with a region pre-selected'
            cand -a 'Accept on first selection'
            cand --accept-on-select 'Accept on first selection'
            cand -d 'Wait this long before launch'
            cand --delay 'Wait this long before launch'
            cand -s 'Save image to path'
            cand --save-path 'Save image to path'
            cand -C 'Use the provided config file'
            cand --config-file 'Use the provided config file'
            cand --log-level 'Choose a minumum level at which to log'
            cand --log-file 'Path to the log file'
            cand -l 'Use last region'
            cand --last-region 'Use last region'
            cand -D 'Write the default config to /home/e/.config/ferrishot.kdl'
            cand --dump-default-config 'Write the default config to /home/e/.config/ferrishot.kdl'
            cand -S 'Run in silent mode'
            cand --silent 'Run in silent mode'
            cand -j 'Print in JSON format'
            cand --json 'Print in JSON format'
            cand --log-stdout 'Log to stdout instead of file'
            cand --debug 'Launch ferrishot in debug mode (F12)'
            cand --print-log-file-path 'Output the path to the log file'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
    ]
    $completions[$command]
}
