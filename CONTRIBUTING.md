# Contributing

Interested in contributing? Awesome!

1. [Making changes](#making-changes)
1. [Adding a template](#adding-a-template)
1. [Editing a template](#editing-a-template)

## Making changes

1. Install [Rust](https://www.rust-lang.org/tools/install)
1. Install [wkhtmltopdf](./README.md#wkhtmltopdf)
1. Make the desired changes
1. Run `cargo test` to run tests
1. Install [Clippy](https://github.com/rust-lang/rust-clippy) and run `cargo clippy` to check for linter errors
1. Run `cargo run -- <command>` to see the effects of your changes on the CLI, e.g., `cargo run -- init --template <template>` or `cargo run -- build --watch`
1. Submit a PR!

## Adding a template

1. Follow the [instructions](#making-changes) to set up your computer
1. Create a directory for your new template in the [templates directory](./templates) containing two files: `resume.pug` and `styles.scss`
    * Alternatively, you may copy from an existing template directory, e.g., [templates/basic](./templates/basic)
1. Update [src/templates.rs](./src/templates.rs) to add your new template
    * Don't know much Rust? Follow the pattern set by an existing template (e.g., `BASIC`)
1. Follow the instructions for [editing a template](#editing-a-template) to edit your new template

## Editing a template

1. Follow the [instructions](#making-changes) to set up your computer
1. Run `resufancy init --template <template name>` to start a resume based on the template you want to edit
1. Edit `resume.pug` and `styles.scss` as desired, running `resufancy build --watch` to see the effects of your changes
1. Once ready, copy `resume.pug` and `styles.scss` back into `templates/<template>`
