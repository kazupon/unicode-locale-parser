use unicode_locale_parser::parse_locale_id;

fn main() {
  // simple language
  let locale = parse_locale_id("ja-JP");
  println!("{:#?}", locale);

  // language & unicode locale extension
  let locale = parse_locale_id("de-Latn-DE-u-ca-buddhist");
  println!("{:#?}", locale);
}
