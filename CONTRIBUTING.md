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

### Test Coverage with Tarpaulin
If you are developing on Linux with a x86_64 processor you can measure the
test coverage using [tarpaulin](https://crates.io/crates/cargo-tarpaulin).

Install tarpaulin with the following:
```
cargo install cargo-tarpaulin
```

Then run with:
```
cargo tarpaulin --out Html
```

Open the generated `tarpaulin-report.html` file in your browser to view test
coverage data.

This tool is also automatically run as part of the pull request process.


## Lint with Clippy
We use [Clippy](https://github.com/rust-lang/rust-clippy) to catch common
mistakes in Rust code. This tool can be installed by running:

```
rustup component add clippy
```

Then run with:

```
cargo clippy && cargo clippy --examples
```

This will point out any problematic spots in the code. Please fix the indicated items.
In rare situations, there might be false positive. In that case please allow an exception 
to the specific rule at the specific location of the violation along with a comment 
describing why it was necessary to suppress the rule.


## Code Formatting
The [rustfmt](https://github.com/rust-lang/rustfmt) to is used to automatically provide
consistent formatting to the code base. This allows you to focus on the high level logic
and not worry about formatting details. To install `rustfmt` run:

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
* Library builds without warnings
* All tests pass
* Clippy reports no problems
* rustfmt has been run to automatically format the code


## Creating Releases
Maintainers, please follow this checklist to create a release and publish it to
[crates.io](https://crates.io/).

1. Ensure all changes are merged to the `main` branch. Verify the README,
   library documentation, and examples are updated and correct.
2. Ensure the `CHANGELOG.md` describes all notable changes from the last release.
3. Update the version number:
   * `version` value in `Cargo.toml`
   * **Usage** section in `README.md`
   * Set the version number and release date in `CHANGELOG.md`
   * Edit the `html_root_url` attribute in `lib.rs`
4. Ensure checks pass by running the following:
   ```
   cargo test
   cargo test --release -- --ignored
   cargo clippy -- -D warnings
   cargo fmt --all -- --check
   ```
5. Commit the changes and tag the commit with the version number.
6. Preform a dry run of publishing the crate with `cargo publish --dry-run`.
7. Publish the crate with `cargo publish`.
8. Push the git commit to GitHub and tags with `git push` and `git push --tags`.
