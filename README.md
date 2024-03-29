# unicode-locale-parser

[![CI][ci-src]][ci-href]
[![crates.io][crate-io-src]][crate-io-href]

The parser for [Unicode Locale Identifiers](https://unicode.org/reports/tr35/#Unicode_locale_identifier)


## ⛏️ Conformance

All code implements of [Unicode UTS #35 Language and Locale Identifiers](https://unicode.org/reports/tr35/#Identifiers).


## 🚀 Usages

```rust
use unicode_locale_parser::parse_locale_id;

fn main() {
    // simple language
    let locale = parse_locale_id("ja-JP");
    println!("{:#?}", locale);

    // language & unicode locale extension
    let locale = parse_locale_id("de-Latn-DE-u-ca-buddhist");
    println!("{:#?}", locale);
}
```


## 🤝 API
- `parse_locale_id`: parse [`unicode_locale_id`](https://unicode.org/reports/tr35/#unicode_locale_id)
- `parse_language_id`: parse [`unicode_language_id`](https://unicode.org/reports/tr35/#unicode_language_id)
- `parse_subdivision_id`: parse [`unicode_subdivision_id`](https://unicode.org/reports/tr35/#unicode_subdivision_id)
- `parse_measure_unit`: parse [`unicode_measure_unit`](https://unicode.org/reports/tr35/#unicode_measure_unit)


## ✅ TODO
- [ ] [Locale Id Canonicalization](https://unicode.org/reports/tr35/#LocaleId_Canonicalization)
- [ ] Split some packages with Cargo workspace
- [ ] Performance
  - should optimize for string processor with using like [`TinyStr`](https://github.com/zbraniecki/tinystr)
- [ ] Add more convenient manipulation API for Locale
- [ ] Some Trait implementation
  - `Eq`, `Clone`, `Default`, `Hash`, `PartialOrd` and `Ord`


## ©️ License

[MIT](https://opensource.org/licenses/MIT)

<!-- Badges -->

[ci-src]: https://github.com/kazupon/unicode-locale-parser/actions/workflows/ci.yml/badge.svg
[ci-href]: https://github.com/kazupon/unicode-locale-parser/actions/workflows/ci.yml
[crate-io-src]: https://img.shields.io/crates/v/unicode-locale-parser.svg
[crate-io-href]: https://crates.io/crates/unicode-locale-parser