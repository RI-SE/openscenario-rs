//! OpenSCENARIO expression parsing and evaluation engine
//!
//! This module provides:
//! - Expression parsing for OpenSCENARIO's mathematical expressions
//! - Expression evaluation with parameter substitution
//! - Support for ${expression} syntax from the XSD schema
//! - Comprehensive error handling for invalid expressions
//!
//! Supported operators: +, -, *, /, %, (, )
//! Supported types: numeric literals, parameters, function calls
//!
//! XSD Pattern: `[$][{][ A-Za-z0-9_\+\-\*/%$\(\)\.,]*[\}]`

use crate::error::{Error, Result};
use std::collections::HashMap;
use std::str::FromStr;

/// Expression token types for parsing
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Parameter(String),
    Operator(Operator),
    LeftParen,
    RightParen,
}

/// Supported mathematical operators
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

/// Abstract syntax tree node for expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Parameter(String),
    BinaryOp {
        left: Box<Expr>,
        operator: Operator,
        right: Box<Expr>,
    },
    UnaryMinus(Box<Expr>),
}

/// Expression parser for OpenSCENARIO mathematical expressions
#[derive(Debug)]
pub struct ExpressionParser {
    tokens: Vec<Token>,
    current: usize,
}

impl ExpressionParser {
    /// Create a new parser with the given expression string
    pub fn new(expr: &str) -> Result<Self> {
        let tokens = Self::tokenize(expr)?;
        Ok(Self { tokens, current: 0 })
    }

    /// Parse the expression into an AST
    pub fn parse(&mut self) -> Result<Expr> {
        let expr = self.parse_expression()?;
        if self.current < self.tokens.len() {
            return Err(Error::validation_error("expression", "unexpected token after expression"));
        }
        Ok(expr)
    }

