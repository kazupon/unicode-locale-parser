use criterion::{black_box, criterion_group, criterion_main, Criterion};
use unicode_locale_parser::parse_language_id;

fn language_identifier_parser_bench(c: &mut Criterion) {
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
        "unicode_locale_id_parser::lang:::parse_unicode_language_id",
        |b| {
            b.iter(|| {
                for s in strings {
                    let _ = parse_language_id(black_box(s));
                }
            })
        },
    );
}

criterion_group!(benches, language_identifier_parser_bench);
criterion_main!(benches);
