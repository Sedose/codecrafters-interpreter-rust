use crate::interpreter::rule::comment_rule::comment_rule;
use crate::interpreter::rule::single_character_rule::single_character_rule;
use crate::interpreter::rule::two_character_rule::two_character_rule;
use crate::interpreter::token::Token;
use crate::interpreter::token_type::TokenType::Eof;
use std::iter::Peekable;
use std::str::Chars;

pub struct ErrorInfo {
    pub line_number: usize,
    pub unexpected_character: char,
}

pub fn scan_tokens(source: &str, rule_functions: &[RuleFunction]) -> ScanResult {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut character_iterator = source.chars().peekable();
    let mut line_number = 1;
    while let Some(&current_character) = character_iterator.peek() {
        if current_character.is_whitespace() {
            if current_character == '\n' {
                line_number += 1;
            }
            character_iterator.next();
            continue;
        }
        if let Some(token) = rule_functions
            .iter()
            .find_map(|rule| rule(&mut character_iterator, &mut line_number))
        {
            tokens.push(token);
            continue;
        }
        if let Some(unexpected_character) = character_iterator.next() {
            errors.push(ErrorInfo {
                line_number,
                unexpected_character,
            });
        }
    }

    tokens.push(Token::new(Eof, ""));
    ScanResult { tokens, errors }
}

type RuleFunction = fn(&mut Peekable<Chars>, &mut usize) -> Option<Token>;

pub struct ScanResult {
    pub tokens: Vec<Token>,
    pub errors: Vec<ErrorInfo>,
}

pub static RULE_FUNCTIONS: [RuleFunction; 3] =
    [comment_rule, two_character_rule, single_character_rule];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combined_mixed_input() {
        let (outputs, errors) = lex_outputs("(,{!==}$)");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line_number, 1);
        assert_eq!(errors[0].unexpected_character, '$');
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
        let (outputs, errors) = lex_outputs("(()");
        assert!(errors.is_empty());
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
        let (outputs, errors) = lex_outputs("{{}}");
        assert!(errors.is_empty());
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
        let (outputs, errors) = lex_outputs("({*.,+*})");
        assert!(errors.is_empty());
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
        let (outputs, errors) = lex_outputs("={===}");
        assert!(errors.is_empty());
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
        let (outputs, errors) = lex_outputs("!!===");
        assert!(errors.is_empty());
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
        let (outputs, errors) = lex_outputs("<<=>>=");
        assert!(errors.is_empty());
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
        let (outputs, errors) = lex_outputs("$#(");
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].line_number, 1);
        assert_eq!(errors[0].unexpected_character, '$');
        assert_eq!(errors[1].line_number, 1);
        assert_eq!(errors[1].unexpected_character, '#');
        assert_eq!(outputs, vec!["LEFT_PAREN ( null", "EOF  null",]);
    }

    #[test]
    fn test_multiline_lexical_errors() {
        let (outputs, errors) = lex_outputs("#\n(\n)\n@");
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].line_number, 1);
        assert_eq!(errors[0].unexpected_character, '#');
        assert_eq!(errors[1].line_number, 4);
        assert_eq!(errors[1].unexpected_character, '@');
        assert_eq!(
            outputs,
            vec!["LEFT_PAREN ( null", "RIGHT_PAREN ) null", "EOF  null"]
        );
    }

    fn lex_outputs(source: &str) -> (Vec<String>, Vec<ErrorInfo>) {
        let ScanResult { tokens, errors } = scan_tokens(source, &RULE_FUNCTIONS);
        let outputs = tokens
            .into_iter()
            .map(|token| format!("{} {} null", token.token_type.as_output(), token.lexeme))
            .collect();
        (outputs, errors)
    }
}
