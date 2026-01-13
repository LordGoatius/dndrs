// use logos::Logos;

// #[derive(Debug, Logos)]
// pub enum TokenType {
//     #[token("(")]
//     LParen,
//     #[token(")")]
//     RParen,
//     #[token("[")]
//     LBracket,
//     #[token("]")]
//     RBracket,
//     #[token("{")]
//     LBrace,
//     #[token("}")]
//     RBrace,
//     #[token("+")]
//     Plus,
//     #[token("-")]
//     Minus,
//     #[token("/")]
//     Div,
//     #[token("*")]
//     Star,
//     #[regex(r"\d+")]
//     Number,
//     #[regex(r"\d+d\d+")]
//     Die,
//     #[token("=>")]
//     RightArrow,
//     #[regex(r"")]
//     Keyword,
//     #[regex(r#""([^"\\\x00-\x1F]|\\(["\\bnfrt/]|u[a-fA-F0-9]{4}))*""#, |lex| lex.slice().to_owned())]
//     Ident(String),
//     #[regex(r#""([^"\\\x00-\x1F]|\\(["\\bnfrt/]|u[a-fA-F0-9]{4}))*""#, |lex| lex.slice().to_owned())]
//     String(String),
//     Dot,
//     Colon,
//     Pipe,
//     Equals,
//     DoubleEquals,
// }
