//!# Simple Regex Library
//!
//!A simple Rust library for building regular expressions.
//!
//!## Features
//!
//!- **Expressive Builder Pattern:** Easily construct regular expressions using a chainable builder pattern.
//!- **Concise API:** The API is designed to be concise and intuitive, making it easy to build complex regex patterns.
//!- **Modifier Support:** Add modifiers like case-insensitive, global search, multiline, and dot-all to your regex patterns.
//!- **Quantifiers:** Use quantifiers like zero or more, one or more, zero or one, exact repetitions, minimum repetitions, and range repetitions.
//!- **Ansi Formatting:** Includes an ANSI formatting module for adding color to terminal output.
//!
//!## ANSI Module
//!
//!### Colors
//!
//!- `fg_black(text: String) -> String`: Formats the given text with black foreground color.
//!- `fg_red(text: String) -> String`: Formats the given text with red foreground color.
//!- `fg_green(text: String) -> String`: Formats the given text with green foreground color.
//!- `fg_yellow(text: String) -> String`: Formats the given text with yellow foreground color.
//!- `fg_blue(text: String) -> String`: Formats the given text with blue foreground color.
//!- `fg_purple(text: String) -> String`: Formats the given text with purple foreground color.
//!- `fg_cyan(text: String) -> String`: Formats the given text with cyan foreground color.
//!- `fg_white(text: String) -> String`: Formats the given text with white foreground color.
//!
//!## RegexBuilder Struct
//!
//!Builder for constructing regular expressions.
//!
//!### Methods
//!
//!| Function                                                             | Description                                                                                  | Example                                                                                            | Result                                     |
//!| -------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- | ------------------------------------------ |
//!| `new() -> Self`                                                      | Creates a new instance of `RegexBuilder`.                                                    | `RegexBuilder::new()`                                                                              | `RegexBuilder` instance                    |
//!| `literal(char) -> Self`                                              | Appends a literal character to the regex.                                                    | `.literal('a')`                                                                                    | "a"                                        |
//!| `dot() -> Self`                                                      | Appends a dot (.) to the regex, matching any single character.                               | `.dot()`                                                                                           | "."                                        |
//!| `escape(char) -> Self`                                               | Appends an escaped character to the regex.                                                   | `.escape('[')`                                                                                     | "\\["                                      |
//!| `start_of_line() -> Self`                                            | Appends the start of line anchor (^) to the regex.                                           | `.start_of_line()`                                                                                 | "^"                                        |
//!| `end_of_line() -> Self`                                              | Appends the end of line anchor ($) to the regex.                                             | `.end_of_line()`                                                                                   | "$"                                        |
//!| `character_class(chars: &str) -> Self`                               | Appends a character class to the regex.                                                      | `.character_class("abc")`                                                                          | "[abc]"                                    |
//!| `negated_character_class(chars: &str) -> Self`                       | Appends a negated character class to the regex.                                              | `.negated_character_class("abc")`                                                                  | "[^abc]"                                   |
//!| `range_character_class(start: char, end: char) -> Self`              | Appends a range character class to the regex.                                                | `.range_character_class('a', 'z')`                                                                 | "[a-z]"                                    |
//!| `digit() -> Self`                                                    | Appends a digit character class to the regex.                                                | `.digit()`                                                                                         | "\\d"                                      |
//!| `non_digit() -> Self`                                                | Appends a non-digit character class to the regex.                                            | `.non_digit()`                                                                                     | "\\D"                                      |
//!| `word_character() -> Self`                                           | Appends a word character class to the regex.                                                 | `.word_character()`                                                                                | "\\w"                                      |
//!| `non_word_character() -> Self`                                       | Appends a non-word character class to the regex.                                             | `.non_word_character()`                                                                            | "\\W"                                      |
//!| `whitespace() -> Self`                                               | Appends a whitespace character class to the regex.                                           | `.whitespace()`                                                                                    | "\\s"                                      |
//!| `non_whitespace() -> Self`                                           | Appends a non-whitespace character class to the regex.                                       | `.non_whitespace()`                                                                                | "\\S"                                      |
//!| `zero_or_more(regex: RegexBuilder) -> Self`                          | Appends a zero or more quantifier to the regex.                                              | `.zero_or_more(RegexBuilder::new().character_class("a"))`                                          | "[a]\*"                                    |
//!| `one_or_more(regex: RegexBuilder) -> Self`                           | Appends a one or more quantifier to the regex.                                               | `.one_or_more(RegexBuilder::new().character_class("a"))`                                           | "[a]+"                                     |
//!| `zero_or_one(regex: RegexBuilder) -> Self`                           | Appends a zero or one quantifier to the regex.                                               | `.zero_or_one(RegexBuilder::new().character_class("a"))`                                           | "[a]?"                                     |
//!| `exact_repetitions(regex: RegexBuilder, n: usize) -> Self`           | Appends an exact repetitions quantifier to the regex.                                        | `.exact_repetitions(RegexBuilder::new().digit(), 3)`                                               | "\\d{3}"                                   |
//!| `min_repetitions(regex: RegexBuilder, n: usize) -> Self`             | Appends a minimum repetitions quantifier to the regex.                                       | `.min_repetitions(RegexBuilder::new().digit(), 3)`                                                 | "\\d{3,}"                                  |
//!| `range_repetitions(regex: RegexBuilder, n: usize, m: usize) -> Self` | Appends a range repetitions quantifier to the regex.                                         | `.range_repetitions(RegexBuilder::new().digit(), 3, 5)`                                            | "\\d{3,5}"                                 |
//!| `group(regex: RegexBuilder) -> Self`                                 | Appends a group to the regex.                                                                | `.group(RegexBuilder::new().character_class("ab"))`                                                | "(?:[ab])"                                 |
//!| `backreference(group_number: usize) -> Self`                         | Appends a backreference to a capturing group in the regex.                                   | `.backreference(1)`                                                                                | "\\1"                                      |
//!| `word_boundary() -> Self`                                            | Appends a word boundary anchor (\b) to the regex.                                            | `.word_boundary()`                                                                                 | "\\b"                                      |
//!| `non_word_boundary() -> Self`                                        | Appends a non-word boundary anchor (\B) to the regex.                                        | `.non_word_boundary()`                                                                             | "\\B"                                      |
//!| `case_insensitive(regex: RegexBuilder) -> Self`                      | Appends a case-insensitive modifier to the regex.                                            | `.case_insensitive(RegexBuilder::new().character_class("a"))`                                      | "(?i[a])"                                  |
//!| `global_search(regex: RegexBuilder) -> Self`                         | Appends a global search modifier to the regex.                                               | `.global_search(RegexBuilder::new().character_class("a"))`                                         | "(?g[a])"                                  |
//!| `multiline(regex: RegexBuilder) -> Self`                             | Appends a multiline modifier to the regex.                                                   | `.multiline(RegexBuilder::new().character_class("a"))`                                             | "(?m[a])"                                  |
//!| `dot_all(regex: RegexBuilder) -> Self`                               | Appends a dot-all modifier to the regex, allowing '.' to match newline characters.           | `.dot_all(RegexBuilder::new().character_class("a"))`                                               | "(?s[a])"                                  |
//!| `alternative(regex1: RegexBuilder, regex2: RegexBuilder) -> Self`    | Appends an alternative (\|) to the regex, allowing either of the provided patterns to match. | `.alternative(RegexBuilder::new().character_class("a"), RegexBuilder::new().character_class("b"))` | "[a]\|[b]"                                 |
//!| `capturing_group(regex: RegexBuilder) -> Self`                       | Appends a capturing group to the regex.                                                      | `.capturing_group(RegexBuilder::new().character_class("a"))`                                       | "([a])"                                    |
//!| `to_regex()`                                                         | Converts the current `RegexBuilder` into a `Regex` object.                                   |                                                                                                    | "Returns a `Result<Regex, regex::Error>`." |
//!| `to_regex_or_panic()`                                                | Converts the current `RegexBuilder` into a `Regex` object or panics if an error occurs.      |                                                                                                    | "Returns a `Regex` object."                |
//!
//!### Download
//!
//!To use this library, add the following to your `Cargo.toml` file:
//!
//!```toml
//![dependencies]
//!simple-regex = "1.0.0"
//!```
//!
//!### Examples
//!
//!```rust
//!use simple_regex::RegexBuilder;
//!
//!let regex = RegexBuilder::new().literal('a').build();
//!assert_eq!(regex, "a");
//!```
//!
//!Please make sure to adjust the version number in the dependency based on the latest release.


