# Rust shogi crates: USI parser (`rlib`)
[![crate](https://img.shields.io/crates/v/shogi_usi_parser)](https://crates.io/crates/shogi_usi_parser)
[![docs](https://docs.rs/shogi_usi_parser/badge.svg)](https://docs.rs/shogi_usi_parser)
![Rust Version](https://img.shields.io/badge/rustc-1.60+-blue.svg)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/mit-license.php)

This repository handles conversion from strings in USI format defined in [the spec](https://web.archive.org/web/20080131070731/http://www.glaurungchess.com/shogi/usi.html).

# About `shogi_usi_parser` crate
This crate provides methods that convert USI strings to data. For example, it can convert following strings:

as [`PartialPosition`][PartialPosition]:
```text
lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1
```

as [`Position`][Position]:
```text
startpos moves 7g7f
```
(excerpt from <https://web.archive.org/web/20080131070731/http://www.glaurungchess.com/shogi/usi.html>)

For conversion, this crate defines the following traits and data types:

- A trait responsible for conversion from strings in USI format to `Self`.
  Implementors of this trait provides a method that reports detailed error reports, and a method that reports only whether an error occurred.
- `Error`: An error type for conversion. Some possible errors are the following:
  + The string representation is invalid (e.g. `9j` is given as a square).
  + The parsed position is invalid.

## What are out of scope?
- Failing to convert to a [`Position`][Position] when the parser encountered illegal moves
  + Other crates are responsible for legality checking

[PartialPosition]: https://docs.rs/shogi_core/latest/shogi_core/struct.PartialPosition.html
[Position]: https://docs.rs/shogi_core/latest/shogi_core/struct.Position.html