    /// Tokenize the input expression string
    fn tokenize(input: &str) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(&ch) = chars.peek() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => {
                    chars.next();
                }
                '+' => {
                    tokens.push(Token::Operator(Operator::Add));
                    chars.next();
                }
                '-' => {
                    tokens.push(Token::Operator(Operator::Subtract));
                    chars.next();
                }
                '*' => {
                    tokens.push(Token::Operator(Operator::Multiply));
                    chars.next();
                }
                '/' => {
                    tokens.push(Token::Operator(Operator::Divide));
                    chars.next();
                }
                '%' => {
                    tokens.push(Token::Operator(Operator::Modulo));
                    chars.next();
                }
                '(' => {
                    tokens.push(Token::LeftParen);
                    chars.next();
                }
                ')' => {
                    tokens.push(Token::RightParen);
                    chars.next();
                }
                '$' => {
                    // Parameter reference: ${paramName} or $paramName
                    chars.next(); // consume '$'
                    if chars.peek() == Some(&'{') {
                        chars.next(); // consume '{'
                        let param_name = Self::read_until_char(&mut chars, '}')?;
                        if chars.next() != Some('}') {
                            return Err(Error::validation_error("parameter", "missing closing brace in parameter reference"));
                        }
                        tokens.push(Token::Parameter(param_name));
                    } else {
                        // Simple $paramName format (deprecated but supported)
                        let param_name = Self::read_identifier(&mut chars)?;
                        if param_name.is_empty() {
                            return Err(Error::validation_error("parameter", "empty parameter name"));
                        }
                        tokens.push(Token::Parameter(param_name));
                    }
                }
                '0'..='9' | '.' => {
                    let number = Self::read_number(&mut chars)?;
                    tokens.push(Token::Number(number));
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let identifier = Self::read_identifier(&mut chars)?;
                    // For now, treat all identifiers as parameter names
                    tokens.push(Token::Parameter(identifier));
                }
                _ => {
                    return Err(Error::validation_error("tokenize", &format!("unexpected character: '{}'", ch)));
                }
            }
        }

        Ok(tokens)
    }

    /// Read a number from the character stream
    fn read_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<f64> {
        let mut number_str = String::new();
        let mut has_dot = false;

        while let Some(&ch) = chars.peek() {
            match ch {
                '0'..='9' => {
                    number_str.push(ch);
                    chars.next();
                }
                '.' if !has_dot => {
                    has_dot = true;
                    number_str.push(ch);
                    chars.next();
                }
                'e' | 'E' => {
                    // Scientific notation
                    number_str.push(ch);
                    chars.next();
                    if chars.peek() == Some(&'+') || chars.peek() == Some(&'-') {
                        number_str.push(chars.next().unwrap());
                    }
                }
                _ => break,
            }
        }

        number_str.parse::<f64>()
            .map_err(|_| Error::validation_error("number", &format!("invalid number: '{}'", number_str)))
    }

    /// Read an identifier from the character stream
    fn read_identifier(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<String> {
        let mut identifier = String::new();

        while let Some(&ch) = chars.peek() {
            match ch {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    identifier.push(ch);
                    chars.next();
                }
                _ => break,
            }
        }

        Ok(identifier)
    }

    /// Read characters until the specified terminator
    fn read_until_char(chars: &mut std::iter::Peekable<std::str::Chars>, terminator: char) -> Result<String> {
        let mut content = String::new();

        while let Some(&ch) = chars.peek() {
            if ch == terminator {
                break;
            }
            content.push(ch);
            chars.next();
        }

        Ok(content)
    }

    /// Parse an expression with precedence handling
    fn parse_expression(&mut self) -> Result<Expr> {
        self.parse_additive()
    }

    /// Parse additive expressions (+ and -)
    fn parse_additive(&mut self) -> Result<Expr> {
        let mut left = self.parse_multiplicative()?;

        while self.current < self.tokens.len() {
            match &self.tokens[self.current] {
                Token::Operator(Operator::Add) => {
                    self.current += 1;
                    let right = self.parse_multiplicative()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        operator: Operator::Add,
                        right: Box::new(right),
                    };
                }
                Token::Operator(Operator::Subtract) => {
                    self.current += 1;
                    let right = self.parse_multiplicative()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        operator: Operator::Subtract,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    /// Parse multiplicative expressions (*, /, %)
    fn parse_multiplicative(&mut self) -> Result<Expr> {
        let mut left = self.parse_unary()?;

        while self.current < self.tokens.len() {
            match &self.tokens[self.current] {
                Token::Operator(Operator::Multiply) => {
                    self.current += 1;
                    let right = self.parse_unary()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        operator: Operator::Multiply,
                        right: Box::new(right),
                    };
                }
                Token::Operator(Operator::Divide) => {
                    self.current += 1;
                    let right = self.parse_unary()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        operator: Operator::Divide,
                        right: Box::new(right),
                    };
                }
                Token::Operator(Operator::Modulo) => {
                    self.current += 1;
                    let right = self.parse_unary()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        operator: Operator::Modulo,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    /// Parse unary expressions (unary minus)
    fn parse_unary(&mut self) -> Result<Expr> {
        if self.current < self.tokens.len() {
            match &self.tokens[self.current] {
                Token::Operator(Operator::Subtract) => {
                    self.current += 1;
                    let expr = self.parse_primary()?;
                    return Ok(Expr::UnaryMinus(Box::new(expr)));
                }
                _ => {}
            }
        }
        self.parse_primary()
    }

    /// Parse primary expressions (numbers, parameters, parentheses)
    fn parse_primary(&mut self) -> Result<Expr> {
        if self.current >= self.tokens.len() {
            return Err(Error::validation_error("expression", "unexpected end of expression"));
        }

        let token = &self.tokens[self.current].clone();
        self.current += 1;

        match token {
            Token::Number(n) => Ok(Expr::Number(*n)),
            Token::Parameter(name) => Ok(Expr::Parameter(name.clone())),
            Token::LeftParen => {
                let expr = self.parse_expression()?;
                if self.current >= self.tokens.len() || self.tokens[self.current] != Token::RightParen {
                    return Err(Error::validation_error("expression", "missing closing parenthesis"));
                }
                self.current += 1;
                Ok(expr)
            }
            _ => Err(Error::validation_error("expression", &format!("unexpected token: {:?}", token))),
        }
    }
}

/// Expression evaluator for OpenSCENARIO expressions
pub struct ExpressionEvaluator {
    parameters: HashMap<String, String>,
}

impl ExpressionEvaluator {
    /// Create a new evaluator with the given parameter context
    pub fn new(parameters: HashMap<String, String>) -> Self {
        Self { parameters }
    }

    /// Evaluate an expression AST to a numeric result
    pub fn evaluate(&self, expr: &Expr) -> Result<f64> {
        match expr {
            Expr::Number(n) => Ok(*n),
            Expr::Parameter(name) => {
                let param_value = self.parameters
                    .get(name)
                    .ok_or_else(|| Error::parameter_error(name, "parameter not found"))?;
                
                param_value.parse::<f64>().map_err(|e| {
                    Error::parameter_error(name, &format!("failed to parse '{}': {}", param_value, e))
                })
            }
            Expr::BinaryOp { left, operator, right } => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                
                match operator {
                    Operator::Add => Ok(left_val + right_val),
                    Operator::Subtract => Ok(left_val - right_val),
                    Operator::Multiply => Ok(left_val * right_val),
                    Operator::Divide => {
                        if right_val == 0.0 {
                            Err(Error::parameter_error("division", "division by zero"))
                        } else {
                            Ok(left_val / right_val)
                        }
                    }
                    Operator::Modulo => {
                        if right_val == 0.0 {
                            Err(Error::parameter_error("modulo", "modulo by zero"))
                        } else {
                            Ok(left_val % right_val)
                        }
                    }
                }
            }
            Expr::UnaryMinus(expr) => {
                let val = self.evaluate(expr)?;
                Ok(-val)
            }
        }
    }
}

/// Parse and evaluate an OpenSCENARIO expression
pub fn evaluate_expression<T>(
    expr: &str, 
    params: &HashMap<String, String>
) -> Result<T>
where 
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let mut parser = ExpressionParser::new(expr)?;
    let ast = parser.parse()?;
    let evaluator = ExpressionEvaluator::new(params.clone());
    let result = evaluator.evaluate(&ast)?;
    
    // Convert the numeric result to the target type
    let result_str = result.to_string();
    result_str.parse::<T>().map_err(|e| {
        Error::parameter_error(expr, &format!("failed to parse result '{}': {}", result_str, e))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_numbers() {
        let tokens = ExpressionParser::tokenize("123.45").unwrap();
        assert_eq!(tokens, vec![Token::Number(123.45)]);
        
        let tokens = ExpressionParser::tokenize("1.5e-3").unwrap();
        assert_eq!(tokens, vec![Token::Number(0.0015)]);
    }

    #[test]
    fn test_tokenize_operators() {
        let tokens = ExpressionParser::tokenize("+ - * / %").unwrap();
        assert_eq!(tokens, vec![
            Token::Operator(Operator::Add),
            Token::Operator(Operator::Subtract),
            Token::Operator(Operator::Multiply),
            Token::Operator(Operator::Divide),
            Token::Operator(Operator::Modulo),
        ]);
    }

    #[test]
    fn test_tokenize_parameters() {
        let tokens = ExpressionParser::tokenize("${speed} + $velocity").unwrap();
        assert_eq!(tokens, vec![
            Token::Parameter("speed".to_string()),
            Token::Operator(Operator::Add),
            Token::Parameter("velocity".to_string()),
        ]);
    }

    #[test]
    fn test_parse_simple_expression() {
        let mut parser = ExpressionParser::new("2 + 3").unwrap();
        let ast = parser.parse().unwrap();
        
        match ast {
            Expr::BinaryOp { left, operator, right } => {
                assert_eq!(*left, Expr::Number(2.0));
                assert_eq!(operator, Operator::Add);
                assert_eq!(*right, Expr::Number(3.0));
            }
            _ => panic!("Expected binary operation"),
        }
    }

    #[test]
    fn test_parse_precedence() {
        let mut parser = ExpressionParser::new("2 + 3 * 4").unwrap();
        let ast = parser.parse().unwrap();
        
        // Should parse as 2 + (3 * 4), not (2 + 3) * 4
        match ast {
            Expr::BinaryOp { left, operator, right } => {
                assert_eq!(*left, Expr::Number(2.0));
                assert_eq!(operator, Operator::Add);
                match *right {
                    Expr::BinaryOp { left, operator, right } => {
                        assert_eq!(*left, Expr::Number(3.0));
                        assert_eq!(operator, Operator::Multiply);
                        assert_eq!(*right, Expr::Number(4.0));
                    }
                    _ => panic!("Expected multiplication on right side"),
                }
            }
            _ => panic!("Expected addition at root"),
        }
    }

    #[test]
    fn test_parse_parentheses() {
        let mut parser = ExpressionParser::new("(2 + 3) * 4").unwrap();
        let ast = parser.parse().unwrap();
        
        // Should parse as (2 + 3) * 4
        match ast {
            Expr::BinaryOp { left, operator, right } => {
                assert_eq!(operator, Operator::Multiply);
                assert_eq!(*right, Expr::Number(4.0));
                match *left {
                    Expr::BinaryOp { left, operator, right } => {
                        assert_eq!(*left, Expr::Number(2.0));
                        assert_eq!(operator, Operator::Add);
                        assert_eq!(*right, Expr::Number(3.0));
                    }
                    _ => panic!("Expected addition in parentheses"),
                }
            }
            _ => panic!("Expected multiplication at root"),
        }
    }

    #[test]
    fn test_evaluate_simple() {
        let params = HashMap::new();
        let evaluator = ExpressionEvaluator::new(params);
        
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::Number(2.0)),
            operator: Operator::Add,
            right: Box::new(Expr::Number(3.0)),
        };
        
        let result = evaluator.evaluate(&expr).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_evaluate_with_parameters() {
        let mut params = HashMap::new();
        params.insert("speed".to_string(), "30.0".to_string());
        params.insert("acceleration".to_string(), "2.5".to_string());
        
        let evaluator = ExpressionEvaluator::new(params);
        
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::Parameter("speed".to_string())),
            operator: Operator::Add,
            right: Box::new(Expr::Parameter("acceleration".to_string())),
        };
        
        let result = evaluator.evaluate(&expr).unwrap();
        assert_eq!(result, 32.5);
    }

    #[test]
    fn test_evaluate_division_by_zero() {
        let params = HashMap::new();
        let evaluator = ExpressionEvaluator::new(params);
        
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::Number(5.0)),
            operator: Operator::Divide,
            right: Box::new(Expr::Number(0.0)),
        };
        
        let result = evaluator.evaluate(&expr);
        assert!(result.is_err());
    }

    #[test]
    fn test_end_to_end_evaluation() {
        let mut params = HashMap::new();
        params.insert("speed".to_string(), "30.0".to_string());
        params.insert("time".to_string(), "2.0".to_string());
        
        // Test: speed * time + 10
        let result: f64 = evaluate_expression("${speed} * ${time} + 10", &params).unwrap();
        assert_eq!(result, 70.0);
        
        // Test: (speed + 10) / time
        let result: f64 = evaluate_expression("(${speed} + 10) / ${time}", &params).unwrap();
        assert_eq!(result, 20.0);
    }

    #[test]
    fn test_complex_expression() {
        let mut params = HashMap::new();
        params.insert("a".to_string(), "5.0".to_string());
        params.insert("b".to_string(), "3.0".to_string());
        params.insert("c".to_string(), "2.0".to_string());
        
        // Test: a * (b + c) - b / c
        let result: f64 = evaluate_expression("${a} * (${b} + ${c}) - ${b} / ${c}", &params).unwrap();
        assert_eq!(result, 23.5); // 5 * (3 + 2) - 3 / 2 = 25 - 1.5 = 23.5
    }

    #[test]
    fn test_unary_minus() {
        let mut params = HashMap::new();
        params.insert("value".to_string(), "10.0".to_string());
        
        let result: f64 = evaluate_expression("-${value} + 5", &params).unwrap();
        assert_eq!(result, -5.0);
        
        let result: f64 = evaluate_expression("-(${value} + 5)", &params).unwrap();
        assert_eq!(result, -15.0);
    }
}