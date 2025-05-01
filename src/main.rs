mod interpreter;

use crate::interpreter::scanner::{scan_tokens, ScanResult, RULE_FUNCTIONS};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

fn main() {
    let input_arguments: Vec<String> = env::args().collect();
    if input_arguments.len() < 3 {
        writeln!(
            io::stderr(),
            "Usage: {} tokenize <filename>",
            input_arguments[0]
        )
        .unwrap();
        return;
    }
    let command_name = &input_arguments[1];
    let file_name = &input_arguments[2];
    if command_name != "tokenize" {
        writeln!(io::stderr(), "Unknown command: {}", command_name).unwrap();
        return;
    }

    let file_contents = fs::read_to_string(file_name).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {file_name}").unwrap();
        String::new()
    });

    let ScanResult {
        tokens,
        encountered_lexical_error,
    } = scan_tokens(&file_contents, &RULE_FUNCTIONS);

    for token in tokens {
        println!("{} {} null", token.token_type.as_output(), token.lexeme);
    }

    if encountered_lexical_error {
        process::exit(65);
    }
}
