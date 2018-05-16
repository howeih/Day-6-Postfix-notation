use std::collections::HashMap;


fn postfix(expression: &str) -> f64 {
    let mut stack = Vec::<f64>::new();
    let mut ops = HashMap::<&str, Box<Fn(f64, f64)->f64>>::new();
    ops.insert("+", Box::new(|op1,op2|{op1+op2}));
    ops.insert("-", Box::new(|op1,op2|{op1-op2}));
    ops.insert("*", Box::new(|op1,op2|{op1*op2}));
    ops.insert("/", Box::new(|op1,op2|{op1/op2}));
    let v: Vec<&str> = expression.split_whitespace().collect();
    for i in v{
        let mut x: f64;
        if ops.contains_key(i){
            let op1 = stack.pop().unwrap();
            let op2 = stack.pop().unwrap();
            let op = ops.get(i).unwrap();
            x = op(op2,op1);
        }else{
            x = i.parse::<f64>().unwrap();
        }
        stack.push(x);
    }
    stack.pop().unwrap()
}

fn main() {
    let res:f64 = postfix("1 2 + 4 6 - + 10 5 / *");
    println!("{}", res)
}
