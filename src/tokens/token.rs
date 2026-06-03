
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // Skip whitespace
   

    // Numbers
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>().ok())]
    Float(f64),

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i128>().ok())]
    Integer(i128),

    // Strings
    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        Some(s[1..s.len() - 1].to_string())
    })]
    String(String),

    // Operators
    #[token("+")]
    Add,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("=")]
    Assign,

    #[token("==")]
    Equal,

    #[token(">")]
    Higher,

    #[token("<")]
    Lesser,

    // Parentheses
    #[token("(")]
    OpenBrace,

    #[token(")")]
    CloseBrace,

    // Keywords
    #[token("femboy")]
    VarKeyword,

    #[token("newline")]
    NewLine,

    #[token("if")]
    IfKeyword,

    #[token("nyaprint")]
    Print,

    // Identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| Some(lex.slice().to_string()))]
    VarName(String),
}