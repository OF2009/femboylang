use std::io::Read;

use logos::{Logos, Source};
use crate::tokens::token::Token;
mod tokens;
mod parser;
mod interpreter;


fn main() {
    let mut buf = Vec::new();
    let mut file = std::fs::File::open("example.fl");
    if let Ok(mut some_file ) = file{
        some_file.read_to_end(&mut buf);

    }else {
        println!("Failed to open file");
        return
    }
    let text  =String::from_utf8(buf).unwrap();
    
    let mut lexer = Token::lexer(&text);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next(){
        if let Ok(token) = token{
            tokens.push(token);
        }
    }
    let states = parser::parser::parse(tokens.as_slice());
    let states =  match states{
        Ok(v)=>{v}
        Err(e)=>{
            println!("Failed parsing: {:?}",e);
            return;
        }

    };
    
    interpreter::interpret::interpret(states.as_slice())
    

    
}
