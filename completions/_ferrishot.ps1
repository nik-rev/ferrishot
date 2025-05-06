
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'ferrishot' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'ferrishot'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'ferrishot' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'Open with a region pre-selected')
            [CompletionResult]::new('--region', '--region', [CompletionResultType]::ParameterName, 'Open with a region pre-selected')
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'Accept on first selection')
            [CompletionResult]::new('--accept-on-select', '--accept-on-select', [CompletionResultType]::ParameterName, 'Accept on first selection')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Wait this long before launch')
            [CompletionResult]::new('--delay', '--delay', [CompletionResultType]::ParameterName, 'Wait this long before launch')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Save image to path')
            [CompletionResult]::new('--save-path', '--save-path', [CompletionResultType]::ParameterName, 'Save image to path')
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'Use the provided config file')
            [CompletionResult]::new('--config-file', '--config-file', [CompletionResultType]::ParameterName, 'Use the provided config file')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Choose a minumum level at which to log')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Path to the log file')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'Use last region')
            [CompletionResult]::new('--last-region', '--last-region', [CompletionResultType]::ParameterName, 'Use last region')
            [CompletionResult]::new('-D', '-D ', [CompletionResultType]::ParameterName, 'Write the default config to /home/e/.config/ferrishot.kdl')
            [CompletionResult]::new('--dump-default-config', '--dump-default-config', [CompletionResultType]::ParameterName, 'Write the default config to /home/e/.config/ferrishot.kdl')
            [CompletionResult]::new('-S', '-S ', [CompletionResultType]::ParameterName, 'Run in silent mode')
            [CompletionResult]::new('--silent', '--silent', [CompletionResultType]::ParameterName, 'Run in silent mode')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'Print in JSON format')
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'Print in JSON format')
            [CompletionResult]::new('--log-stdout', '--log-stdout', [CompletionResultType]::ParameterName, 'Log to stdout instead of file')
            [CompletionResult]::new('--debug', '--debug', [CompletionResultType]::ParameterName, 'Launch ferrishot in debug mode (F12)')
            [CompletionResult]::new('--print-log-file-path', '--print-log-file-path', [CompletionResultType]::ParameterName, 'Output the path to the log file')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
