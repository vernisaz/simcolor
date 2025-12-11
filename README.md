# Simple Color

## What it is and why
When you implement CLI app in Rust and want to see a colorized output,
 the crate will help. 

You will find two absolutely beautiful crates for the task [Colored](https://github.com/colored-rs/colored)
and [OWO-Colors](https://github.com/owo-colors/owo-colors). So, this crate was inspired by the giants.

As usually the **Simple Color** is a very minimalistic Rust crate.

## Features

- Safe rust, easy to use, minimal dependencies, complete test suite
- Respect the `CLICOLOR`/`CLICOLOR_FORCE` behavior (see [the specs](http://bixense.com/clicolors/))
- Respect the `NO_COLOR` behavior (see [the specs](https://no-color.org/))
- Do note that `CLICOLOR_FORCE` overrules `NO_COLOR`, which overrules `CLICOLOR`
- Works on Linux, MacOS, and Windows (Powershell)

## How build
There are no dependencies, so use [RustBee](https://github.com/vernisaz/rust_bee) or Cargo.

## How to use
[test.rs](test.rs)

## Credits
The crate was inspired by [conqp](https://users.rust-lang.org/u/conqp)
