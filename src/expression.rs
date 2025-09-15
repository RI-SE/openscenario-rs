//! OpenSCENARIO expression parsing and evaluation engine
//!
//! This module provides:
//! - Expression parsing for OpenSCENARIO's mathematical expressions
//! - Expression evaluation with parameter substitution
//! - Support for ${expression} syntax from the XSD schema
//! - Comprehensive error handling for invalid expressions
//!
//! Supported operators: +, -, *, /, %, (, ), >, <, >=, <=, ==, !=
//! Supported types: numeric literals, parameters, function calls, constants
//! Supported functions: sin, cos, tan, sqrt, abs, floor, ceil, min, max
//! Supported constants: PI, E
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
    Function(String),
    Constant(String),
    LeftParen,
    RightParen,
    Comma,
}

/// Supported mathematical operators
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    // Comparison operators
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Equal,
    NotEqual,
}

/// Abstract syntax tree node for expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Parameter(String),
    Constant(String),
    BinaryOp {
        left: Box<Expr>,
        operator: Operator,
        right: Box<Expr>,
    },
    UnaryMinus(Box<Expr>),
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
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
            return Err(Error::validation_error(
                "expression",
                "unexpected token after expression",
            ));
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
                ',' => {
                    tokens.push(Token::Comma);
                    chars.next();
                }
                '>' => {
                    chars.next();
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(Token::Operator(Operator::GreaterEqual));
                    } else {
                        tokens.push(Token::Operator(Operator::Greater));
                    }
                }
                '<' => {
                    chars.next();
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(Token::Operator(Operator::LessEqual));
                    } else {
                        tokens.push(Token::Operator(Operator::Less));
                    }
                }
                '=' => {
                    chars.next();
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(Token::Operator(Operator::Equal));
                    } else {
                        return Err(Error::validation_error(
                            "tokenize",
                            "single '=' not supported, use '==' for equality",
                        ));
                    }
                }
                '!' => {
                    chars.next();
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(Token::Operator(Operator::NotEqual));
                    } else {
                        return Err(Error::validation_error(
                            "tokenize",
                            "single '!' not supported, use '!=' for inequality",
                        ));
                    }
                }
                '$' => {
                    // Parameter reference: ${paramName} or $paramName
                    chars.next(); // consume '$'
                    if chars.peek() == Some(&'{') {
                        chars.next(); // consume '{'
                        let param_name = Self::read_until_char(&mut chars, '}')?;
                        if chars.next() != Some('}') {
                            return Err(Error::validation_error(
                                "parameter",
                                "missing closing brace in parameter reference",
                            ));
                        }
                        tokens.push(Token::Parameter(param_name));
                    } else {
                        // Simple $paramName format (deprecated but supported)
                        let param_name = Self::read_identifier(&mut chars)?;
                        if param_name.is_empty() {
                            return Err(Error::validation_error(
                                "parameter",
                                "empty parameter name",
                            ));
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
                    // Check if it's followed by '(' for function call
                    if chars.peek() == Some(&'(') {
                        tokens.push(Token::Function(identifier));
                    } else if Self::is_constant(&identifier) {
                        tokens.push(Token::Constant(identifier));
                    } else {
                        tokens.push(Token::Parameter(identifier));
                    }
                }
                _ => {
                    return Err(Error::validation_error(
                        "tokenize",
                        &format!("unexpected character: '{}'", ch),
                    ));
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

        number_str.parse::<f64>().map_err(|_| {
            Error::validation_error("number", &format!("invalid number: '{}'", number_str))
        })
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
    fn read_until_char(
        chars: &mut std::iter::Peekable<std::str::Chars>,
        terminator: char,
    ) -> Result<String> {
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

    /// Check if identifier is a mathematical constant
    fn is_constant(identifier: &str) -> bool {
        matches!(identifier, "PI" | "E")
    }

    /// Parse an expression with precedence handling
    fn parse_expression(&mut self) -> Result<Expr> {
        self.parse_comparison()
    }

    /// Parse comparison expressions (>, <, >=, <=, ==, !=)
    fn parse_comparison(&mut self) -> Result<Expr> {
        let mut left = self.parse_additive()?;

        while self.current < self.tokens.len() {
            match &self.tokens[self.current] {
                Token::Operator(Operator::Greater) => {
                    self.current += 1;
                    let right = self.parse_additive()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        operator: Operator::Greater,
                        right: Box::new(right),
                    };
                }
                Token::Operator(Operator::Less) => {
                    self.current += 1;
                    let right = self.parse_additive()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        operator: Operator::Less,
                        right: Box::new(right),
                    };
                }
                Token::Operator(Operator::GreaterEqual) => {
                    self.current += 1;
                    let right = self.parse_additive()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        operator: Operator::GreaterEqual,
                        right: Box::new(right),
                    };
                }
                Token::Operator(Operator::LessEqual) => {
                    self.current += 1;
                    let right = self.parse_additive()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        operator: Operator::LessEqual,
                        right: Box::new(right),
                    };
                }
                Token::Operator(Operator::Equal) => {
                    self.current += 1;
                    let right = self.parse_additive()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        operator: Operator::Equal,
                        right: Box::new(right),
                    };
                }
                Token::Operator(Operator::NotEqual) => {
                    self.current += 1;
                    let right = self.parse_additive()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        operator: Operator::NotEqual,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
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
            if let Token::Operator(Operator::Subtract) = &self.tokens[self.current] {
                self.current += 1;
                let expr = self.parse_primary()?;
                return Ok(Expr::UnaryMinus(Box::new(expr)));
            }
        }
        self.parse_primary()
    }

    /// Parse primary expressions (numbers, parameters, parentheses)
    fn parse_primary(&mut self) -> Result<Expr> {
        if self.current >= self.tokens.len() {
            return Err(Error::validation_error(
                "expression",
                "unexpected end of expression",
            ));
        }

        let token = &self.tokens[self.current].clone();
        self.current += 1;

        match token {
            Token::Number(n) => Ok(Expr::Number(*n)),
            Token::Parameter(name) => Ok(Expr::Parameter(name.clone())),
            Token::Constant(name) => Ok(Expr::Constant(name.clone())),
            Token::Function(name) => {
                // Function call: function_name(arg1, arg2, ...)
                if self.current >= self.tokens.len()
                    || self.tokens[self.current] != Token::LeftParen
                {
                    return Err(Error::validation_error(
                        "expression",
                        "expected '(' after function name",
                    ));
                }
                self.current += 1; // consume '('

                let mut args = Vec::new();

                // Handle empty function calls like sin()
                if self.current < self.tokens.len()
                    && self.tokens[self.current] != Token::RightParen
                {
                    loop {
                        args.push(self.parse_expression()?);

                        if self.current >= self.tokens.len() {
                            return Err(Error::validation_error(
                                "expression",
                                "missing closing parenthesis in function call",
                            ));
                        }

                        match &self.tokens[self.current] {
                            Token::Comma => {
                                self.current += 1; // consume comma
                                continue;
                            }
                            Token::RightParen => break,
                            _ => {
                                return Err(Error::validation_error(
                                    "expression",
                                    "expected ',' or ')' in function call",
                                ))
                            }
                        }
                    }
                }

                if self.current >= self.tokens.len()
                    || self.tokens[self.current] != Token::RightParen
                {
                    return Err(Error::validation_error(
                        "expression",
                        "missing closing parenthesis in function call",
                    ));
                }
                self.current += 1; // consume ')'

                Ok(Expr::FunctionCall {
                    name: name.clone(),
                    args,
                })
            }
            Token::LeftParen => {
                let expr = self.parse_expression()?;
                if self.current >= self.tokens.len()
                    || self.tokens[self.current] != Token::RightParen
                {
                    return Err(Error::validation_error(
                        "expression",
                        "missing closing parenthesis",
                    ));
                }
                self.current += 1;
                Ok(expr)
            }
            _ => Err(Error::validation_error(
                "expression",
                &format!("unexpected token: {:?}", token),
            )),
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
                let param_value = self
                    .parameters
                    .get(name)
                    .ok_or_else(|| Error::parameter_error(name, "parameter not found"))?;

                param_value.parse::<f64>().map_err(|e| {
                    Error::parameter_error(
                        name,
                        &format!("failed to parse '{}': {}", param_value, e),
                    )
                })
            }
            Expr::Constant(name) => match name.as_str() {
                "PI" => Ok(std::f64::consts::PI),
                "E" => Ok(std::f64::consts::E),
                _ => Err(Error::parameter_error(name, "unknown constant")),
            },
            Expr::BinaryOp {
                left,
                operator,
                right,
            } => {
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
                    Operator::Greater => Ok(if left_val > right_val { 1.0 } else { 0.0 }),
                    Operator::Less => Ok(if left_val < right_val { 1.0 } else { 0.0 }),
                    Operator::GreaterEqual => Ok(if left_val >= right_val { 1.0 } else { 0.0 }),
                    Operator::LessEqual => Ok(if left_val <= right_val { 1.0 } else { 0.0 }),
                    Operator::Equal => Ok(if (left_val - right_val).abs() < f64::EPSILON {
                        1.0
                    } else {
                        0.0
                    }),
                    Operator::NotEqual => Ok(if (left_val - right_val).abs() >= f64::EPSILON {
                        1.0
                    } else {
                        0.0
                    }),
                }
            }
            Expr::UnaryMinus(expr) => {
                let val = self.evaluate(expr)?;
                Ok(-val)
            }
            Expr::FunctionCall { name, args } => self.evaluate_function(name, args),
        }
    }

    /// Evaluate a function call
    fn evaluate_function(&self, name: &str, args: &[Expr]) -> Result<f64> {
        match name {
            "sin" => {
                if args.len() != 1 {
                    return Err(Error::parameter_error(
                        name,
                        "sin() requires exactly 1 argument",
                    ));
                }
                let arg = self.evaluate(&args[0])?;
                Ok(arg.sin())
            }
            "cos" => {
                if args.len() != 1 {
                    return Err(Error::parameter_error(
                        name,
                        "cos() requires exactly 1 argument",
                    ));
                }
                let arg = self.evaluate(&args[0])?;
                Ok(arg.cos())
            }
            "tan" => {
                if args.len() != 1 {
                    return Err(Error::parameter_error(
                        name,
                        "tan() requires exactly 1 argument",
                    ));
                }
                let arg = self.evaluate(&args[0])?;
                Ok(arg.tan())
            }
            "sqrt" => {
                if args.len() != 1 {
                    return Err(Error::parameter_error(
                        name,
                        "sqrt() requires exactly 1 argument",
                    ));
                }
                let arg = self.evaluate(&args[0])?;
                if arg < 0.0 {
                    return Err(Error::parameter_error(name, "sqrt() of negative number"));
                }
                Ok(arg.sqrt())
            }
            "abs" => {
                if args.len() != 1 {
                    return Err(Error::parameter_error(
                        name,
                        "abs() requires exactly 1 argument",
                    ));
                }
                let arg = self.evaluate(&args[0])?;
                Ok(arg.abs())
            }
            "floor" => {
                if args.len() != 1 {
                    return Err(Error::parameter_error(
                        name,
                        "floor() requires exactly 1 argument",
                    ));
                }
                let arg = self.evaluate(&args[0])?;
                Ok(arg.floor())
            }
            "ceil" => {
                if args.len() != 1 {
                    return Err(Error::parameter_error(
                        name,
                        "ceil() requires exactly 1 argument",
                    ));
                }
                let arg = self.evaluate(&args[0])?;
                Ok(arg.ceil())
            }
            "min" => {
                if args.len() != 2 {
                    return Err(Error::parameter_error(
                        name,
                        "min() requires exactly 2 arguments",
                    ));
                }
                let arg1 = self.evaluate(&args[0])?;
                let arg2 = self.evaluate(&args[1])?;
                Ok(arg1.min(arg2))
            }
            "max" => {
                if args.len() != 2 {
                    return Err(Error::parameter_error(
                        name,
                        "max() requires exactly 2 arguments",
                    ));
                }
                let arg1 = self.evaluate(&args[0])?;
                let arg2 = self.evaluate(&args[1])?;
                Ok(arg1.max(arg2))
            }
            _ => Err(Error::parameter_error(name, "unknown function")),
        }
    }
}