use regex::Regex;

pub mod ansi {
    const ANSI_RESET: &str = "\x1b[0m";
    const ANSI_BLACK: &str = "\x1b[30m";
    const ANSI_RED: &str = "\x1b[31m";
    const ANSI_GREEN: &str = "\x1b[32m";
    const ANSI_YELLOW: &str = "\x1b[33m";
    const ANSI_BLUE: &str = "\x1b[34m";
    const ANSI_PURPLE: &str = "\x1b[35m";
    const ANSI_CYAN: &str = "\x1b[36m";
    const ANSI_WHITE: &str = "\x1b[37m";

    /// Formats the given text with black foreground color.
    pub fn fg_black(text: String) -> String {
        format!("{}{}{}", ANSI_BLACK, text, ANSI_RESET)
    }

    /// Formats the given text with red foreground color.
    pub fn fg_red(text: String) -> String {
        format!("{}{}{}", ANSI_RED, text, ANSI_RESET)
    }

    /// Formats the given text with green foreground color.
    pub fn fg_green(text: String) -> String {
        format!("{}{}{}", ANSI_GREEN, text, ANSI_RESET)
    }

    /// Formats the given text with yellow foreground color.
    pub fn fg_yellow(text: String) -> String {
        format!("{}{}{}", ANSI_YELLOW, text, ANSI_RESET)
    }

