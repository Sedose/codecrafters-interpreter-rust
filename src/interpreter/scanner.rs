use crate::interpreter::rule::comment_rule::comment_rule;
use crate::interpreter::rule::single_character_rule::single_character_rule;
use crate::interpreter::rule::two_character_rule::two_character_rule;
use crate::interpreter::token::Token;
use crate::interpreter::token_type::TokenType::Eof;
use std::iter::Peekable;
use std::str::Chars;

pub fn scan_tokens(source: &str, rule_functions: &[RuleFunction]) -> ScanResult {
    let mut tokens = Vec::new();
    let mut encountered_lexical_error = false;
    let mut character_iterator = source.chars().peekable();

    while character_iterator.peek().is_some() {
        let maybe_token = rule_functions
            .iter()
            .find_map(|rule| rule(&mut character_iterator));

        if let Some(token) = maybe_token {
            tokens.push(token);
        } else if let Some(unexpected_character) = character_iterator.next() {
            eprintln!(
                "[line 1] Error: Unexpected character: {}",
                unexpected_character
            );
            encountered_lexical_error = true;
        }
    }

    tokens.push(Token::new(Eof, ""));
    ScanResult {
        tokens,
        encountered_lexical_error,
    }
}

type RuleFunction = fn(&mut Peekable<Chars>) -> Option<Token>;

pub struct ScanResult {
    pub tokens: Vec<Token>,
    pub encountered_lexical_error: bool,
}

pub static RULE_FUNCTIONS: [RuleFunction; 3] =
    [comment_rule, two_character_rule, single_character_rule];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combined_mixed_input() {
        let (outputs, error) = lex_outputs("(,{!==}$)");
        assert!(error);
        assert_eq!(
            outputs,
            vec![
                "LEFT_PAREN ( null",
                "COMMA , null",
                "LEFT_BRACE { null",
                "BANG_EQUAL != null",
                "EQUAL = null",
                "RIGHT_BRACE } null",
                "RIGHT_PAREN ) null",
                "EOF  null",
            ]
        );
    }

    #[test]
    fn test_parentheses_scanning() {
        let (outputs, error) = lex_outputs("(()");
        assert!(!error);
        assert_eq!(
            outputs,
            vec![
                "LEFT_PAREN ( null",
                "LEFT_PAREN ( null",
                "RIGHT_PAREN ) null",
                "EOF  null",
            ]
        );
    }

    #[test]
    fn test_braces_scanning() {
        let (outputs, error) = lex_outputs("{{}}");
        assert!(!error);
        assert_eq!(
            outputs,
            vec![
                "LEFT_BRACE { null",
                "LEFT_BRACE { null",
                "RIGHT_BRACE } null",
                "RIGHT_BRACE } null",
                "EOF  null",
            ]
        );
    }

    #[test]
    fn test_single_character_tokens() {
        let (outputs, error) = lex_outputs("({*.,+*})");
        assert!(!error);
        assert_eq!(
            outputs,
            vec![
                "LEFT_PAREN ( null",
                "LEFT_BRACE { null",
                "STAR * null",
                "DOT . null",
                "COMMA , null",
                "PLUS + null",
                "STAR * null",
                "RIGHT_BRACE } null",
                "RIGHT_PAREN ) null",
                "EOF  null",
            ]
        );
    }

    #[test]
    fn test_assignment_and_equality_scanning() {
        let (outputs, error) = lex_outputs("={===}");
        assert!(!error);
        assert_eq!(
            outputs,
            vec![
                "EQUAL = null",
                "LEFT_BRACE { null",
                "EQUAL_EQUAL == null",
                "EQUAL = null",
                "RIGHT_BRACE } null",
                "EOF  null",
            ]
        );
    }

    #[test]
    fn test_negation_and_inequality_scanning() {
        let (outputs, error) = lex_outputs("!!===");
        assert!(!error);
        assert_eq!(
            outputs,
            vec![
                "BANG ! null",
                "BANG_EQUAL != null",
                "EQUAL_EQUAL == null",
                "EOF  null",
            ]
        );
    }

    #[test]
    fn test_relational_operators() {
        let (outputs, error) = lex_outputs("<<=>>=");
        assert!(!error);
        assert_eq!(
            outputs,
            vec![
                "LESS < null",
                "LESS_EQUAL <= null",
                "GREATER > null",
                "GREATER_EQUAL >= null",
                "EOF  null",
            ]
        );
    }

    #[test]
    fn test_unexpected_characters() {
        let (outputs, error) = lex_outputs("$#(");
        assert!(error);
        assert_eq!(outputs, vec!["LEFT_PAREN ( null", "EOF  null",]);
    }

    fn lex_outputs(source: &str) -> (Vec<String>, bool) {
        let ScanResult {
            tokens,
            encountered_lexical_error,
        } = scan_tokens(source, &RULE_FUNCTIONS);
        let outputs = tokens
            .into_iter()
            .map(|token| format!("{} {} null", token.token_type.as_output(), token.lexeme))
            .collect();
        (outputs, encountered_lexical_error)
    }
}
