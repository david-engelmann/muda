# fish completion for muda
complete -c muda -n '__fish_use_subcommand' -a 'statement' -d 'muda statement'
complete -c muda -s h -l help -d 'Show help'
complete -c muda -n '__fish_seen_subcommand_from statement' -l 'month' -r
complete -c muda -n '__fish_seen_subcommand_from statement' -l 'rates' -r -F
complete -c muda -n '__fish_seen_subcommand_from statement' -s 'o' -r -F
complete -c muda -n '__fish_seen_subcommand_from statement' -l 'output' -r -F
