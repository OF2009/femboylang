use std::collections::HashMap;

use crate::parser::defs::{self, Expression, Operation, Value};

use super::handlers::handle_print;
pub fn interpret(values: &[defs::Defs]){
    let mut map = HashMap::new();
    for value in values{
        match value{
            

            
            defs::Defs::Print(defs::Print{value: val})=>{
                match val{
                    Value::VarName(s)=>{
                        
                        println!("{:?}",map[s.as_str()]);
                    }
                _=>{println!("{:?}",val)}
                }
                
            }
            defs::Defs::Assignment(defs::Assignment{var_name,value: val})=>{
                map.insert(var_name.to_string(), val);
            }
        }
        
    }

}
//TODO 
fn evaluate_expression<'a>(expression: Expression<'a>)->Result<Value<'a>,()>{
    for op in expression.ops{
        return Ok(match op{
            Operation::Add(val1,val2 )=>*val1+*val2,
            _=>{panic!("re")}
        })
    }
    Err(())
}