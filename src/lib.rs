use crate::CalculatorInput::Value;

#[derive(Debug, Copy, Clone)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = vec![];

    for input in inputs {
        match input {
            Value(number) => stack.push(*number),
            Add => {
                let i1 = stack.pop().map(|num| num + stack.pop().unwrap());
                if let Some(num) = i1 { stack.push(num)};
                // let option = stack.pop();
                // stack.push(i1)
            },
            // Subtract=> stack.pop(), stack.pop(),
            // Multiply=> stack.pop(), stack.pop(),
            // Divide=> stack.pop(), stack.pop(),
            _ => return None
        };
    }

    stack.pop()
}

