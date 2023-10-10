use codespan_reporting::diagnostic::*;
use orecc_front::{chars, Codebase, TokenReader};

pub fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("Usage: cargo run --example simple_c_lexer -- sample.c");
        return;
    }

    let code = std::fs::read_to_string(&args[1]).unwrap();
    let mut codebase = Codebase::new();
    let key = codebase.add(args[1].clone(), code);

    let mut reader = TokenReader::new(codebase.get(key).unwrap().source().clone());
    while let Some(char) = reader.next_char() {
        if chars::is_digit(char) {
            // * Number
            println!("Number {}", reader.next_token(chars::is_digit, char));
        } else if chars::is_ident_start(char) {
            // * Identifier
            println!(
                "Ident {}",
                reader.next_token(chars::is_ident_continue, char)
            );
        } else if char == '"' {
            // * String
            let mut string = String::new();
            loop {
                match reader.next_char() {
                    // Escape sequences
                    Some('\\') => match reader.next_char() {
                        Some('n') => string.push('\n'),
                        Some(char) => string.push(char),
                        None => codebase.emit(
                            // Technically won't happen
                            Diagnostic::error()
                                .with_message("expected escape code")
                                .with_labels(vec![Label::primary(
                                    key,
                                    reader.cursor..reader.cursor + 1,
                                )]),
                        ),
                    },

                    // Quote
                    Some('"') => break,

                    // Char
                    Some(char) if char != '\n' => string.push(char),
                    _ => {
                        codebase.emit(
                            Diagnostic::error()
                                .with_message("unterminated string")
                                .with_labels(vec![Label::primary(
                                    key,
                                    reader.cursor..reader.cursor + 1,
                                )]),
                        );
                        break;
                    }
                }
            }
            println!("String {string:?}");
        } else if ['(', ')', '{', '}', ';'].contains(&char) {
            // * Operator
            println!("Operator {char}");
        }
    }
}
