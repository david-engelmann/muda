# Security Policy

## Supported Versions

| Version | Supported |
| --- | --- |
| `0.1.x` | Yes |

## Reporting a Vulnerability

Please do **not** open a public GitHub issue for suspected security vulnerabilities (such as `git` subprocess argument/environment injection, `-o` output-guard traversal, symlink clobbering, or JSON parser denial-of-service).

Report vulnerabilities privately through [GitHub Security Advisories](https://github.com/david-engelmann/muda/security/advisories/new) on `david-engelmann/muda`, including:

1. A minimal reproduction command, Git repository layout, or rates card triggering the issue.
2. Affected version or commit SHA and operating system.
3. Expected vs observed behavior and potential impact.

Reports are acknowledged within 72 hours, and coordinated fixes ship with regression tests before public disclosure.

## Security Model Notes

- `muda` scrubs ambient `GIT_*` environment variables, disables replace objects (`--no-replace-objects`) and optional locks (`--no-optional-locks`), and isolates grafts before invoking `git`.
- Atomic `-o` writes draw fresh OS CSPRNG entropy for sibling temporary file suffixes (`0600` permissions on Unix) and enforce a repository-interior guard so `muda` refuses to overwrite files inside the audited repository.
- Rate-card JSON parsing enforces container depth (`9,994`) and integer digit (`4,300`) bounds with zero external crate dependencies.
