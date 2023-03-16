#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stk = vec![];

    for input in inputs {
        match input {
            CalculatorInput::Value(val) => {
                stk.push(*val);
            },
            CalculatorInput::Add |
            CalculatorInput::Subtract |
            CalculatorInput::Divide | 
            CalculatorInput::Multiply => {
                if stk.len() < 2 {
                    return None;
                }
                let rhs =  stk.pop().unwrap();
                let lhs = stk.pop().unwrap();

                match input {
                    CalculatorInput::Add => stk.push(lhs + rhs),
                    CalculatorInput::Subtract => stk.push(lhs - rhs),
                    CalculatorInput::Multiply => stk.push(lhs * rhs),
                    CalculatorInput::Divide => stk.push(lhs / rhs),
                    _ => panic!(),
                }
            },
        }
    }   

    if stk.len() != 1{ 
        return None;
    } 
    stk.pop()
}
