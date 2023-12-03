# Simple Regex Library

A simple Rust library for building regular expressions.

## Features

- **Expressive Builder Pattern:** Easily construct regular expressions using a chainable builder pattern.
- **Concise API:** The API is designed to be concise and intuitive, making it easy to build complex regex patterns.
- **Modifier Support:** Add modifiers like case-insensitive, global search, multiline, and dot-all to your regex patterns.
- **Quantifiers:** Use quantifiers like zero or more, one or more, zero or one, exact repetitions, minimum repetitions, and range repetitions.
- **Ansi Formatting:** Includes an ANSI formatting module for adding color to terminal output.

## ANSI Module

### Colors

- `fg_black(text: String) -> String`: Formats the given text with black foreground color.
- `fg_red(text: String) -> String`: Formats the given text with red foreground color.
- `fg_green(text: String) -> String`: Formats the given text with green foreground color.
- `fg_yellow(text: String) -> String`: Formats the given text with yellow foreground color.
- `fg_blue(text: String) -> String`: Formats the given text with blue foreground color.
- `fg_purple(text: String) -> String`: Formats the given text with purple foreground color.
- `fg_cyan(text: String) -> String`: Formats the given text with cyan foreground color.
- `fg_white(text: String) -> String`: Formats the given text with white foreground color.

## RegexBuilder Struct

Builder for constructing regular expressions.

### Methods

