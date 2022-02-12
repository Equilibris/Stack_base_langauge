use anyhow::bail;

use crate::numeric_litteral::NumericLiteral;
use crate::tokenizer::Token;
use std::slice::Iter;
use std::str::FromStr;

pub enum TopLevelNode {
    Comment(String), // ~

    WordDeclare(String, ASTNode),       // @ident {expr}
    Typing(String, Vec<TypingASTNode>), // ?ident type
}

pub enum ASTNode {
    Curly(Vec<ASTNode>),
    Square(Vec<ASTNode>),

    Comment(String),
    Ident(String),
    NumericLiteral(NumericLiteral),
    CharLiteral(char),
    StringLiteral(String),

    DecTyped(String, Vec<TypingASTNode>), // value ::variable:type | value ::CONST:type
    Dec(String),                          // value ::variable | value ::CONST
    DecPointerSized(String, Vec<ASTNode>), // :#pointer[size]
    DecPointerVariable(String),           // size :#pointer[]
    DecPointer(String),                   // value :#pointer
    Assign(String),                       // value $:variables
    IndexAssign(String, Vec<ASTNode>),    // value $:pointer[index]
    Address(String),                      // #pointerOrVariable
    ReadAddress(String),                  // $pointerOrVariable

    // TODO
    PointerAssign(String), // value $:#{single stack entry expression}
}

pub enum TypingASTNode {
    Push(Vec<TypeComponent>),
    Pop(Vec<TypeComponent>),
}
pub struct TypeComponent {
    variable: Option<String>,
    type_name_components: Vec<String>,
    explicit: bool,
    poly: bool,
}

#[derive(Debug)]
pub enum FoldedStreamNode {
    Comment(String),
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
        out.push(match x {
            Token::Square(true) => FoldedStreamNode::Square(fold_until_sqend(stream)),
            Token::Curly(true) => FoldedStreamNode::Curly(fold_until_cuend(stream)),

            Token::Square(false) => break,
            Token::Curly(false) => panic!("missmatch perens"),

            Token::Comment(x) => FoldedStreamNode::Comment(x.clone()),
            Token::Ident(x) => FoldedStreamNode::Ident(x.clone()),
            Token::NumericLiteral(x) => FoldedStreamNode::NumericLiteral(x.clone()),
            Token::CharLiteral(x) => FoldedStreamNode::CharLiteral(x.clone()),
            Token::StringLiteral(x) => FoldedStreamNode::StringLiteral(x.clone()),
            Token::Dollar => FoldedStreamNode::Dollar,
            Token::Colon => FoldedStreamNode::Colon,
            Token::Octothorp => FoldedStreamNode::Octothorp,
            Token::AtSign => FoldedStreamNode::AtSign,
            Token::QMark => FoldedStreamNode::QMark,
        })
    }

    out
}
fn fold_until_cuend(stream: &mut Iter<Token>) -> Vec<FoldedStreamNode> {
    let mut out = Vec::new();

    while let Some(x) = stream.next() {
        out.push(match x {
            Token::Square(true) => FoldedStreamNode::Square(fold_until_sqend(stream)),
            Token::Curly(true) => FoldedStreamNode::Curly(fold_until_cuend(stream)),

            Token::Square(false) => panic!("missmatch perens"),
            Token::Curly(false) => break,

            Token::Comment(x) => FoldedStreamNode::Comment(x.clone()),
            Token::Ident(x) => FoldedStreamNode::Ident(x.clone()),
            Token::NumericLiteral(x) => FoldedStreamNode::NumericLiteral(x.clone()),
            Token::CharLiteral(x) => FoldedStreamNode::CharLiteral(x.clone()),
            Token::StringLiteral(x) => FoldedStreamNode::StringLiteral(x.clone()),
            Token::Dollar => FoldedStreamNode::Dollar,
            Token::Colon => FoldedStreamNode::Colon,
            Token::Octothorp => FoldedStreamNode::Octothorp,
            Token::AtSign => FoldedStreamNode::AtSign,
            Token::QMark => FoldedStreamNode::QMark,
        })
    }

    out
}
pub fn fold_stream(stream: Vec<Token>) -> Vec<FoldedStreamNode> {
    let mut out = Vec::new();

    let mut stream = stream.iter();

    while let Some(x) = stream.next() {
        out.push(match x {
            Token::Square(true) => FoldedStreamNode::Square(fold_until_sqend(&mut stream)),
            Token::Curly(true) => FoldedStreamNode::Curly(fold_until_cuend(&mut stream)),

            Token::Square(false) => panic!("missmatch perens"),
            Token::Curly(false) => panic!("missmatch perens"),

            Token::Comment(x) => FoldedStreamNode::Comment(x.clone()),
            Token::Ident(x) => FoldedStreamNode::Ident(x.clone()),
            Token::NumericLiteral(x) => FoldedStreamNode::NumericLiteral(x.clone()),
            Token::CharLiteral(x) => FoldedStreamNode::CharLiteral(x.clone()),
            Token::StringLiteral(x) => FoldedStreamNode::StringLiteral(x.clone()),
            Token::Dollar => FoldedStreamNode::Dollar,
            Token::Colon => FoldedStreamNode::Colon,
            Token::Octothorp => FoldedStreamNode::Octothorp,
            Token::AtSign => FoldedStreamNode::AtSign,
            Token::QMark => FoldedStreamNode::QMark,
        })
    }

    out
}