    /// Formats the given text with blue foreground color.
    pub fn fg_blue(text: String) -> String {
        format!("{}{}{}", ANSI_BLUE, text, ANSI_RESET)
    }

    /// Formats the given text with purple foreground color.
    pub fn fg_purple(text: String) -> String {
        format!("{}{}{}", ANSI_PURPLE, text, ANSI_RESET)
    }

    /// Formats the given text with cyan foreground color.
    pub fn fg_cyan(text: String) -> String {
        format!("{}{}{}", ANSI_CYAN, text, ANSI_RESET)
    }

    /// Formats the given text with white foreground color.
    pub fn fg_white(text: String) -> String {
        format!("{}{}{}", ANSI_WHITE, text, ANSI_RESET)
    }
}

/// Builder for constructing regular expressions.
#[derive(Clone)]
pub struct RegexBuilder {
    value: String
}

impl RegexBuilder {
    /// Creates a new instance of `RegexBuilder`.
    pub fn new() -> Self {
        Self {
            value: String::new()
        } 
    }
    /// Converts the current `RegexBuilder` into a `Regex` object.
    ///
    /// # Example
    ///
    /// ```
    /// use simple_regex::RegexBuilder;
    /// use regex::Regex;
    ///
    /// let regex_builder = RegexBuilder::new().literal('a');
    /// let regex_result = regex_builder.to_regex();
    ///
    /// match regex_result {
    ///     Ok(regex) => {
    ///         // Use the regex object
    ///         assert!(regex.is_match("a"));
    ///     }
    ///     Err(e) => {
    ///         // Handle the error if the regex is invalid
    ///         eprintln!("Error creating regex: {}", e);
    ///     }
    /// }
    /// ```
    pub fn to_regex(&self) -> Result<Regex, regex::Error> {
        Regex::new(&self.build())
    }

    /// Converts the current `RegexBuilder` into a `Regex` object or panics if an error occurs.
    ///
    /// # Panics
    ///
    /// Panics if the regex construction fails.
    ///
    /// # Example
    ///
    /// ```
    /// use simple_regex::RegexBuilder;
    /// use regex::Regex;
    ///
    /// let regex_builder = RegexBuilder::new().literal('a');
    /// let regex = regex_builder.to_regex_or_panic();
    ///
    /// // Use the regex object
    /// assert!(regex.is_match("a"));
    /// ```
    pub fn to_regex_or_panic(&self) -> Regex {
        self.to_regex().unwrap()
    }


    /// Appends a literal character to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new().literal('a').build();
    /// assert_eq!(regex, "a");
    /// ```
    pub fn literal(&mut self, char_: char) -> Self {
        self.value.push(char_);
        self.clone()
    }
    
    /// Appends a dot (.) to the regular expression, matching any single character.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new().dot().build();
    /// assert_eq!(regex, ".");
    /// ```
    pub fn dot(&mut self) -> Self {
        self.value.push('.');
        self.clone()
    }
    
    /// Appends an escaped character to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new().escape('[').build();
    /// assert_eq!(regex, "\\[");
    /// ```
    pub fn escape(&mut self, char_: char) -> Self {
        self.value.push('\\');
        self.value.push(char_);
        self.clone()
    }
    
