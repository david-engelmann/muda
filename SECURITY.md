# Security Policy

## Supported Versions

Security fixes land on `main` and the latest tagged release.

## Security Boundaries

1. **Read-only Git subprocess isolation**: `muda` passes `--no-replace-objects` and `--no-optional-locks`, sets `GIT_GRAFT_FILE=/dev/null`, and removes the eight repository-redirect variables (`GIT_DIR`, `GIT_WORK_TREE`, `GIT_INDEX_FILE`, `GIT_OBJECT_DIRECTORY`, `GIT_ALTERNATE_OBJECT_DIRECTORIES`, `GIT_NAMESPACE`, `GIT_PREFIX`, `GIT_CEILING`) before invoking `git`. Run in a trusted environment when evaluating untrusted repositories, as standard `GIT_CONFIG_*` or host Git configuration still applies.
2. **Repository write-refusal guard**: `muda` resolves `-o` against the target repository's worktree and `.git` directory and refuses to write inside either (`exit 1`).

## Reporting a Vulnerability

Report security vulnerabilities privately via [GitHub Security Advisories](https://github.com/david-engelmann/muda/security/advisories/new).
