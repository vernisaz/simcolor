# Simple Color

## What is it and why
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
- colors from [216 colors scheme](https://hexdocs.pm/color_palette/color_table.html)
- "true" (RGB)

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

- Safe Rust, easy to use, no dependencies, a complete test suite
- Respect the `CLICOLOR`/`CLICOLOR_FORCE` behavior (see [the specs](http://bixense.com/clicolors/))
- Respect the `NO_COLOR` behavior (see [the specs](https://no-color.org/))
- Do note that `CLICOLOR_FORCE` overrules `NO_COLOR`, which overrules `CLICOLOR`
- Works on Linux/FreeBSD, MacOS, and Windows (Command Prompt & Powershell)

## How to build
There are no dependencies, so use [RustBee](https://github.com/vernisaz/rust_bee) +
[common scripts](https://github.com/vernisaz/simscript) or Cargo.

If you do not to use nested colorized items, then use `partial=true` (default) at building the crate.

## How to use
The complete test suite [test.rs](test.rs) is also the documentation of usage.

## Where it is used
- Build script interpreter [RustBee](https://github.com/vernisaz/rust_bee)
- Terminal crate for web [Simple Terminal](https://github.com/vernisaz/simterminal/tree/master)

## Credits
The crate was inspired by [conqp](https://users.rust-lang.org/u/conqp)
