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
            _add => {
                if let Some(num1) = stack.pop() {
                    if let Some(num2) = stack.pop() {
                        stack.push(num1 + num2)
                    }
                }

                // let i1 = stack.pop().map(|num| num + stack.pop().unwrap());
                // if let Some(num) = i1 { stack.push(num)};

            }
            // Subtract=> stack.pop(), stack.pop(),
            // Multiply=> stack.pop(), stack.pop(),
            // Divide=> stack.pop(), stack.pop(),
            _ => return None
        };
    }

    stack.pop()
}

