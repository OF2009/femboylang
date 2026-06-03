use std::{fmt::Error, ops::Add};


pub enum Defs<'a>{
    Assignment( Assignment<'a>),
    Print(Print<'a>),
    

}
#[derive(Debug)]
pub enum Value<'a>{
    Float(f64),
    Int(i128),
    String(& 'a String),
    VarName(& 'a String),
    Operation{val1: Box<Value<'a>>,val2:Box<Value<'a>>,variant: Operation<'a> },
    Expression( Expression<'a>)
}
impl <'a>Add for Value<'a>{
    type Output = Value<'a>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self,rhs){
            (Value::Float(f1),Value::Float(f2))=>Value::Float(f1 + f2),
            (Value::Int(i1),Value::Int(i2))=>Value::Int(i1 + i2),
            _=>{panic!("ss")}
        }
        
    }
}

impl <'a>std::fmt::Display for Value<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Value::Float(float)=>{write!(f,"{float}")}
            Value::Int(int)=>{write!(f,"{int}")}
            Value::String(s)=>{write!(f,"{s}")}
            Value::VarName(s)=>{Err(Error)}
            Value::Operation { val1, val2, variant }=>{write!(f,"{val1}")}
            Value::Expression(s)=>todo!()
        
        }
        
    }

}
pub struct Print<'a>{
    pub value: Value<'a>
}

pub struct Assignment<'a>{
    pub var_name: &'a str,
    pub value: Expression<'a>
}
#[derive(Debug)]
pub enum Operation<'a>{
    Equal(Box<Value<'a>>,Box<Value<'a>>),
    Higher(Box<Value<'a>>,Box<Value<'a>>),
    Lesser(Box<Value<'a>>,Box<Value<'a>>),
    Add(Box<Value<'a>>,Box<Value<'a>>),
    Minus(Box<Value<'a>>,Box<Value<'a>>),
    None(Box<Value<'a>>)
}
#[derive(Debug)]
pub struct Expression<'a>{
    pub ops: Vec<Operation<'a>>
   
}
