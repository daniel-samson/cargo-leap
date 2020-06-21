Cargo Leap
====

[![crates.io](https://img.shields.io/crates/v/cargo-leap.svg)](https://crates.io/crates/cargo-leap)
[![Rust](https://github.com/daniel-samson/cargo-leap/workflows/Rust/badge.svg?branch=master)](https://github.com/daniel-samson/cargo-leap/actions)
[![Docs](https://docs.rs/cargo-leap/badge.svg?version=0.1.0)](https://docs.rs/leap/0.1.0/cargo-leap/)
[![codecov](https://codecov.io/gh/daniel-samson/cargo-leap/branch/master/graph/badge.svg)](https://codecov.io/gh/daniel-samson/cargo-leap)
[![book](https://img.shields.io/badge/Book-Master-blue)](https://leap.rs/book/version/master/introduction/)

Leap pass the setup for your project and craft applications quickly with ease. Cargo leap is a cargo subcommand for generating new projects from crates which contain a leap directory.

Project Goals:
- Provide a means to create project template
- Download any crate from crates.io and read leap directory
- Provide usuful features as they are needed.


### Getting Leap

You can user cargo install to get leap:
```bash
# cargo install cargo-leap
```
Please ensure that you have added your cargo binary directory to you PATH environement variable.


### Usage

Run cargo leap:
```bash
# cargo leap template-crate project-name
```

- **template-create** is the name of the [crate](https://crates.io/) you wish to use as a template.
- **project-name** is the name you wish to call your project

Please check your project and update files such as cargo.toml to match the details of your project.

### Example

```bash
# cargo leap leap portal
```


### Contributions

Any contributions are welcome, please read the [contributing guidlines](CONTRIBUTING.md), before making any changes.

