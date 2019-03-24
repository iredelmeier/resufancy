# Resufancy
## Supercharge your resume with the power of your favourite web development tools

[![CircleCI](https://circleci.com/gh/iredelmeier/resufancy.svg?style=svg)](https://circleci.com/gh/iredelmeier/resufancy)
[![Docs](https://docs.rs/resufancy/badge.svg)](https://docs.rs/resufancy)
[![Crates.io](https://img.shields.io/crates/v/resufancy.svg)](https://crates.io/crates/resufancy)

1. [Installing](#installing)
1. [Usage](#usage)

Tired of Word destroying your resume format when you add a word? Want to use something more powerful than Google Docs? Love (or at least *sometimes* love) CSS? Resufancy helps you format your resume with the web development tools you use day-to-day!

## Installing

### Prerequisities

#### [`wkhtmltopdf`](https://wkhtmltopdf.org/)

##### MacOS

```bash
brew cask install wkhtmltopdf
```

##### Linux

Resufancy requires the `wkhtmltox` libraries, which currently seem to be omitted by some package managers. Some [older downloads](https://wkhtmltopdf.org/downloads.html) still contain these.

Check [scripts/install-wkhtmltox.sh] for reference.

### Using `cargo`

1. Install [cargo](https://github.com/rust-lang/cargo/), e.g., by installing [Rust](https://www.rust-lang.org/):
     * See the [Rust installation guide][https://www.rust-lang.org/tools/install] for the latest instructions
1. Run `cargo install resufancy`

**Note:** To run `resufancy` directly, please make sure to add the cargo installation path (e.g., `${HOME}/.cargo/bin`) to your `$PATH`. Otherwise, you can run `cargo resufancy` to use the CLI.

### Using a pre-built binary

Grab the latest version from the [Releases](https://github.com/iredelmeier/resufancy/releases).

## Usage

1. Initialize your resume from one of the [templates](./templates):
    ```bash
    resufancy init --template <template>
    ```
1. Set a watcher for fast recompilation as you edit:
    ```bash
    resufancy build --watch
    ```
1. Edit `resume.pug` and `style.scss` to your heart's content
1. Use `resume.pdf` in your applications
1. Get your dream job! 🎉
