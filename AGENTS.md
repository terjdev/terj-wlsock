# Repository instructions

These instructions apply to the entire repository and take precedence over
skill instructions and supporting references whenever the rules conflict.

1. For every task that requires creating, editing, or refactoring Rust code, use the repository's [rust-guidelines skill](.agents/skills/rust-guidelines/SKILL.md) before making any Rust code changes or code review. This is mandatory without exceptions. Activate this skill only when Rust code must be changed or reviewed; do not activate it for questions, explanations or tasks that only change non-code files.
2. Change public APIs only with the maintainer's explicit permission. Preserve existing signatures, visibility, and contracts unless the maintainer has authorized the change.
3. Keep changes focused on the files and behavior required by the task. Do not modify unrelated parts of the project, even when they contain defects. Guidelines do not authorize changes outside the requested scope.
4. Report defects found outside the task scope in the final report, including their locations and effects. Leave their correction to a separately authorized task.
5. Panics are permitted only in these three cases:
   - An exceptional, critical failure cannot be handled as an error and must unambiguously terminate the program. Use a panic only as a last resort and only after explicit maintainer approval.
   - A panic in a test is the expected mechanism for reporting a test failure.
   - A `todo!` placeholder marks an unfinished part of the code during architectural prototyping.
   All other deliberate panic paths, including `unwrap`, `expect`, and assertions, are prohibited outside these cases. Return errors for malformed input and recoverable failures.
6. Do not add tests unless the maintainer requests them.
7. Write code, documentation, comments, and every repository file exclusively in English language.
8. Public wildcard imports (`pub use module::*;`) are permitted for re-exports. Private wildcard imports (`use module::*;`) are permitted only when more than 10 distinct items from that module are actually used in the importing module.
9. Git is strictly read-only. Never run Git operations that modify the working tree, index, repository metadata, configuration, refs, history, or remote repositories. This includes staging, committing, restoring, checking out, switching, resetting, cleaning, stashing, merging, rebasing, cherry-picking, fetching, pulling, pushing, creating or deleting branches or tags, and managing worktrees. Do not modify Git state indirectly through scripts, libraries, tools, or direct writes to `.git`. Use inspection commands such as `diff`, `log`, `show`, `ls-files`, and `rev-parse` only in read-only forms, and disable optional Git writes with `GIT_OPTIONAL_LOCKS=0` when running Git commands.
