# Contributing
Contributions to the library welcome! This page describes how to get started
developing for this library.

This guide describes the tools and procedures used to ensure work is done
efficiently while maintaining consistent code quality. Don't worry if this
guide seems overwhelming; we use automated tools to check pull requests
so you do not have to worry if a step is accidentally skipped.


## Getting Started
This library is developed using the [Rust programming language](https://www.rust-lang.org/).
Install Rust then build and test the library using `cargo`:

```
cargo test
```

All the tests should pass and you are ready to start modifying code.

If you are new to Rust, [The Rust Programming Language](https://doc.rust-lang.org/stable/book/)
book is a great place to start learning about the language.


## What to Work On
Feel free to take a look at the issue tracker for tasks and bugs to tackle.
If you have an idea for a new feature file a feature request then assign it
to yourself to start work. This ensures others have clarity of new features
being added to the library.

Also, pull requests for adding, clarifying, or fixing typos in the
documentation are always welcome.


## Unit Tests
A goal of this project is to maintain excellent test coverage to ensure we
deliver a quality library. We strive for 100% branch coverage.

Please ensure any added code is covered with unit tests and the unit tests
pass before sending a pull request.

In general, unit tests should conform to the following:

* There is a single `assert` statement in the test.
* There are no branches in the test; e.g. no `if`, `while`, or other such statements.
* The names following the format: unit of work **when** state under test **should** expected behavior.

See the existing unit tests for examples.


## Lint with Clippy
We use [Clippy](https://github.com/rust-lang/rust-clippy) to catch common
mistakes in Rust code. This tool can be installed by running:

```
rustup component add clippy
```

Then run with:

```
cargo clippy
```

This will point out any problematic spots in the code. Please fix the indicated items.
Rarely, there might be false positive. In that case please allow an exception for the
specific rule at the specific location of the violation along with a comment describing
why it was necessary to suppress the rule.


## Code Formatting
The [rustfmt](https://github.com/rust-lang/rustfmt) to is used to automatically provide
consistent formatting to the code base. This allows you to focus on the high level logic
and not worry about formatting details. To instal `rustfmt` run:

```
rustup component add rustfmt
```

Then before you commit any code run:

```
cargo fmt
```


## Commits
Please try to keep commits small and containing a single logical change.

Consider these seven rules when writing a git commit message:

1. Separate subject from body with a blank line
2. Limit the subject line to 50 characters
3. Capitalize the subject line
4. Do not end the subject line with a period
5. Use the imperative mood in the subject line
6. Wrap the body at 72 characters
7. Use the body to explain what and why vs. how

The excellent [How to Write a Git Commit Message](https://chris.beams.io/posts/git-commit/)
guide provides additional details and examples for each of these items.


## Pull Requests
When the change you worked on is complete please review the following checklist
before sending a pull request:

* Tests have been added for new code
* Any new public APIs have been fully documented
* Library builds with no warnings
* All tests pass
* Clippy reports no problems
* rustfmt has been run to automatically format the code
