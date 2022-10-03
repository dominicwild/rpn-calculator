extern crate core;

use crate::CalculatorInput::Value;
use CalculatorInput::{Add, Divide, Multiply, Subtract};

#[derive(Debug, Copy, Clone)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

impl CalculatorInput {
    fn opt(self, num1: i32, num2: i32) -> i32 {
        match self {
            Add => num1 + num2,
            Subtract => num1 - num2,
            Multiply => num1 * num2,
            Divide => num1 / num2,
            _ => panic!("No operation defined for {:?}", self),
        }
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = vec![];

    for input in inputs {
        match input {
            Value(number) => stack.push(*number),
            operation => {
                if stack.len() < 2 {
                    return None;
                }

                let right_num = stack.pop().unwrap();
                let left_num = stack.pop().unwrap();

                // stack.push(operation.opt(left_num, right_num));

                match input {
                    Add => stack.push(Add.opt(left_num, right_num)),
                    Subtract => stack.push(Subtract.opt(left_num, right_num)),
                    Multiply => stack.push(Multiply.opt(left_num, right_num)),
                    Divide => stack.push(Divide.opt(left_num, right_num)),
                    _ => return None,
                }
            }
        };
    }

    if stack.len() != 1 {
        return None;
    }

    stack.pop()
}
