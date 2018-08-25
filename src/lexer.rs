#[derive(Debug, PartialEq)]
pub enum Token {
    Num(usize),
    Plus,
    Minus,
    Asterisk,
    Slash,
}

#[derive(Debug, PartialEq)]
pub struct Lexer {
    code: String,
    pos: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(code: &str) -> Self {
        Lexer {
            code: code.to_string(),
            pos: 0,
            tokens: vec![],
        }
    }

    pub fn run(mut self) -> Result<Vec<Token>, ()> {
        while !self.is_eof() {
            self = self.read_token()?;
        }
        Ok(self.tokens)
    }
}

impl Lexer {
    fn read_token(self) -> Result<Self, ()> {
        match self.peek()? {
            // 'a'...'z' | 'A'...'Z' => self.read_key_id(),
            '0'...'9' => self.read_num(),
            '\n' | 't' | ' ' => self.step().read_token(),
            _ => self.read_symbol(),
        }
    }

    fn read_num(mut self) -> Result<Self, ()> {
        let (l, num) = self.cut_token(|c| c.is_numeric())?;
        self = l;
        match num.parse() {
            Ok(n) => {
                self.tokens.push(Token::Num(n));
                Ok(self)
            }
            Err(_e) => Err(()),
        }
    }

    fn read_symbol(mut self) -> Result<Self, ()> {
        let token = match self.peek()? {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            _ => return Err(()),
        };
        self = self.step();
        self.tokens.push(token);
        Ok(self)
    }
}

impl Lexer {
    fn step(mut self) -> Self {
        self.pos += 1;
        self
    }

    fn peek(&self) -> Result<char, ()> {
        self.code[self.pos..].chars().next().ok_or(())
    }

    fn cut_token<F>(mut self, mut cond: F) -> Result<(Self, String), ()>
    where
        F: FnMut(char) -> bool,
    {
        let mut t = "".to_string();
        while !self.is_eof() && cond(self.peek()?) {
            t.push(self.peek().unwrap());
            self = self.step();
        }
        Ok((self, t))
    }

    fn is_eof(&self) -> bool {
        self.code.len() <= self.pos
    }
}

#[test]
fn run_test() {
    let a = Lexer::new("20+3-5");
    assert_eq!(
        a.run().unwrap(),
        vec![
            Token::Num(20),
            Token::Plus,
            Token::Num(3),
            Token::Minus,
            Token::Num(5),
        ]
    );
}

#[test]
fn read_token_test() {
    let a1 = Lexer::new("20+3-5"); // 18
    let a2 = a1.read_token().unwrap();
    assert_eq!(
        a2,
        Lexer {
            code: "20+3-5".to_string(),
            pos: 2,
            tokens: vec![Token::Num(20)],
        }
    );
    let a3 = a2.read_token().unwrap();
    assert_eq!(
        a3,
        Lexer {
            code: "20+3-5".to_string(),
            pos: 3,
            tokens: vec![Token::Num(20), Token::Plus],
        }
    );
    let a4 = a3.read_token().unwrap();
    assert_eq!(
        a4,
        Lexer {
            code: "20+3-5".to_string(),
            pos: 4,
            tokens: vec![Token::Num(20), Token::Plus, Token::Num(3)],
        }
    );
    let a5 = a4.read_token().unwrap();
    assert_eq!(
        a5,
        Lexer {
            code: "20+3-5".to_string(),
            pos: 5,
            tokens: vec![Token::Num(20), Token::Plus, Token::Num(3), Token::Minus],
        }
    );
    let a6 = a5.read_token().unwrap();
    assert_eq!(
        a6,
        Lexer {
            code: "20+3-5".to_string(),
            pos: 6,
            tokens: vec![
                Token::Num(20),
                Token::Plus,
                Token::Num(3),
                Token::Minus,
                Token::Num(5),
            ],
        }
    );
    assert!(a6.is_eof())
}

#[test]
fn read_symbol_test() {
    let a = Lexer::new("+*12");
    assert_eq!(
        a.read_symbol().unwrap(),
        Lexer {
            code: "+*12".to_string(),
            pos: 1,
            tokens: vec![Token::Plus],
        }
    );
    let a = Lexer::new("*12");
    assert_eq!(
        a.read_symbol().unwrap(),
        Lexer {
            code: "*12".to_string(),
            pos: 1,
            tokens: vec![Token::Asterisk],
        }
    );
}

#[test]
fn read_num_test() {
    let a = Lexer::new("12345a");
    assert_eq!(
        a.read_num().unwrap(),
        Lexer {
            code: "12345a".to_string(),
            pos: 5,
            tokens: vec![Token::Num(12345)],
        }
    );
}

#[test]
fn cut_token_test() {
    let a = Lexer::new("12345a");
    assert_eq!(
        a.cut_token(|c| c.is_numeric()).unwrap(),
        (
            Lexer {
                code: "12345a".to_string(),
                pos: 5,
                tokens: vec![]
            },
            "12345".to_string()
        )
    );
}

#[test]
fn lexer_peek_test() {
    let a = Lexer::new("abc");
    assert_eq!(a.peek().unwrap(), 'a');
}

#[test]
fn lexer_step_test() {
    let a = Lexer::new("abc");
    assert_eq!(
        a.step(),
        Lexer {
            code: "abc".to_string(),
            pos: 1,
            tokens: vec![],
        }
    );
}

#[test]
fn lexer_new_test() {
    let a = Lexer::new("abc");
    assert_eq!(
        a,
        Lexer {
            code: "abc".to_string(),
            pos: 0,
            tokens: vec![],
        }
    );
}
