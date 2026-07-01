# Security Policy

## Supported versions

`numancy` follows semantic versioning. Security fixes are released against the
latest published version on [crates.io](https://crates.io/crates/numancy).

## Reporting a vulnerability

Please report suspected vulnerabilities privately rather than opening a public
issue. Use GitHub's
[private vulnerability reporting](https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing-information-about-vulnerabilities/privately-reporting-a-security-vulnerability)
on this repository ("Security" tab → "Report a vulnerability").

Please include:

- a description of the issue and its impact;
- steps to reproduce (a minimal code sample if possible);
- the `numancy` version affected.

We will acknowledge your report as soon as possible and keep you updated on the
fix and disclosure timeline.

## Scope

`numancy` is a pure, dependency-free computation library: it performs
arithmetic, reads no files, opens no network connections, and uses no `unsafe`
code (`#![forbid(unsafe_code)]`). Reports about memory safety, panics on
untrusted input, or incorrect results are all in scope.
