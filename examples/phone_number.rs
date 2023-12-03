use simple_regex::{RegexBuilder, ansi};

fn main() {
    // \s*(?:\+?[0-9]{10}|\(\d{3}\)|\d{3})(?:[-\s]?\d{3}[-\s]?\d{4})?\s*$
    let builder =  RegexBuilder::new()
    .start_of_line()
    .zero_or_more(RegexBuilder::new().whitespace())
    .group(
        RegexBuilder::new()
            .alternative(
                RegexBuilder::new()
                    .alternative(
                        RegexBuilder::new()
                            .zero_or_one(RegexBuilder::new().literal('+'))
                            .exact_repetitions(RegexBuilder::new().range_character_class('0', '9'), 10),
                        RegexBuilder::new()
                            .literal('(')
                            .exact_repetitions(RegexBuilder::new().digit(), 3)
                            .literal(')'),
                ),
                RegexBuilder::new().exact_repetitions(RegexBuilder::new().digit(), 3)

            )
    )
    .zero_or_one(
        RegexBuilder::new()
            .group(
                RegexBuilder::new()
                    .zero_or_one(
                        RegexBuilder::new()
                            .dash_space_character_class()
                    )
                    .exact_repetitions(RegexBuilder::new().digit(), 3)
                    .zero_or_one(
                        RegexBuilder::new()
                            .dash_space_character_class()
                    )
                    .exact_repetitions(RegexBuilder::new().digit(), 4)
            )
    )
    .zero_or_more(RegexBuilder::new().whitespace())
    .end_of_line();

    let regex_ = builder.to_regex_or_panic();
    println!("{}", builder.build());
    println!("{}", ansi::fg_green(
        format!(
            "{}", regex_.is_match("1234567890")
        )
    ));
    println!("{}", ansi::fg_green(
        format!(
            "{}", regex_.is_match("(123) 456-7890")
        )
    ));
    println!("{}", ansi::fg_green(
        format!(
            "{}", regex_.is_match("123-456-7890")
        )
    ));
    println!("{}", ansi::fg_green(
        format!(
            "{}", regex_.is_match("123 456 7890")
        )
    ));
    println!("{}", ansi::fg_green(
        format!(
            "{}", regex_.is_match("1234567890")
        )
    ));
}