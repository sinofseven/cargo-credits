cargo credits
=======

[![MIT License](http://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)][license]
[![crates.io](https://img.shields.io/crates/v/cargo-credits.svg)](https://crates.io/crates/cargo-credits)

[license]: https://github.com/sinofseven/cargo-credits/blob/master/LICENSE

cargo-credits creates CREDITS file from LICENSE files of dependencies

## Installation

```console
$ cargo install cargo-credits
```

## How to use

in your rust repository root

```console
$ cargo credits
```

## Description

When distributing built executable in Rust, we need to include LICENSE of the dependent
libraries into the package.  
So cargo-credits creates CREDITS file.

## Author

[sinofseven](https://github.com/sinofseven)
