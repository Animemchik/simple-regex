use simple_regex::RegexBuilder;

fn main() {
    let builder: RegexBuilder = RegexBuilder::new().bound_word(
        RegexBuilder::new()
            .character_class("A-Za-z0-9._%+-")
            .literal('@')
            .character_class("A-Za-z0-9.-")
            .literal('\\')
            .dot()
            .exact_repetitions(RegexBuilder::new().character_class("A-Za-z"), 2)
    );
    let regex = builder.to_regex_or_panic();
    println!("{}", regex.is_match("asasinbro3@gmail.com"));
    println!("{}", regex.is_match("ABC@gmail.az"));
}