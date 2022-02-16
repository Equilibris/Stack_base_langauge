use anyhow::bail;

use crate::numeric_litteral::NumericLiteral;
use crate::tokenizer::Token;
use std::slice::Iter;
use std::str::FromStr;

#[derive(Debug)]
pub enum TopLevelNode {
    WordDeclare(String, ASTNode),       // @ident {expr}
    Typing(String, Vec<TypingASTNode>), // ?ident type
}
#[allow(dead_code)]
#[derive(Debug)]
pub enum ASTNode {
    Curly(Vec<ASTNode>),
    Square(Vec<ASTNode>),

    Ident(String),
    NumericLiteral(NumericLiteral),
    StringLiteral(String),

    Dec(String), // value ::variable | value ::CONST

    // TODO
    DecArraySized(String, NumericLiteral), // :#pointer[size]
    DecTyped(String, TypeComponent),       // value ::variable:type | value ::CONST:type
    DecArrayVariable(String),              // size :#pointer[]
    DecPointer(String),                    // value :#pointer
    PointerAssign(String),                 // value $:#{single stack entry expression}
    Assign(String),                        // value :variables
    IndexAssign(String),                   // value index $:pointer[]
    Address(String),                       // #pointerOrVariable
    ReadAddress(String),                   // $pointerOrVariable
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum TypingASTNode {
    Push(TypeComponent),
    Pop(TypeComponent),
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct TypeComponent {
    variable: Option<String>,
    type_name_components: Vec<String>,
    explicit: bool,
    poly: bool,
}

#[derive(Debug)]
pub enum FoldedStreamNode {
    Ident(String),
    NumericLiteral(String),
    CharLiteral(char),
    StringLiteral(String),

    Dollar,
    Colon,
    Octothorp,
    AtSign,
    QMark,

    Square(Vec<FoldedStreamNode>),
    Curly(Vec<FoldedStreamNode>),
}
fn fold_until_sqend(stream: &mut Iter<Token>) -> Vec<FoldedStreamNode> {
    let mut out = Vec::new();

    while let Some(x) = stream.next() {
        use FoldedStreamNode::*;
        out.push(match x {
            Token::Square(true) => Square(fold_until_sqend(stream)),
            Token::Curly(true) => Curly(fold_until_cuend(stream)),

            Token::Square(false) => break,
            Token::Curly(false) => panic!("missmatch perens"),

            Token::Comment(_) => continue,
            Token::Ident(x) => Ident(x.clone()),
            Token::NumericLiteral(x) => NumericLiteral(x.clone()),
            Token::CharLiteral(x) => CharLiteral(*x),
            Token::StringLiteral(x) => StringLiteral(x.clone()),
            Token::Dollar => Dollar,
            Token::Colon => Colon,
            Token::Octothorp => Octothorp,
            Token::AtSign => AtSign,
            Token::QMark => QMark,
        })
    }

    out
}
fn fold_until_cuend(stream: &mut Iter<Token>) -> Vec<FoldedStreamNode> {
    let mut out = Vec::new();

    while let Some(x) = stream.next() {
        use FoldedStreamNode::*;
        out.push(match x {
            Token::Square(true) => Square(fold_until_sqend(stream)),
            Token::Curly(true) => Curly(fold_until_cuend(stream)),

            Token::Square(false) => panic!("missmatch perens"),
            Token::Curly(false) => break,

            Token::Comment(_) => continue,
            Token::Ident(x) => Ident(x.clone()),
            Token::NumericLiteral(x) => NumericLiteral(x.clone()),
            Token::CharLiteral(x) => CharLiteral(x.clone()),
            Token::StringLiteral(x) => StringLiteral(x.clone()),
            Token::Dollar => Dollar,
            Token::Colon => Colon,
            Token::Octothorp => Octothorp,
            Token::AtSign => AtSign,
            Token::QMark => QMark,
        })
    }

    out
}
pub fn fold_stream(stream: Vec<Token>) -> Vec<FoldedStreamNode> {
    let mut out = Vec::new();

    let mut stream = stream.iter();

    while let Some(x) = stream.next() {
        use FoldedStreamNode::*;
        out.push(match x {
            Token::Square(true) => Square(fold_until_sqend(&mut stream)),
            Token::Curly(true) => Curly(fold_until_cuend(&mut stream)),

            Token::Square(false) => panic!("missmatch perens"),
            Token::Curly(false) => panic!("missmatch perens"),

            Token::Comment(_) => continue,
            Token::Ident(x) => Ident(x.clone()),
            Token::NumericLiteral(x) => NumericLiteral(x.clone()),
            Token::CharLiteral(x) => CharLiteral(x.clone()),
            Token::StringLiteral(x) => StringLiteral(x.clone()),
            Token::Dollar => Dollar,
            Token::Colon => Colon,
            Token::Octothorp => Octothorp,
            Token::AtSign => AtSign,
            Token::QMark => QMark,
        })
    }

    out
}

type FoldedStream = std::iter::Peekable<std::vec::IntoIter<FoldedStreamNode>>;

impl ASTNode {
    pub fn new(node: &mut FoldedStream) -> anyhow::Result<ASTNode> {
        if let Some(x) = node.next() {
            Ok(match x {
                FoldedStreamNode::Square(values) => {
                    let mut out = Vec::with_capacity(values.len());

                    let mut values: FoldedStream = values.into_iter().peekable();

                    while let Some(_) = values.peek() {
                        out.push(ASTNode::new(&mut values)?);
                    }

                    ASTNode::Square(out)
                }
                FoldedStreamNode::Curly(values) => {
                    let mut out = Vec::with_capacity(values.len());

                    let mut values: FoldedStream = values.into_iter().peekable();

                    while let Some(_) = values.peek() {
                        out.push(ASTNode::new(&mut values)?);
                    }

                    ASTNode::Curly(out)
                }

                FoldedStreamNode::Ident(x) => ASTNode::Ident(x),
                FoldedStreamNode::NumericLiteral(x) => {
                    ASTNode::NumericLiteral(NumericLiteral::from_str(x.as_str())?)
                }

                FoldedStreamNode::CharLiteral(char) => {
                    ASTNode::NumericLiteral(NumericLiteral::Uint(char as u8, 8))
                }
                FoldedStreamNode::StringLiteral(s) => ASTNode::StringLiteral(s),

                FoldedStreamNode::Dollar => todo!(),
                FoldedStreamNode::Colon => match (node.next(), node.next()) {
                    (Some(FoldedStreamNode::Colon), Some(FoldedStreamNode::Ident(name))) => {
                        ASTNode::Dec(name)
                    }
                    _ => bail!("Unexpected token or EOF"),
                },
                FoldedStreamNode::Octothorp => todo!(),
                FoldedStreamNode::AtSign => todo!(),
                FoldedStreamNode::QMark => todo!(),
            })
        } else {
            bail!("Token stream empty at EOF")
        }
    }
}

pub fn parse_type_component() -> TypeComponent {
    todo!()
}
pub fn parse_types(content: Vec<FoldedStreamNode>) -> anyhow::Result<Vec<TypingASTNode>> {
    todo!()
}

pub fn build_tree(stream: Vec<Token>) -> anyhow::Result<Vec<TopLevelNode>> {
    let mut out = Vec::new();

    let stream = fold_stream(stream);
    let mut stream: FoldedStream = stream.into_iter().peekable();

    while let Some(x) = stream.next() {
        out.push(match x {
            FoldedStreamNode::QMark => match (stream.next(), stream.next()) {
                (Some(FoldedStreamNode::Ident(ident)), Some(FoldedStreamNode::Square(content))) => {
                    TopLevelNode::Typing(ident, parse_types(content)?)
                }

                _ => bail!("Typing must be followed by ident and bracket"),
            },
            FoldedStreamNode::AtSign => match stream.next() {
                Some(FoldedStreamNode::Ident(ident)) => {
                    TopLevelNode::WordDeclare(ident, ASTNode::new(&mut stream)?)
                }

                _ => bail!("Word deceleration must be followed by ident"),
            },
            _ => panic!("Invalid top-level deceleration"),
        })
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::tokenizer;

    use super::{build_tree, fold_stream};

    #[test]
    fn exp_fold() {
        let program = "?main [] @main {\"Hello world!\\n\".}";
        let program = tokenizer(program.to_string()).unwrap();
        println!("{:?}", fold_stream(program));
    }

    #[test]
    fn exp_extract_top_level() {
        let program = "@main{1b{\"Hello world!\\n\".}if}@other main";
        let program = tokenizer(program.to_string()).unwrap();
        println!("{:?}", program);
        let program = build_tree(program).unwrap();
        println!("{:?}", program);
    }
}

// TODO: Compile time code compilation without proc macros
#[macro_export]
macro_rules! sbl_expr {
    () => {};
}

#[macro_export]
macro_rules! sbl_exprs {
    () => {};
}

// Typings
// Walk+Quack:a a must implement walk and quack
// -a           Stack pop of type a
// -Walk+Quack  Stack pop of duck
// +a           Stack push of type a
// -a!          Stack pop of explicit variable of name a
// +a!          Stack push of explicit variable of name a
// !            Never
// -*           Read an unspecific amount of untyped stack entries or the entire stack
// +a*          Write an unspecific amount of values of type a to the stack
// -a@(pattern) Pattern match

// Pre-declared words
// dup      Duplicate   -a! +a! +a!
// drop     Drop        -a
// swap     Swap        -a! -b! +b! +a!
// pick     Pick from   -a +b
// @        Jump        -Callable
// .        Put         -Writeable
// if       If          -a! -Callable +a!
// else     Else        -a! -Callable +a!
// +        Add         -Add:a -a +a
// -        Sub         -Sub:a -a +a
// *        Mul         -Mul:a -a +a
// /        Div         -Div:a -a +a
// =        Eq          -Eq:a -a +Bool
// /=       Neq         -Eq:a -a +Bool
// >        Lt          -Ord:a -a +Bool
// <        Gt          -Ord:a -a +Bool

/*
; HELLO WORLD
?main []
@main { "Hello world!" . }

; FIB
?fib [-a@(1 | 2) +a]
@fib { drop 1 }

?fib [-a +a]
@fib { dup 1 - fib swap 2 - fib + }

?main []
@main { 10i fib . }

; VECTOR ADD
?for [-a -a -a]
@for {}

@main {
    10 ::size
    $:size :#array[]
}

; IF

@main { 1 0 = { "They are equal" . } if { "They are not equal" } else }
*/
