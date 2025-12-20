# Simple Color

## What it is and why
When you implement CLI app in Rust and want to see a colorized output,
 the crate will help. 

Although there are several crates for the task, they look too complex for me.

**Simple Color** is a very minimalistic Rust crate doing the job well.

### Colors:

- black
- red
- green
- yellow
- blue
- magenta
- cyan
- white

### Styles:

- bold
- underline
- italic
- dimmed
- reversed
- blink
- hidden
- strikethrough

### Features

- Safe rust, easy to use, no dependencies, a complete test suite
- Respect the `CLICOLOR`/`CLICOLOR_FORCE` behavior (see [the specs](http://bixense.com/clicolors/))
- Respect the `NO_COLOR` behavior (see [the specs](https://no-color.org/))
- Do note that `CLICOLOR_FORCE` overrules `NO_COLOR`, which overrules `CLICOLOR`
- Works on Linux, MacOS, and Windows (Powershell)

## How to build
There are no dependencies, so use [RustBee](https://github.com/vernisaz/rust_bee) +
[common scripts](https://github.com/vernisaz/simscript) or Cargo.

If you do not to use nested colorized items, then use `partial=true` (default) at building the crate.

## How to use
The complete test suite [test.rs](test.rs) is also the documentation of usage.

## Credits
The crate was inspired by [conqp](https://users.rust-lang.org/u/conqp)
