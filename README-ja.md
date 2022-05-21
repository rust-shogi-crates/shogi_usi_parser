# Rust shogi crates: USI parser
[![Rust](https://github.com/rust-shogi-crates/shogi_usi_parser/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/rust-shogi-crates/shogi_usi_parser/actions/workflows/rust.yml?query=branch%3Amain)
[![C bindings](https://github.com/rust-shogi-crates/shogi_usi_parser/actions/workflows/c-bindings.yml/badge.svg?branch=main)](https://github.com/rust-shogi-crates/shogi_usi_parser/actions/workflows/c-bindings.yml?query=branch%3Amain)
![Rust Version](https://img.shields.io/badge/rustc-1.60+-blue.svg)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/mit-license.php)

このリポジトリは、[仕様](https://web.archive.org/web/20080131070731/http://www.glaurungchess.com/shogi/usi.html) により定められた USI フォーマットの文字列からの変換を行います。このリポジトリは 2 個のクレイトからなります: アイテムを定義するライブラリクレイト (`rlib` クレイト) と、それらのアイテムへの C バインディングを定義するライブラリクレイト (`cdylib` クレイト) です。
このリポジトリに含まれているクレイトは標準ライブラリを要求しません。つまり、`no_std` クレイトから使用可能です。組み込みシステムに適しており、もちろん普通のアプリケーションにも適しています。

## 利用可能なフィーチャ
- `alloc`: `alloc` 関連の機能が利用可能になります。デフォルトで有効化されています。
- `std`: `std` 関連の機能が利用可能になります。有効化すると `alloc` も有効化されます。デフォルトで有効化されています。
