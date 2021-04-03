// https://en.wikipedia.org/wiki/Shunting-yard_algorithm
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Parser {
    radix: Radix,
}

#[derive(Debug, Clone, Copy)]
pub enum Radix {
    Binary = 2,
    Decimal = 10,
    Hexadecimal = 16,
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    Float(f64),
    Integer(i64),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Oper(Operand),
    Operator(Op),
}

impl Parser {
    pub fn new(radix: Radix) -> Parser {
        Parser { radix }
    }

    pub fn parse(&self, expr: String) -> VecDeque<Token> {
        let mut operator_stack: Vec<Token> = vec![];
        let mut output_queue: VecDeque<Token> = VecDeque::new();
        for ch in expr.chars() {
            if ch.is_digit(self.radix as u32) {
                output_queue.push_back(Token::Oper(Operand::Integer(
                    ch.to_string().parse::<i64>().unwrap(),
                )));
            } else {
                match ch {
                    '+' => operator_stack.push(Token::Operator(Op::Addition)),
                    '-' => operator_stack.push(Token::Operator(Op::Subtraction)),
                    '/' => operator_stack.push(Token::Operator(Op::Division)),
                    '*' => operator_stack.push(Token::Operator(Op::Multiplication)),
                    _ => {}
                }
            }
        }

        while operator_stack.len() > 0 {
            output_queue.push_back(operator_stack.pop().unwrap());
        }

        return output_queue;
    }
}

#[cfg(test)]
mod test {
    use crate::parser::*;

    #[test]
    fn test_parser() {
        let expr = String::from("4+4");
        let parser = Parser::new(Radix::Decimal);
        let tokens = parser.parse(expr);

        assert_eq!(
            tokens,
            vec![
                Token::Oper(Operand::Integer(4)),
                Token::Oper(Operand::Integer(4)),
                Token::Operator(Op::Addition),
            ]
        );
    }

    #[test]
    fn test_parser_subtract() {
        let expr = String::from("4-3");
        let parser = Parser::new(Radix::Decimal);
        let tokens = parser.parse(expr);
        assert_eq!(
            tokens,
            vec![
                Token::Oper(Operand::Integer(4)),
                Token::Oper(Operand::Integer(3)),
                Token::Operator(Op::Subtraction)
            ]
        );
    }

    #[test]
    fn test_parser_multiply() {
        let expr = String::from("4*3");
        let parser = Parser::new(Radix::Decimal);
        let tokens = parser.parse(expr);
        assert_eq!(
            tokens,
            vec![
                Token::Oper(Operand::Integer(4)),
                Token::Oper(Operand::Integer(3)),
                Token::Operator(Op::Multiplication)
            ]
        );
    }

    #[test]
    fn test_parser_division() {
        let expr = String::from("9/3");
        let parser = Parser::new(Radix::Decimal);
        let tokens = parser.parse(expr);
        assert_eq!(
            tokens,
            vec![
                Token::Oper(Operand::Integer(9)),
                Token::Oper(Operand::Integer(3)),
                Token::Operator(Op::Division)
            ]
        );
    }
}
