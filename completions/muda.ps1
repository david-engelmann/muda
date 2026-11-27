# PowerShell completion for muda
Register-ArgumentCompleter -Native -CommandName muda -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)
    $candidates = @('statement', '--month', '--rates', '-o', '--output', '-h', '--help')
    $candidates | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
        [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
    }
}
