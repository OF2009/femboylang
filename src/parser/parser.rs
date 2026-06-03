use std::fmt::Debug;

use crate::tokens::token::Token;

use super::defs::{self, Assignment, Defs, Expression, Operation, Value};
#[derive(Debug)]
pub enum ParserError {
    InvalidToken,
    NotValueToken,
    NotStatementToken,
}
pub fn parse(tokens: &[Token]) -> Result<Vec<defs::Defs>, ParserError> {
    let mut res = Vec::new();
    let mut idx: usize = 0;

    while idx < tokens.len() {
        let token = &tokens[idx];
        let element: Defs = match token {
            Token::VarKeyword => parse_def(&tokens[idx..tokens.len()], &mut idx)?,
            Token::Print => parse_print(&tokens[idx..tokens.len()], &mut idx)?,

            _ => return Err(ParserError::InvalidToken),
        };
        res.push(element);
    }
    Ok(res)
}

fn parse_print<'a>(tokens: &'a [Token], idx: &mut usize) -> Result<defs::Defs<'a>, ParserError> {
    let print_val = parse_value(&tokens[2])?;
    *idx += 4;

    let res = defs::Print { value: print_val };
    Ok(defs::Defs::Print(res))
}

fn parse_def<'a>(tokens: &'a [Token], idx: &mut usize) -> Result<defs::Defs<'a>, ParserError> {
    let name = match &tokens[1] {
        Token::VarName(name) => name,
        _ => return Err(ParserError::InvalidToken),
    };
    if tokens[2] != Token::Assign {
        return Err(ParserError::InvalidToken);
    }
    let value = parse_expression(&tokens[3..tokens.len()],idx)?;
    *idx += 4;
    let def = Assignment {
        var_name: name,
        value,
    };
    Ok(Defs::Assignment(def))
}


fn parse_value<'a>(token: &'a Token)->Result<Value<'a>,ParserError>{
    match token{
        Token::String(s)=>Ok(Value::String(s)),
        _=>Err(ParserError::NotValueToken),
    }
}

fn parse_expression<'a>(tokens: &'a [Token], idx: &mut usize) -> Result<Expression<'a>, ParserError> {
    let mut expression = vec![];
    for (end_value_idx, token) in tokens.iter().enumerate() {
        if matches!(token,Token::Add){
            expression.push(Operation::Add(Box::new(parse_value(&tokens[end_value_idx -1])?),Box::new(parse_value(&tokens[end_value_idx +1] )?)));


        }

        
        if matches!(token, Token::NewLine) {

        }
    }
    Err(ParserError::NotStatementToken)
}
