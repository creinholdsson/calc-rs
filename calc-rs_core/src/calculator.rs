use std::collections::VecDeque;

use crate::parser::*;

pub struct Calculator {}

#[derive(Debug, PartialEq)]
pub enum CalculationResult {
    Integer(i64),
    Float(f64),
}

impl Calculator {
    fn multiply(a: Operand, b: Operand) -> Operand {
        println!("multiply a:{:?} b: {:?}", a, b);
        match a {
            Operand::Integer(x) => match b {
                Operand::Float(y) => Operand::Float(x as f64 * y),
                Operand::Integer(y) => Operand::Integer(x * y),
            },
            Operand::Float(x) => match b {
                Operand::Integer(y) => Operand::Float(x * y as f64),
                Operand::Float(y) => Operand::Float(x * y),
            },
        }
    }

    fn add(a: Operand, b: Operand) -> Operand {
        println!("add a:{:?} b: {:?}", a, b);
        match a {
            Operand::Integer(x) => match b {
                Operand::Float(y) => Operand::Float(x as f64 + y),
                Operand::Integer(y) => Operand::Integer(x + y),
            },
            Operand::Float(x) => match b {
                Operand::Integer(y) => Operand::Float(x + y as f64),
                Operand::Float(y) => Operand::Float(x + y),
            },
        }
    }

    fn subtract(a: Operand, b: Operand) -> Operand {
        println!("subtract a:{:?} b: {:?}", a, b);
        match a {
            Operand::Integer(x) => match b {
                Operand::Float(y) => Operand::Float(x as f64 - y),
                Operand::Integer(y) => Operand::Integer(x - y),
            },
            Operand::Float(x) => match b {
                Operand::Integer(y) => Operand::Float(x - y as f64),
                Operand::Float(y) => Operand::Float(x - y),
            },
        }
    }

    fn divide(a: Operand, b: Operand) -> Operand {
        println!("divide a:{:?} b: {:?}", a, b);
        match a {
            Operand::Integer(x) => match b {
                Operand::Float(y) => Operand::Float(x as f64 / y),
                Operand::Integer(y) => Operand::Integer(x / y),
            },
            Operand::Float(x) => match b {
                Operand::Integer(y) => Operand::Float(x / y as f64),
                Operand::Float(y) => Operand::Float(x / y),
            },
        }
    }

    /*
        tokens is in reverse polish notation (RPN)
    */
    pub fn calculate(tokens: &mut VecDeque<Token>) -> CalculationResult {
        let mut stack: Vec<Operand> = vec![];

        while tokens.len() > 0 {
            match tokens.pop_front().unwrap() {
                Token::Operator(x) => {
                    println!("Found operator: {:?}", x);
                    let second = stack.pop().unwrap();
                    let first = stack.pop().unwrap();
                    match x {
                        Op::Addition => stack.push(Calculator::add(first, second)),
                        Op::Subtraction => stack.push(Calculator::subtract(first, second)),
                        Op::Multiplication => stack.push(Calculator::multiply(first, second)),
                        Op::Division => stack.push(Calculator::divide(first, second)),
                    }
                }
                Token::Oper(x) => {
                    println!("Operand: {:?}", x);
                    stack.push(x)
                }
            }
        }

        assert!(stack.len() == 1);
        match stack.pop().unwrap() {
            Operand::Float(x) => CalculationResult::Float(x),
            Operand::Integer(x) => CalculationResult::Integer(x),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::calculator::*;

    #[test]
    fn test_calculation() {
        let mut tokens: VecDeque<Token> = VecDeque::new();

        tokens.push_back(Token::Oper(Operand::Integer(4)));
        tokens.push_back(Token::Oper(Operand::Integer(4)));
        tokens.push_back(Token::Operator(Op::Addition));

        let calc_result = Calculator::calculate(&mut tokens);

        let result = match calc_result {
            CalculationResult::Integer(x) => x,
            CalculationResult::Float(_) => 0,
        };

        assert_eq!(result, 8);
    }

    #[test]
    fn test_calculation_subt() {
        let mut tokens: VecDeque<Token> = VecDeque::new();

        tokens.push_back(Token::Oper(Operand::Integer(4)));
        tokens.push_back(Token::Oper(Operand::Integer(3)));
        tokens.push_back(Token::Operator(Op::Subtraction));

        let calc_result = Calculator::calculate(&mut tokens);

        let result = match calc_result {
            CalculationResult::Integer(x) => x,
            CalculationResult::Float(_) => 0,
        };

        assert_eq!(result, 1);
    }

    #[test]
    fn test_calculation_mult() {
        let mut tokens: VecDeque<Token> = VecDeque::new();

        tokens.push_back(Token::Oper(Operand::Integer(4)));
        tokens.push_back(Token::Oper(Operand::Integer(3)));
        tokens.push_back(Token::Operator(Op::Multiplication));

        let calc_result = Calculator::calculate(&mut tokens);

        let result = match calc_result {
            CalculationResult::Integer(x) => x,
            CalculationResult::Float(_) => 0,
        };

        assert_eq!(result, 12);
    }

    #[test]
    fn test_calculation_div() {
        let mut tokens: VecDeque<Token> = VecDeque::new();

        tokens.push_back(Token::Oper(Operand::Integer(9)));
        tokens.push_back(Token::Oper(Operand::Integer(3)));
        tokens.push_back(Token::Operator(Op::Division));

        let calc_result = Calculator::calculate(&mut tokens);

        let result = match calc_result {
            CalculationResult::Integer(x) => x,
            CalculationResult::Float(_) => 0,
        };

        assert_eq!(result, 3);
    }
}
