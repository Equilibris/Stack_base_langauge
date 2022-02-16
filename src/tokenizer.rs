use core::panic;

#[derive(Debug)]
pub enum Token {
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

    Square(bool), // True = open, False = close
    Curly(bool),  // True = open, False = close
}
type CharStream<'a> = std::iter::Peekable<std::str::Chars<'a>>;

fn escape_char(s: &mut CharStream) -> char {
    s.next();
    match s.peek() {
        Some('n') => '\n',
        Some('r') => '\r',
        Some('\n') => '\n',
        Some(x) => {
            println!("{}", x);
            todo!()
        }
        _ => todo!(),
    }
}

fn numeric_tokenize(s: &mut CharStream) -> Token {
    let mut out = String::new();

    while let Some(x @ (('0'..='9') | '-' | '.' | 'E' | 'd' | 'b' | 'f' | 'i' | 'u')) = s.peek() {
        out.push(*x);
        s.next();
    }

    Token::NumericLiteral(out)
}

fn char_tokenizer(s: &mut CharStream) -> Token {
    s.next();

    let v = match s.peek() {
        Some('\\') => Token::CharLiteral(escape_char(s)),
        Some(x) => Token::CharLiteral(*x),
        _ => panic!("Unimplemented"),
    };
    s.next();
    v
}

fn string_tokenizer(s: &mut CharStream) -> Token {
    s.next();
    let mut string = String::new();

    loop {
        match s.peek() {
            Some('"') => break,
            Some('\\') => string.push(escape_char(s)),
            Some(x) => string.push(*x),
            None => todo!(),
        }
        s.next();
    }

    Token::StringLiteral(string)
}

fn ident_tokenizer(s: &mut CharStream) -> Token {
    let mut string = String::new();
    while let Some(x) = s.peek() {
        if x.is_whitespace() {
            break;
        }
        if let '[' | ']' | '{' | '}' = *x {
            break;
        }
        string.push(*x);
        s.next();
    }

    Token::Ident(string)
}

fn comment_tokenizer(s: &mut CharStream) -> Token {
    let mut string = String::new();
    while let Some(x) = s.peek() {
        if '\n' == *x {
            break;
        }
        string.push(*x);
        s.next();
    }

    Token::Comment(string)
}

pub fn tokenizer(input: String) -> anyhow::Result<Vec<Token>> {
    let mut out = Vec::new();

    let mut stream: CharStream = input.chars().peekable();
    while let Some(x) = stream.peek() {
        if x.is_whitespace() {
            stream.next();

            // if let Some(next) = stream.peek() {
            //     if !next.is_whitespace() {
            //         out.push(Token::Spacer)
            //     }
            // }
        } else {
            // TODO: Comments
            out.push(match x {
                '$' => Token::Dollar,
                ':' => Token::Colon,
                '#' => Token::Octothorp,
                '@' => Token::AtSign,
                '?' => Token::QMark,

                '{' => Token::Curly(true),
                '}' => Token::Curly(false),
                '[' => Token::Square(true),
                ']' => Token::Square(false),

                ';' => comment_tokenizer(&mut stream),
                '"' => string_tokenizer(&mut stream),
                '\'' => char_tokenizer(&mut stream),
                '0'..='9' | '-' => {
                    out.push(numeric_tokenize(&mut stream));
                    continue;
                }
                _ => {
                    out.push(ident_tokenizer(&mut stream));
                    continue;
                }
            });
            stream.next();
        }
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::{tokenizer, Token};

    #[test]
    fn it_escape_chars_in_strings() {
        let program = "\"Hello world!\\n\"\"Hello world!\\n\" ";

        let result = tokenizer(program.to_string()).unwrap();

        let goal = "Hello world!\n".to_string();

        if let Token::StringLiteral(x) = &result[0] {
            assert_eq!(x.clone(), goal);
        } else {
            panic!("Failed to escape")
        }

        if let Token::StringLiteral(x) = &result[1] {
            assert_eq!(x.clone(), goal);
        } else {
            panic!("Failed to escape")
        }
    }

    #[test]
    fn it_escape_chars_in_chars() {
        let program = "'\\n''\\n'";

        let result = tokenizer(program.to_string()).unwrap();

        let goal = '\n';

        if let Token::CharLiteral(x) = &result[0] {
            assert_eq!(*x, goal);
        } else {
            panic!("Failed to escape")
        }

        if let Token::CharLiteral(x) = &result[1] {
            assert_eq!(*x, goal);
        } else {
            panic!("Failed to escape")
        }
    }

    #[test]
    fn exp_0() {
        let program = "@main { \"Hello world!\\n\" . }";

        println!("{:?}", tokenizer(program.to_string()));
    }
    #[test]
    fn exp_1() {
        let program = "@main { \"Hello world!\" . }";

        println!("{:?}", tokenizer(program.to_string()));
    }

    #[test]
    fn exp_2() {
        let program = "@main {   \"Hello world!\"  .   }";

        println!("{:?}", tokenizer(program.to_string()));
    }
    #[test]
    fn exp_3() {
        let program = "?fib[-a@(1 | 2) +a] @fib { drop 1 }

    ?fib[-a +a] @fib { dup 1 - fib swap 2 - fib + }

    @main { 10 fib . }";

        println!("{:?}", tokenizer(program.to_string()));
    }
}
