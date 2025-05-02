Contribution guide
========

First of all, thank you for your interest in contributing to basm-rs!

We welcome all contributions, whether it be new feature additions, bugfixes, refactorings, or anything we haven't thought of.

Here are a few things to check before filing a PR:

- Do NOT edit `basm/src/solution.rs`, since CI expects this file to be a solution to the problem of adding two integers (i.e., Baekjoon Online Judge Problem 1000). Hence, CI will fail if this expectation breaks.
- Please run `clippy` and `rustfmt` (both with latest nightly) and ensure they have no errors before making a PR. This can be done by enabling CI on your fork, or testing them locally with the following commands:
  * `clippy`: `cargo clippy --all-targets -- -D warnings -A clippy::missing_safety_doc`
  * `rustfmt`:  `cargo fmt --check`

We look forward to your valuable contributions!