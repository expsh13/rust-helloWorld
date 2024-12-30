//! 正規表現の指揮をパースし、抽象構文木（AST）に変換。
use std::{
    error::Error,
    fmt::{self, Display},
    mem::take,
};

/// 抽象構文木を表現するための型
#[derive(Debug)]
pub enum AST {
    Char(char),
    Plus(Box<AST>),
    Star(Box<AST>),
    Question(Box<AST>),
    Or(Box<AST>, Box<AST>),
    Seq(Vec<AST>),
}

/// パースエラーを表現するための型
#[derive(Debug)]
pub enum ParserError {
    InvalidEscape(usize, char), // 誤ったエスケープシーケンス
    InvalidRightParen(usize),   // 開き括弧なし
    NoPrev(usize),              // + 、　｜　、*、　?の前に何もない
    NoRightParen,               // 閉じ括弧なし
    Empty,                      //空のパターン
}
/// パースエラーを表示するために、Displayトレイトを実装
impl Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::InvalidEscape(pos, ch) => {
                write!(f, "Invalid escape sequence at position {}: '{}'", pos, ch)
            }
            ParserError::InvalidRightParen(pos) => {
                write!(f, "Invalid right parenthesis at position {}", pos)
            }
            ParserError::NoPrev(pos) => write!(f, "No previous character at position {}", pos),
            ParserError::NoRightParen => write!(f, "No right parenthesis"),
            ParserError::Empty => write!(f, "Empty pattern"),
        }
    }
}
