use std::collections::HashMap;

use crate::parser::defs::Value;

pub fn handle_print(print_val: Value,map: HashMap<String,Value>) {
    match print_val {
        Value::VarName(n) => {
            if let Some(value) = map.get(n){
                println!("{}",value)
            }
            else{
                panic!("Non existing variable name")

            }
        }
        _ => {}
    }
    println!("{}", print_val);
}
