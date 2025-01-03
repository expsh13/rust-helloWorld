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

impl Error for ParserError {}

/// 特殊文字のエスケープ
fn parse_escape(pos: usize, c: char) -> Result<AST, ParserError> {
    // posが現在の文字位置、cがエスケープする特殊文字
    match c {
        '\\' | '+' | '*' | '?' | '|' | '(' | ')' => Ok(AST::Char(c)),
        _ => {
            let err = ParserError::InvalidEscape(pos, c);
            Err(err)
        }
    }
}

/// parse_plus_star_question関数で利用する列挙型
enum PSQ {
    Plus,
    Star,
    Question,
}

/// +、*、?ASTに変換
///
/// 後置記法で、+、*、?の前にパターンがない場合はエラー
///
/// 例：*ab、 abc | +など
fn parse_plus_star_question(
    seq: &mut Vec<AST>,
    ast_type: PSQ,
    pos: usize,
) -> Result<(), ParserError> {
    if let Some(prev) = seq.pop() {
        let ast = match ast_type {
            PSQ::Plus => AST::Plus(Box::new(prev)),
            PSQ::Star => AST::Star(Box::new(prev)),
            PSQ::Question => AST::Question(Box::new(prev)),
        };
        seq.push(ast);
        Ok(())
    } else {
        let err = ParserError::NoPrev(pos);
        Err(err)
    }
}

/// Orで結合された複数の式をASTに変換
///
/// 例えば、abc | def | ghiは、AST::Or("abc", AST::Or("def", "ghi"))に変換される
fn fold_or(mut seq_or: Vec<AST>) -> Option<AST> {
    if seq_or.len() > 1 {
        // seq_orの要素が複数レバ、Orで式を結合
        let mut ast = seq_or.pop().unwrap();
        seq_or.reverse();
        for s in seq_or {
            ast = AST::Or(Box::new(s), Box::new(ast));
        }
        Some(ast)
    } else {
        // seq_orの要素が1つの場合、その要素を返す
        seq_or.pop()
    }
}