    /// Appends the start of line anchor (^) to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new().start_of_line().build();
    /// assert_eq!(regex, "^");
    /// ```
    pub fn start_of_line(&mut self) -> Self {
        self.value.push('^');
        self.clone()
    }
    
    /// Appends the end of line anchor ($) to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new().end_of_line().build();
    /// assert_eq!(regex, "$");
    /// ```
    pub fn end_of_line(&mut self) -> Self {
        self.value.push('$');
        self.clone()
    }
    
    /// Appends a character class to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new().character_class("abc").build();
    /// assert_eq!(regex, "[abc]");
    /// ```
    pub fn character_class(&mut self, chars: &str) -> Self {
        self.value.push('[');
        self.value.push_str(chars);
        self.value.push(']');
        self.clone()
    }
    
    /// Appends a negated character class to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new().negated_character_class("abc").build();
    /// assert_eq!(regex, "[^abc]");
    /// ```
    pub fn negated_character_class(&mut self, chars: &str) -> Self {
        self.value.push('[');
        self.value.push('^');
        self.value.push_str(chars);
        self.value.push(']');
        self.clone()
    }
    
    /// Appends a range character class to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new().range_character_class('a', 'z').build();
    /// assert_eq!(regex, "[a-z]");
    /// ```
    pub fn range_character_class(&mut self, start: char, end: char) -> Self {
        self.value.push('[');
        self.value.push(start);
        self.value.push('-');
        self.value.push(end);
        self.value.push(']');
        self.clone()
    }
    
    /// Appends a digit character class to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new().digit().build();
    /// assert_eq!(regex, "\\d");
    /// ```
    pub fn digit(&mut self) -> Self {
        self.value.push_str("\\d");
        self.clone()
    }
    
    /// Appends a non-digit character class to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new().non_digit().build();
    /// assert_eq!(regex, "\\D");
    /// ```
    pub fn non_digit(&mut self) -> Self {
        self.value.push_str("\\D");
        self.clone()
    }
    
    /// Appends a word character class to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new().word_character().build();
    /// assert_eq!(regex, "\\w");
    /// ```
    pub fn word_character(&mut self) -> Self {
        self.value.push_str("\\w");
        self.clone()
    }
    
    /// Appends a non-word character class to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new().non_word_character().build();
    /// assert_eq!(regex, "\\W");
    /// ```
    pub fn non_word_character(&mut self) -> Self {
        self.value.push_str("\\W");
        self.clone()
    }
    
    /// Appends a whitespace character class to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new().whitespace().build();
    /// assert_eq!(regex, "\\s");
    /// ```
    pub fn whitespace(&mut self) -> Self {
        self.value.push_str("\\s");
        self.clone()
    }
    
    /// Appends a non-whitespace character class to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new().non_whitespace().build();
    /// assert_eq!(regex, "\\S");
    /// ```
    pub fn non_whitespace(&mut self) -> Self {
        self.value.push_str("\\S");
        self.clone()
    }
    
    /// Appends a zero or more quantifier to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new()
    ///     .zero_or_more(RegexBuilder::new().character_class("a"))
    ///     .build();
    /// assert_eq!(regex, "[a]*");
    /// ```
    pub fn zero_or_more(&mut self, regex: RegexBuilder) -> Self {
        self.value.push_str(&regex.build());
        self.value.push('*');
        self.clone()
    }
    
    /// Appends a one or more quantifier to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new()
    ///     .one_or_more(RegexBuilder::new().character_class("a"))
    ///     .build();
    /// assert_eq!(regex, "[a]+");
    /// ```
    pub fn one_or_more(&mut self, regex: RegexBuilder) -> Self {
        self.value.push_str(&regex.build());
        self.value.push('+');
        self.clone()
    }
    
    /// Appends a zero or one quantifier to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new()
    ///     .zero_or_one(RegexBuilder::new().character_class("a"))
    ///     .build();
    /// assert_eq!(regex, "[a]?");
    /// ```
    pub fn zero_or_one(&mut self, regex: RegexBuilder) -> Self {
        self.value.push_str(&regex.build());
        self.value.push('?');
        self.clone()
    }
    
    /// Appends an exact repetitions quantifier to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new()
    ///     .exact_repetitions(RegexBuilder::new().digit(), 3)
    ///     .build();
    /// assert_eq!(regex, "\\d{3}");
    /// ```
    pub fn exact_repetitions(&mut self, regex: RegexBuilder, n: usize) -> Self {
        self.value.push_str(&format!("{}{{{}}}", regex.build(), n));
        self.clone()
    }
    