| Function                                                             | Description                                                                                  | Example                                                                                            | Result                                     |
| -------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- | ------------------------------------------ |
| `new() -> Self`                                                      | Creates a new instance of `RegexBuilder`.                                                    | `RegexBuilder::new()`                                                                              | `RegexBuilder` instance                    |
| `literal(char) -> Self`                                              | Appends a literal character to the regex.                                                    | `.literal('a')`                                                                                    | "a"                                        |
| `dot() -> Self`                                                      | Appends a dot (.) to the regex, matching any single character.                               | `.dot()`                                                                                           | "."                                        |
| `escape(char) -> Self`                                               | Appends an escaped character to the regex.                                                   | `.escape('[')`                                                                                     | "\\["                                      |
| `start_of_line() -> Self`                                            | Appends the start of line anchor (^) to the regex.                                           | `.start_of_line()`                                                                                 | "^"                                        |
| `end_of_line() -> Self`                                              | Appends the end of line anchor ($) to the regex.                                             | `.end_of_line()`                                                                                   | "$"                                        |
| `character_class(chars: &str) -> Self`                               | Appends a character class to the regex.                                                      | `.character_class("abc")`                                                                          | "[abc]"                                    |
| `negated_character_class(chars: &str) -> Self`                       | Appends a negated character class to the regex.                                              | `.negated_character_class("abc")`                                                                  | "[^abc]"                                   |
| `range_character_class(start: char, end: char) -> Self`              | Appends a range character class to the regex.                                                | `.range_character_class('a', 'z')`                                                                 | "[a-z]"                                    |
| `digit() -> Self`                                                    | Appends a digit character class to the regex.                                                | `.digit()`                                                                                         | "\\d"                                      |
| `non_digit() -> Self`                                                | Appends a non-digit character class to the regex.                                            | `.non_digit()`                                                                                     | "\\D"                                      |
| `word_character() -> Self`                                           | Appends a word character class to the regex.                                                 | `.word_character()`                                                                                | "\\w"                                      |
| `non_word_character() -> Self`                                       | Appends a non-word character class to the regex.                                             | `.non_word_character()`                                                                            | "\\W"                                      |
| `whitespace() -> Self`                                               | Appends a whitespace character class to the regex.                                           | `.whitespace()`                                                                                    | "\\s"                                      |
| `non_whitespace() -> Self`                                           | Appends a non-whitespace character class to the regex.                                       | `.non_whitespace()`                                                                                | "\\S"                                      |
| `zero_or_more(regex: RegexBuilder) -> Self`                          | Appends a zero or more quantifier to the regex.                                              | `.zero_or_more(RegexBuilder::new().character_class("a"))`                                          | "[a]\*"                                    |
| `one_or_more(regex: RegexBuilder) -> Self`                           | Appends a one or more quantifier to the regex.                                               | `.one_or_more(RegexBuilder::new().character_class("a"))`                                           | "[a]+"                                     |
| `zero_or_one(regex: RegexBuilder) -> Self`                           | Appends a zero or one quantifier to the regex.                                               | `.zero_or_one(RegexBuilder::new().character_class("a"))`                                           | "[a]?"                                     |
| `exact_repetitions(regex: RegexBuilder, n: usize) -> Self`           | Appends an exact repetitions quantifier to the regex.                                        | `.exact_repetitions(RegexBuilder::new().digit(), 3)`                                               | "\\d{3}"                                   |
| `min_repetitions(regex: RegexBuilder, n: usize) -> Self`             | Appends a minimum repetitions quantifier to the regex.                                       | `.min_repetitions(RegexBuilder::new().digit(), 3)`                                                 | "\\d{3,}"                                  |
| `range_repetitions(regex: RegexBuilder, n: usize, m: usize) -> Self` | Appends a range repetitions quantifier to the regex.                                         | `.range_repetitions(RegexBuilder::new().digit(), 3, 5)`                                            | "\\d{3,5}"                                 |
| `group(regex: RegexBuilder) -> Self`                                 | Appends a group to the regex.                                                                | `.group(RegexBuilder::new().character_class("ab"))`                                                | "(?:[ab])"                                 |
| `backreference(group_number: usize) -> Self`                         | Appends a backreference to a capturing group in the regex.                                   | `.backreference(1)`                                                                                | "\\1"                                      |
| `word_boundary() -> Self`                                            | Appends a word boundary anchor (\b) to the regex.                                            | `.word_boundary()`                                                                                 | "\\b"                                      |
| `non_word_boundary() -> Self`                                        | Appends a non-word boundary anchor (\B) to the regex.                                        | `.non_word_boundary()`                                                                             | "\\B"                                      |
| `case_insensitive(regex: RegexBuilder) -> Self`                      | Appends a case-insensitive modifier to the regex.                                            | `.case_insensitive(RegexBuilder::new().character_class("a"))`                                      | "(?i[a])"                                  |
| `global_search(regex: RegexBuilder) -> Self`                         | Appends a global search modifier to the regex.                                               | `.global_search(RegexBuilder::new().character_class("a"))`                                         | "(?g[a])"                                  |
| `multiline(regex: RegexBuilder) -> Self`                             | Appends a multiline modifier to the regex.                                                   | `.multiline(RegexBuilder::new().character_class("a"))`                                             | "(?m[a])"                                  |
| `dot_all(regex: RegexBuilder) -> Self`                               | Appends a dot-all modifier to the regex, allowing '.' to match newline characters.           | `.dot_all(RegexBuilder::new().character_class("a"))`                                               | "(?s[a])"                                  |
| `alternative(regex1: RegexBuilder, regex2: RegexBuilder) -> Self`    | Appends an alternative (\|) to the regex, allowing either of the provided patterns to match. | `.alternative(RegexBuilder::new().character_class("a"), RegexBuilder::new().character_class("b"))` | "[a]\|[b]"                                 |
| `capturing_group(regex: RegexBuilder) -> Self`                       | Appends a capturing group to the regex.                                                      | `.capturing_group(RegexBuilder::new().character_class("a"))`                                       | "([a])"                                    |
| `to_regex()`                                                         | Converts the current `RegexBuilder` into a `Regex` object.                                   |                                                                                                    | "Returns a `Result<Regex, regex::Error>`." |
| `to_regex_or_panic()`                                                | Converts the current `RegexBuilder` into a `Regex` object or panics if an error occurs.      |                                                                                                    | "Returns a `Regex` object."                |

### Download

To use this library, add the following to your `Cargo.toml` file:

```toml
[dependencies]
simple-regex = "1.0.0"
```

### Examples

```rust
use simple_regex::RegexBuilder;

let regex = RegexBuilder::new().literal('a').build();
assert_eq!(regex, "a");
```

Please make sure to adjust the version number in the dependency based on the latest release.
