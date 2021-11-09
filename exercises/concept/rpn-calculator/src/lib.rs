use CalculatorInput::*;
use std::ops::{ Add, Sub, Mul, Div };

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    fn apply(stack: &mut Vec<i32>, op: impl Fn(i32, i32) -> i32) -> Option<i32> {
        stack.pop().and_then(|right| stack.pop().and_then(|left| Some(op(left, right))))
    }

    let mut result = inputs.iter().fold(vec!(), |mut stack, input| {
        if let Some(value) = match input {
            Add        => apply(&mut stack, i32::add),
            Subtract   => apply(&mut stack, i32::sub),
            Multiply   => apply(&mut stack, i32::mul),
            Divide     => apply(&mut stack, i32::div),
            Value(val) => Some(*val),
        } {
            stack.push(value);
        }
        stack
    });

    if result.len() == 1 {
        result.pop()
    } else {
        None
    }
}