type FoldedStream<'a> = std::iter::Peekable<Iter<'a, FoldedStreamNode>>;

pub fn fold_to_node(node: &mut FoldedStream) -> anyhow::Result<ASTNode> {
    if let Some(&x) = node.next() {
        Ok(match x {
            FoldedStreamNode::Square(values) => {
                let mut out = Vec::with_capacity(values.len());

                let mut values: FoldedStream = values.iter().peekable();

                while let Some(x) = values.peek() {
                    out.push(fold_to_node(&mut values)?);
                }

                ASTNode::Square(out)
            }
            FoldedStreamNode::Curly(values) => {
                let mut out = Vec::with_capacity(values.len());

                let mut values: FoldedStream = values.iter().peekable();

                while let Some(x) = values.peek() {
                    out.push(fold_to_node(&mut values)?);
                }

                ASTNode::Curly(out)
            }

            FoldedStreamNode::Comment(x) => ASTNode::Comment(x),
            FoldedStreamNode::Ident(x) => ASTNode::Ident(x),
            FoldedStreamNode::NumericLiteral(x) => {
                ASTNode::NumericLiteral(NumericLiteral::from_str(x.as_str())?)
            }
            FoldedStreamNode::CharLiteral(char) => ASTNode::CharLiteral(char),
            FoldedStreamNode::StringLiteral(s) => ASTNode::StringLiteral(s),

            FoldedStreamNode::Dollar => {}
            FoldedStreamNode::Colon => todo!(),
            FoldedStreamNode::Octothorp => todo!(),
            FoldedStreamNode::AtSign => todo!(),
            FoldedStreamNode::QMark => todo!(),
        })
    } else {
        bail!("Token stream empty at EOF")
    }
}

pub fn build_tree(stream: Vec<Token>) -> Vec<TopLevelNode> {
    let mut out = Vec::new();

    let stream = fold_stream(stream);
    let mut stream: FoldedStream = stream.iter().peekable();

    while let Some(x) = stream.peek() {
        match x {
            FoldedStreamNode::Comment(x) => {
                out.push(TopLevelNode::Comment(x.clone()));
            }
            FoldedStreamNode::QMark => {}
            FoldedStreamNode::AtSign => {}
            _ => panic!("Invalid top-level deceleration"),
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use crate::{ast::fold_stream, tokenizer::tokenizer};

    #[test]
    fn experiments() {
        let program = "@main {\"Hello world!\\n\".}";
        let tokens = tokenizer(program.to_string()).unwrap();
        println!("{:?}", fold_stream(tokens));
    }
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
// if       If          -Callable -a! +a!
// else     Else        -Callable -a! +a!
// +        Add         -Add:a -a +a
// -        Sub         -Sub:a -a +a
// *        Mul         -Mul:a -a +a
// /        Div         -Div:a -a +a
// =        Eq          -Eq:a -a +Bool
// /=       Neq         -Eq:a -a +Bool
// >        Lt          -Ord:a -a +Bool
// <        Gt          -Ord:a -a +Bool

/*
// HELLO WORLD
?main []
@main { "Hello world!" . }

// FIB
?fib [-a@(1 | 2) +a]
@fib { drop 1 }

?fib [-a +a]
@fib { dup 1 - fib swap 2 - fib + }

?main []
@main { 10i fib . }

// VECTOR ADD
?for [-a -a -a]
@for {}

@main {
    10 ::size
    $:size :#array[]
}

// IF

@main {
    1 0 = { "They are equal" . } else { "They are not equal" } if
}
*/