    /// Appends a minimum repetitions quantifier to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new()
    ///     .min_repetitions(RegexBuilder::new().digit(), 3)
    ///     .build();
    /// assert_eq!(regex, "\\d{3,}");
    /// ```
    pub fn min_repetitions(&mut self, regex: RegexBuilder, n: usize) -> Self {
        self.value.push_str(&format!("{}{{{},}}", regex.build(), n));
        self.clone()
    }
    
    /// Appends a range repetitions quantifier to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new()
    ///     .range_repetitions(RegexBuilder::new().digit(), 3, 5)
    ///     .build();
    /// assert_eq!(regex, "\\d{3,5}");
    /// ```
    pub fn range_repetitions(&mut self, regex: RegexBuilder, n: usize, m: usize) -> Self {
        self.value.push_str(&format!("{}{{{},{}}}", regex.build(), n, m));
        self.clone()
    }
    
    /// Appends a group to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new()
    ///     .group(RegexBuilder::new().character_class("ab"))
    ///     .build();
    /// assert_eq!(regex, "(?:[ab])");
    /// ```
    pub fn group(&mut self, regex: RegexBuilder) -> Self {
        self.value.push('(');
        self.value.push_str(&regex.build());
        self.value.push(')');
        self.clone()
    }
    
    /// Appends a backreference to a capturing group in the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new()
    ///     .capturing_group(RegexBuilder::new().character_class("ab"))
    ///     .backreference(1)
    ///     .build();
    /// assert_eq!(regex, "([ab])\\1");
    /// ```
    pub fn backreference(&mut self, group_number: usize) -> Self {
        self.value.push_str(&format!("\\{}", group_number));
        self.clone()
    }
    
    /// Appends a word boundary anchor (\b) to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new().word_boundary().build();
    /// assert_eq!(regex, "\\b");
    /// ```
    pub fn word_boundary(&mut self) -> Self {
        self.value.push_str("\\b");
        self.clone()
    }
    
    /// Appends a non-word boundary anchor (\B) to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new().non_word_boundary().build();
    /// assert_eq!(regex, "\\B");
    /// ```
    pub fn non_word_boundary(&mut self) -> Self {
        self.value.push_str("\\B");
        self.clone()
    }
    
    /// Appends a case-insensitive modifier to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new()
    ///     .case_insensitive(RegexBuilder::new().character_class("a"))
    ///     .build();
    /// assert_eq!(regex, "(?i[a])");
    /// ```
    pub fn case_insensitive(&mut self, regex: RegexBuilder) -> Self {
        self.value.push_str(&format!("(?i[{}]", regex.build()));
        self.clone()
    }
    
    /// Appends a global search modifier to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new()
    ///     .global_search(RegexBuilder::new().character_class("a"))
    ///     .build();
    /// assert_eq!(regex, "(?g[a])");
    /// ```
    pub fn global_search(&mut self, regex: RegexBuilder) -> Self {
        self.value.push_str(&format!("(?g[{}]", regex.build()));
        self.clone()
    }
    
    /// Appends a multiline modifier to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new()
    ///     .multiline(RegexBuilder::new().character_class("a"))
    ///     .build();
    /// assert_eq!(regex, "(?m[a])");
    /// ```
    pub fn multiline(&mut self, regex: RegexBuilder) -> Self {
        self.value.push_str(&format!("(?m[{}]", regex.build()));
        self.clone()
    }
    
    /// Appends a dot-all modifier to the regular expression, allowing '.' to match newline characters.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new()
    ///     .dot_all(RegexBuilder::new().character_class("a"))
    ///     .build();
    /// assert_eq!(regex, "(?s[a])");
    /// ```
    pub fn dot_all(&mut self, regex: RegexBuilder) -> Self {
        self.value.push_str(&format!("(?s[{}]", regex.build()));
        self.clone()
    }
    
    /// Appends an alternative (|) to the regular expression, allowing either of the provided patterns to match.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new()
    ///     .alternative(RegexBuilder::new().character_class("a"), RegexBuilder::new().character_class("b"))
    ///     .build();
    /// assert_eq!(regex, "[a]|[b]");
    /// ```
    pub fn alternative(&mut self, regex1: RegexBuilder, regex2: RegexBuilder) -> Self {
        self.value.push_str(&format!("{}|{}", regex1.build(), regex2.build()));
        self.clone()
    }
    
