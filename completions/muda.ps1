# PowerShell completion for muda
Register-ArgumentCompleter -Native -CommandName muda -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)
    $elements = $commandAst.CommandElements
    $subMap = @{ 'statement' = @('--month', '--rates', '-o', '--output', '-h', '--help') }
    if ($elements.Count -le 1 -or ($elements.Count -eq 2 -and $wordToComplete.Length -gt 0 -and -not $subMap.ContainsKey($elements[1].Extent.Text))) {
        $candidates = @('statement', '-h', '--help')
    } else {
        $sub = $elements[1].Extent.Text
        if ($subMap.ContainsKey($sub)) {
            $candidates = $subMap[$sub]
        } else {
            $candidates = @('-h', '--help')
        }
    }
    $candidates | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
        [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
    }
}
