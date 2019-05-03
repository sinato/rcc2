use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Associativity {
    Right,
    Left,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    pub precedence: u32,
    pub associativity: Associativity,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Type(String), // TODO change to Enum
    ParenS,
    ParenE,
    BlockS,
    BlockE,
    Semi,
    Return,
    Num(String),
    Op(String, Property),
    Ide(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tokens {
    pub tokens: Vec<Token>,
}
impl Tokens {
    pub fn pop(&mut self) -> Option<Token> {
        self.tokens.reverse();
        let token = self.tokens.pop();
        self.tokens.reverse();
        token
    }
    pub fn peek(&self) -> Option<Token> {
        let mut tokens = self.clone();
        tokens.pop()
    }
}

pub fn get_property(op: &String) -> Property {
    let mut map = HashMap::new();
    map.insert("=", (2, Associativity::Right));
    map.insert("+", (12, Associativity::Left));
    map.insert("*", (13, Associativity::Left));
    let op: &str = &op;
    let (precedence, associativity): (u32, Associativity) = map[op].clone();
    Property {
        precedence,
        associativity,
    }
}

pub struct Lexer {
    re: Regex,
    names: Vec<&'static str>,
}

impl Lexer {
    // static constructor
    pub fn new() -> Lexer {
        let token_patterns = vec![
            ("TYPE", r"int"),
            ("PARENS", r"\("),
            ("PARENE", r"\)"),
            ("BLOCKS", r"\{"),
            ("BLOCKE", r"\}"),
            ("SEMI", r";"),
            ("RETURN", r"return"),
            ("NUM", r"(\d+(\.\d)*)"),
            ("OP", r"(\+|-|\*|=|,)"),
            ("IDE", r"\w+"),
        ];
        let re = make_regex(&token_patterns);
        let names = get_names(&token_patterns);
        let re = Regex::new(&re).expect("something went wrong making the regex");
        Lexer { re, names }
    }
    pub fn lex(&self, code: String) -> Tokens {
        let tokens = self.tokenize(code);
        tokens
    }
    fn tokenize(&self, code: String) -> Tokens {
        let mut tokens: Vec<Token> = Vec::new();
        for caps in self.re.captures_iter(&code) {
            let mut typ = String::from("nil");
            let val = String::from(&caps[0]);
            for name in &self.names {
                if caps.name(name).is_some() {
                    typ = name.to_string();
                }
            }
            let token = match typ.as_ref() {
                "TYPE" => Token::Type(val),
                "PARENS" => Token::ParenS,
                "PARENE" => Token::ParenE,
                "BLOCKS" => Token::BlockS,
                "BLOCKE" => Token::BlockE,
                "SEMI" => Token::Semi,
                "RETURN" => Token::Return,
                "NUM" => Token::Num(val),
                "OP" => {
                    let val = val.trim_end().to_string();
                    Token::Op(val.clone(), get_property(&val))
                }
                "IDE" => Token::Ide(val),
                _ => panic!("This is not an expected panic"),
            };
            tokens.push(token);
        }
        Tokens { tokens }
    }
}
fn make_regex(token_patterns: &Vec<(&str, &str)>) -> String {
    token_patterns
        .into_iter()
        .map(|pattern| format!("(?P<{}>{})", pattern.0, pattern.1))
        .collect::<Vec<String>>()
        .join("|")
}

fn get_names<'a, 'b>(token_patterns: &Vec<(&'a str, &'b str)>) -> Vec<&'a str> {
    token_patterns
        .into_iter()
        .map(|pattern| pattern.0)
        .collect()
}