    /// Appends a capturing group to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new()
    ///     .capturing_group(RegexBuilder::new().character_class("ab"))
    ///     .build();
    /// assert_eq!(regex, "([ab])");
    /// ```
    pub fn capturing_group(&mut self, regex: RegexBuilder) -> Self {
        self.value.push('(');
        self.value.push_str(&regex.build());
        self.value.push(')');
        self.clone()
    }
    
    /// Appends a non-capturing group to the regular expression.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new()
    ///     .non_capturing_group(RegexBuilder::new().character_class("ab"))
    ///     .build();
    /// assert_eq!(regex, "(?:[ab])");
    /// ```
    pub fn non_capturing_group(&mut self, regex: RegexBuilder) -> Self {
        self.value.push_str("(?:");
        self.value.push_str(&regex.build());
        self.value.push(')');
        self.clone()
    }
    
    /// Appends a word boundary anchor (\b) to the regular expression, asserting the position between a word character and a non-word character.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new()
    ///     .bound_word(RegexBuilder::new().character_class("a"))
    ///     .build();
    /// assert_eq!(regex, "\\b[a]\\b");
    /// ```
    pub fn bound_word(&mut self, regex: RegexBuilder) -> Self {
        self.value.push_str(&format!("\\b[{}]\\b", regex.build()));
        self.clone()
    }
    
    /// Appends a negative word boundary anchor (\B) to the regular expression, asserting a position where a word character is not followed by another word character.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new()
    ///     .negative_word_boundary(RegexBuilder::new().character_class("a"))
    ///     .build();
    /// assert_eq!(regex, "\\B[a]\\B");
    /// ```
    pub fn negative_word_boundary(&mut self, regex: RegexBuilder) -> Self {
        self.value.push_str(&format!("\\B[{}]\\B", regex.build()));
        self.clone()
    }
    
    /// Appends a positive lookahead assertion to the regular expression, asserting that the given pattern can match next at the current position.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new()
    ///     .positive_lookahead(RegexBuilder::new().character_class("a"))
    ///     .build();
    /// assert_eq!(regex, "(?=[a])");
    /// ```
    pub fn positive_lookahead(&mut self, regex: RegexBuilder) -> Self {
        self.value.push_str(&format!("(?={})", regex.build()));
        self.clone()
    }
    
    /// Appends a negative lookahead assertion to the regular expression, asserting that the given pattern cannot match next at the current position.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new()
    ///     .negative_lookahead(RegexBuilder::new().character_class("a"))
    ///     .build();
    /// assert_eq!(regex, "(?![a])");
    /// ```
    pub fn negative_lookahead(&mut self, regex: RegexBuilder) -> Self {
        self.value.push_str(&format!("(?!{})", regex.build()));
        self.clone()
    }
    
    /// Appends a positive lookbehind assertion to the regular expression, asserting that the given pattern can match preceding at the current position.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new()
    ///     .positive_lookbehind(RegexBuilder::new().character_class("a"))
    ///     .build();
    /// assert_eq!(regex, "(?<=a)");
    /// ```
    pub fn positive_lookbehind(&mut self, regex: RegexBuilder) -> Self {
        self.value.push_str(&format!("(?<={})", regex.build()));
        self.clone()
    }
    
    /// Appends a negative lookbehind assertion to the regular expression, asserting that the given pattern cannot match preceding at the current position.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new()
    ///     .negative_lookbehind(RegexBuilder::new().character_class("a"))
    ///     .build();
    /// assert_eq!(regex, "(?<!a)");
    /// ```
    pub fn negative_lookbehind(&mut self, regex: RegexBuilder) -> Self {
        self.value.push_str(&format!("(?<!{})", regex.build()));
        self.clone()
    }
    
    /// Builds the regular expression as a string.
    ///
    /// # Example
    ///
    /// ```
    /// use my_regex_builder::RegexBuilder;
    ///
    /// let regex = RegexBuilder::new()
    ///     .literal('a')
    ///     .character_class("bc")
    ///     .zero_or_more(RegexBuilder::new().digit())
    ///     .build();
    /// assert_eq!(regex, "a[bc]*\\d*");
    /// ```
    pub fn build(&self) -> String {
        self.value.clone()
    }
}
