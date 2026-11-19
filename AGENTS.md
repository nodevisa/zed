# AGENTS.md

## Writing

- **Semantic line feeds**
  In Markdown, start each sentence on a new line.
  Break long sentences at clause boundaries, such as after a comma, semicolon or colon.
  Don't wrap at a fixed width or join a paragraph onto one line.
  Line breaks don't change the rendered page, and diffs then show which sentence changed.
- **Commit messages**
  Follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/): `<type>: <description>`.
  - The type is one of `feat`, `fix`, `docs`, `refactor`, `perf`, `test`, `build`, `ci` or `chore`.
  - The description is in the imperative, starts lowercase and has no closing period.

## Checking changes

- `pixi run lint` runs every check in `prek.toml` on every file.
- `pixi run lint-install` runs them on `git commit`.
Formatters fix files in place, so rerun after a failure.
In TOML files, end inline tables with a trailing comma.
