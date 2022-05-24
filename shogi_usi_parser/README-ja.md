# Rust shogi crates: USI parser (`rlib`)
[![crate](https://img.shields.io/crates/v/shogi_usi_parser)](https://crates.io/crates/shogi_usi_parser)
[![docs](https://docs.rs/shogi_usi_parser/badge.svg)](https://docs.rs/shogi_usi_parser)
![Rust Version](https://img.shields.io/badge/rustc-1.60+-blue.svg)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/mit-license.php)

このリポジトリは、[仕様](https://web.archive.org/web/20080131070731/http://www.glaurungchess.com/shogi/usi.html) により定められた USI フォーマットの文字列からの変換を行います。

# `shogi_usi_parser` クレイトについて
USI 文字列からデータへの変換を行います。例えば以下のような文字列を変換します。

[`PartialPosition`][PartialPosition] として:
```text
lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1
```

[`Position`][Position] として:
```text
startpos moves 7g7f
```
(https://web.archive.org/web/20080131070731/http://www.glaurungchess.com/shogi/usi.html より抜粋)

そのために以下のようなトレイト・データ型を定義します。

- `FromUsi`: USI 文字列から `Self` への変換を担当するトレイト。
  詳細なエラーを報告するメソッドとエラーが起きたかどうかだけを報告するメソッドを提供します。
- `Error`: 変換途中で発生したエラーを定義する型。以下のようなエラーがあり得ます。
  + 文字列表現が不正。(例えばマス目として `9j` などが与えられた場合。) 入力文字列の何バイト目にエラーを見つけたのか報告します。
  + 盤面が不正。

## このクレイトのスコープ外のこと
- パーサが非合法手を見つけた時の変換失敗
  + 合法手かどうかの判定は別のクレイトの責務なので

[PartialPosition]: https://docs.rs/shogi_core/latest/shogi_core/struct.PartialPosition.html
[Position]: https://docs.rs/shogi_core/latest/shogi_core/struct.Position.html
