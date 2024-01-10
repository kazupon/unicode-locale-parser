use criterion::{black_box, criterion_group, criterion_main, Criterion};
use unicode_locale_parser::parse_locale_id;

fn locale_identifier_parser_bench(c: &mut Criterion) {
  let strings = [
    "en-US",
    "en-GB",
    "es-AR",
    "it",
    "zh-Hans-CN",
    "de-AT",
    "pl",
    "fr-FR",
    "de-AT",
    "sr-Cyrl-SR",
    "nb-NO",
    "fr-FR",
    "mk",
    "uk",
    "en-US",
    "en-GB",
    "es-AR",
    "th",
    "de",
    "zh-Cyrl-HN",
    "en-Latn-US",
  ];

  c.bench_function(
    "unicode_locale_id_parser::locale::parse_unicode_locale_id",
    |b| {
      b.iter(|| {
        for s in strings {
          let _ = parse_locale_id(black_box(s));
        }
      })
    },
  );
}

criterion_group!(benches, locale_identifier_parser_bench);
criterion_main!(benches);
