# Contributing to Kiln

Thanks for helping make Kiln better! 

Issues and pull requests are welcome in Kiln.
However, issues and pull requests **must follow the templates below**. Ones that ignore them may be closed without review.

Kiln is still an early alpha, so things change quickly. If you're planning something big, open an issue first so we can talk it through before you spend time on it.

## Table of Contents

- [Getting Started](#getting-started)
- [Issue Template](#issue-template)
- [Pull Request Template](#pull-request-template)
- [Code Style](#code-style)
- [Error Codes](#error-codes)
- [Commit Messages](#commit-messages)

## Getting Started

1. Fork the repository and clone your fork.
2. Build the project:

   ```sh
   cargo build
   ```

3. Create a branch for your change:

   ```sh
   git checkout -b fix/short-description
   ```

Project layout (`Kiln/src`): `token`, `lexer`, `ast`, `parser`, `interpreter` and `main.rs`.

## Issue Template

Before opening an issue, search existing issues to make sure it isn't already reported.

Copy the right template into your issue and fill in **every** section.

### Bug Report

```markdown
**Summary**
One or two sentences describing the bug.

**Kiln version**
e.g. V0.0.1 alpha (see the banner printed by `main.rs`)

**Environment**
OS / device, Rust version (`rustc --version`)

**Code that triggers it**
(a minimal `.kiln` example)

**Steps to reproduce**
1.
2.
3.

**Expected behavior**
What you thought would happen.

**Actual behavior**
What happened instead. Include the full output and any error code (e.g. `[E001]`).

**Additional context**
Anything else that might help.
```

### Feature Request

```markdown
**Problem**
What are you trying to do that Kiln can't do (or does badly) today?

**Proposed solution**
Describe the syntax or behavior you'd like, with a short example.

**Alternatives considered**
Other approaches you thought about.

**Additional context**
Links, related issues, prior art in other languages.
```

### Question or Discussion

Use a clear title, say what you're trying to achieve, and include what you've already tried.

## Pull Request Template

Keep pull requests small and focused: multiple changes per PR are allowed.

```markdown
**What does this PR do?**
Short description of the change.

**Related issue**
Closes #

**Type of change**
- [ ] Bug fix
- [ ] New feature
- [ ] Refactor (no behavior change)
- [ ] Documentation
- [ ] Other:

**How was it tested?**
Describe how you checked it works, and include a `.kiln` example if relevant.

**Checklist**
- [ ] `cargo build` succeeds with no new warnings
- [ ] `cargo fmt` has been run
- [ ] I ran a `.kiln` example that exercises my change
- [ ] I updated the docs / README if behavior changed
- [ ] New errors use a new, unused error code
```

## Code Style

- Run `cargo fmt` before committing.
- Prefer clear names over clever code, in line with Kiln's goal: *readable, reliable and reasonable code*.
- Add comments for anything non-obvious, especially in the lexer, parser and interpreter.

## Error Codes

Kiln errors use the format `[E<number>]`. If your change adds a new error:

- Pick the next unused number in the relevant range (lexer, parser or runtime).
- Include a clear message, and a line and column where the error type supports it.
- Include a short `.kiln` example that triggers it.

## Commit Messages

Write short, imperative messages that say what changed:

```
Add unary minus to the parser
Fix unterminated string error code
```

## Code of Conduct

Be kind and respectful. Disagree with ideas, not people. Harassment of any kind isn't tolerated.

---

Thanks again for contributing! 🔥
