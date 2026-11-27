# bash completion for muda
_muda() {
    local cur prev words cword
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    if [[ ${COMP_CWORD} -eq 1 ]]; then
        COMPREPLY=( $(compgen -W "statement -h --help" -- "${cur}") )
        return 0
    fi
    case "${cur}" in
        -*)
            COMPREPLY=( $(compgen -W "--month --rates -o --output -h --help" -- "${cur}") )
            return 0
            ;;
    esac
    COMPREPLY=( $(compgen -f -- "${cur}") )
}
complete -F _muda muda