/// Parse and evaluate an OpenSCENARIO expression
pub fn evaluate_expression<T>(expr: &str, params: &HashMap<String, String>) -> Result<T>
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
        Error::parameter_error(
            expr,
            &format!("failed to parse result '{}': {}", result_str, e),
        )
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
        assert_eq!(
            tokens,
            vec![
                Token::Operator(Operator::Add),
                Token::Operator(Operator::Subtract),
                Token::Operator(Operator::Multiply),
                Token::Operator(Operator::Divide),
                Token::Operator(Operator::Modulo),
            ]
        );
    }

    #[test]
    fn test_tokenize_parameters() {
        let tokens = ExpressionParser::tokenize("${speed} + $velocity").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Parameter("speed".to_string()),
                Token::Operator(Operator::Add),
                Token::Parameter("velocity".to_string()),
            ]
        );
    }

    #[test]
    fn test_parse_simple_expression() {
        let mut parser = ExpressionParser::new("2 + 3").unwrap();
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp {
                left,
                operator,
                right,
            } => {
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
            Expr::BinaryOp {
                left,
                operator,
                right,
            } => {
                assert_eq!(*left, Expr::Number(2.0));
                assert_eq!(operator, Operator::Add);
                match *right {
                    Expr::BinaryOp {
                        left,
                        operator,
                        right,
                    } => {
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
            Expr::BinaryOp {
                left,
                operator,
                right,
            } => {
                assert_eq!(operator, Operator::Multiply);
                assert_eq!(*right, Expr::Number(4.0));
                match *left {
                    Expr::BinaryOp {
                        left,
                        operator,
                        right,
                    } => {
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
        let result: f64 =
            evaluate_expression("${a} * (${b} + ${c}) - ${b} / ${c}", &params).unwrap();
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

    #[test]
    fn test_mathematical_constants() {
        let params = HashMap::new();

        // Test PI constant
        let result: f64 = evaluate_expression("PI", &params).unwrap();
        assert!((result - std::f64::consts::PI).abs() < f64::EPSILON);

        // Test E constant
        let result: f64 = evaluate_expression("E", &params).unwrap();
        assert!((result - std::f64::consts::E).abs() < f64::EPSILON);

        // Test constants in expressions
        let result: f64 = evaluate_expression("2 * PI", &params).unwrap();
        assert!((result - 2.0 * std::f64::consts::PI).abs() < f64::EPSILON);
    }

    #[test]
    fn test_trigonometric_functions() {
        let params = HashMap::new();

        // Test sin function
        let result: f64 = evaluate_expression("sin(0)", &params).unwrap();
        assert!((result - 0.0).abs() < f64::EPSILON);

        let result: f64 = evaluate_expression("sin(PI / 2)", &params).unwrap();
        assert!((result - 1.0).abs() < 1e-10);

        // Test cos function
        let result: f64 = evaluate_expression("cos(0)", &params).unwrap();
        assert!((result - 1.0).abs() < f64::EPSILON);

        let result: f64 = evaluate_expression("cos(PI)", &params).unwrap();
        assert!((result - (-1.0)).abs() < 1e-10);

        // Test tan function
        let result: f64 = evaluate_expression("tan(0)", &params).unwrap();
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_mathematical_functions() {
        let params = HashMap::new();

        // Test sqrt function
        let result: f64 = evaluate_expression("sqrt(4)", &params).unwrap();
        assert_eq!(result, 2.0);

        let result: f64 = evaluate_expression("sqrt(9)", &params).unwrap();
        assert_eq!(result, 3.0);

        // Test abs function
        let result: f64 = evaluate_expression("abs(-5)", &params).unwrap();
        assert_eq!(result, 5.0);

        let result: f64 = evaluate_expression("abs(3.14)", &params).unwrap();
        assert_eq!(result, 3.14);

        // Test floor function
        let result: f64 = evaluate_expression("floor(3.7)", &params).unwrap();
        assert_eq!(result, 3.0);

        let result: f64 = evaluate_expression("floor(-2.3)", &params).unwrap();
        assert_eq!(result, -3.0);

        // Test ceil function
        let result: f64 = evaluate_expression("ceil(3.2)", &params).unwrap();
        assert_eq!(result, 4.0);

        let result: f64 = evaluate_expression("ceil(-2.7)", &params).unwrap();
        assert_eq!(result, -2.0);
    }

    #[test]
    fn test_min_max_functions() {
        let params = HashMap::new();

        // Test min function
        let result: f64 = evaluate_expression("min(5, 3)", &params).unwrap();
        assert_eq!(result, 3.0);

        let result: f64 = evaluate_expression("min(-2, 1)", &params).unwrap();
        assert_eq!(result, -2.0);

        // Test max function
        let result: f64 = evaluate_expression("max(5, 3)", &params).unwrap();
        assert_eq!(result, 5.0);

        let result: f64 = evaluate_expression("max(-2, 1)", &params).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_comparison_operators() {
        let params = HashMap::new();

        // Test greater than
        let result: f64 = evaluate_expression("5 > 3", &params).unwrap();
        assert_eq!(result, 1.0);

        let result: f64 = evaluate_expression("2 > 5", &params).unwrap();
        assert_eq!(result, 0.0);

        // Test less than
        let result: f64 = evaluate_expression("3 < 5", &params).unwrap();
        assert_eq!(result, 1.0);

        let result: f64 = evaluate_expression("5 < 3", &params).unwrap();
        assert_eq!(result, 0.0);

        // Test greater than or equal
        let result: f64 = evaluate_expression("5 >= 5", &params).unwrap();
        assert_eq!(result, 1.0);

        let result: f64 = evaluate_expression("5 >= 3", &params).unwrap();
        assert_eq!(result, 1.0);

        let result: f64 = evaluate_expression("3 >= 5", &params).unwrap();
        assert_eq!(result, 0.0);

        // Test less than or equal
        let result: f64 = evaluate_expression("3 <= 3", &params).unwrap();
        assert_eq!(result, 1.0);

        let result: f64 = evaluate_expression("3 <= 5", &params).unwrap();
        assert_eq!(result, 1.0);

        let result: f64 = evaluate_expression("5 <= 3", &params).unwrap();
        assert_eq!(result, 0.0);

        // Test equality
        let result: f64 = evaluate_expression("5 == 5", &params).unwrap();
        assert_eq!(result, 1.0);

        let result: f64 = evaluate_expression("5 == 3", &params).unwrap();
        assert_eq!(result, 0.0);

        // Test inequality
        let result: f64 = evaluate_expression("5 != 3", &params).unwrap();
        assert_eq!(result, 1.0);

        let result: f64 = evaluate_expression("5 != 5", &params).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_complex_expressions_with_functions() {
        let mut params = HashMap::new();
        params.insert("angle".to_string(), "0.5".to_string());
        params.insert("radius".to_string(), "10.0".to_string());

        // Test complex trigonometric expression: radius * sin(angle)
        let result: f64 = evaluate_expression("${radius} * sin(${angle})", &params).unwrap();
        assert!((result - 10.0 * 0.5_f64.sin()).abs() < 1e-10);

        // Test with constants: 2 * PI * radius
        let result: f64 = evaluate_expression("2 * PI * ${radius}", &params).unwrap();
        assert!((result - 2.0 * std::f64::consts::PI * 10.0).abs() < 1e-10);

        // Test nested functions: sqrt(abs(-16))
        let result: f64 = evaluate_expression("sqrt(abs(-16))", &params).unwrap();
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_function_error_handling() {
        let params = HashMap::new();

        // Test wrong number of arguments
        assert!(evaluate_expression::<f64>("sin(1, 2)", &params).is_err());
        assert!(evaluate_expression::<f64>("sqrt()", &params).is_err());
        assert!(evaluate_expression::<f64>("min(5)", &params).is_err());
        assert!(evaluate_expression::<f64>("max(1, 2, 3)", &params).is_err());

        // Test unknown function
        assert!(evaluate_expression::<f64>("unknown_func(5)", &params).is_err());

        // Test sqrt of negative number
        assert!(evaluate_expression::<f64>("sqrt(-1)", &params).is_err());

        // Test unknown constant
        assert!(evaluate_expression::<f64>("UNKNOWN_CONSTANT", &params).is_err());
    }

    #[test]
    fn test_complex_automotive_scenarios() {
        let mut params = HashMap::new();
        params.insert("current_speed".to_string(), "50.0".to_string());
        params.insert("target_speed".to_string(), "30.0".to_string());
        params.insert("deceleration".to_string(), "3.0".to_string());
        params.insert("reaction_time".to_string(), "1.5".to_string());

        // Test braking distance calculation: speed^2 / (2 * deceleration)
        // Note: Using multiplication instead of exponentiation for now
        let result: f64 = evaluate_expression(
            "(${current_speed} * ${current_speed}) / (2 * ${deceleration})",
            &params,
        )
        .unwrap();
        assert!((result - (50.0 * 50.0) / (2.0 * 3.0)).abs() < 1e-10);

        // Test speed comparison: current_speed > target_speed
        let result: f64 =
            evaluate_expression("${current_speed} > ${target_speed}", &params).unwrap();
        assert_eq!(result, 1.0);

        // Test time-based calculation with functions
        let result: f64 = evaluate_expression(
            "max(${reaction_time}, min(5.0, abs(${current_speed} - ${target_speed}) / 10.0))",
            &params,
        )
        .unwrap();
        assert_eq!(result, 2.0); // max(1.5, min(5.0, 20.0 / 10.0)) = max(1.5, 2.0) = 2.0
    }
}
