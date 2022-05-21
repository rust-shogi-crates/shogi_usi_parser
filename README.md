# Rust shogi crates: USI parser
[![Rust](https://github.com/rust-shogi-crates/shogi_usi_parser/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/rust-shogi-crates/shogi_usi_parser/actions/workflows/rust.yml?query=branch%3Amain)
[![C bindings](https://github.com/rust-shogi-crates/shogi_usi_parser/actions/workflows/c-bindings.yml/badge.svg?branch=main)](https://github.com/rust-shogi-crates/shogi_usi_parser/actions/workflows/c-bindings.yml?query=branch%3Amain)
![Rust Version](https://img.shields.io/badge/rustc-1.60+-blue.svg)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/mit-license.php)

This repository handles conversion from strings in USI format defined in [the spec](https://web.archive.org/web/20080131070731/http://www.glaurungchess.com/shogi/usi.html). It consists of two crates: a library crate that defines items (`rlib` crate), and a library crate that defines C bindings to them (`cdylib` crate).

Crates in this repository do not require the standard library (i.e., can be used by `no_std` crates) and suitable for embedded systems, as well as ordinary applications, of course.

## Available features
- `alloc`: `alloc`-related functionalities are made available. Enabled by default.
- `std`: `std`-related functionalities are made available. Implies `alloc`. Enabled by default.
