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
/// Errorトレイトはトレイト制約としてDisplay、Debugトレイトを持つ
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

/// 特殊文字のエスケープ（特殊文字を単なる文字として表現）
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

/// parse_plus_star_question関数で利用する列挙型（限量子）
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
    seq: &mut Vec<AST>, // 限量子より前に出現する正規表現の並び
    ast_type: PSQ,      // 限量子の種類
    pos: usize,         // 限量子の位置
) -> Result<(), ParserError> {
    // .pop()メソッドは、Vecの最後の要素を取り出し、それをOption<T>として返す。
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
        // seq_orの要素が複数あれば、Orで式を結合
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

/// 正規表現をパースし、ASTに変換
pub fn parse(expr: &str) -> Result<AST, ParserError> {
    // 内部状態を表現するための型
    // Char:文字列処理中
    // Escape:エスケープ処理中
    enum ParseState {
        Char,
        Escape,
    }

    let mut seq = Vec::new(); // 現在のSeqコンテキスト
    let mut seq_or = Vec::new(); // 現在のOrコンテキスト
    let mut stack = Vec::new(); //コンテキストのスタック
    let mut state = ParseState::Char; // 現在の状態

    for (i, c) in expr.chars().emumerate() {
        match &state {
            ParseState::Char => match c {
                '+' => {
                    parse_plus_star_question(&mut seq, PSQ::Plus, i)?;
                }
                '*' => {
                    parse_plus_star_question(&mut seq, PSQ::Star, i)?;
                }
                '?' => {
                    parse_plus_star_question(&mut seq, PSQ::Question, i)?;
                }
                '(' => {
                    let prev = take(&mut seq);
                    let prev_or = take(&mut seq_or);
                    stack.push((prev, prev_or));
                }
                ')' => {
                    if let Some((mut prev, prev_or)) = stack.pop() {
                        if !seq.is_empty() {
                            seq_or.push(AST::Seq(seq));
                        }
                        if let Some(ast) = fold_or(seq_or) {
                            prev.push(ast);
                        }
                        seq = prev;
                        seq_or = prev_or;
                    } else {
                        let err = Box::new(ParserError::InvalidRightParen(i));
                        return Err(err);
                    }
                }
                '|' => {
                    if seq.is_empty() {
                        return Err(Box::new(ParserError::NoPrev(i)));
                    } else {
                        let prev = take(&mut seq);
                        seq_or.push(AST::Seq(prev));
                    }
                }
                '\\' => state = ParseState::Escape,
                _ => seq.push(AST::Char(c)),
            },
            ParseState::Escape => {
                seq.push(parse_escape(i, c)?);
                state = ParseState::Char;
            }
        }
    }

    if !stack.is_empty() {
        return Err(Box::new(ParserError::NoRightParen));
    }

    if !seq.is_empty() {
        seq_or.push(AST::Seq(seq));
    }

    if let Some(ast) = fold_or(seq_or) {
        Ok(ast)
    } else {
        Err(Box::new(ParserError::Empty))
    }
}
