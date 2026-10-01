//! Parsing and evaluation of OpenSCENARIO expressions.
//!
//! The XSD admits `${...}` wherever a value is expected, against the pattern
//! `[$][{][ A-Za-z0-9_\+\-\*/%$\(\)\.,]*[\}]`. This module parses that content and
//! evaluates it once the parameters it names are bound.
//!
//! Operators: `+ - * / %`, parentheses, `> < >= <= == !=`, and the Boolean operators
//! `not`/`and`/`or` (ASAM OpenSCENARIO XML v1.3.0 section 9.2, "Supported Boolean operators
//! (ordered by operator precedence): Negation operator (not), Conjunction operator (and),
//! Disjunction operator (or)").
//! Operands: numeric literals, parameter references, and the constant `pi`. `pi` is defined by
//! OpenSCENARIO XML v1.4.0 section 9.2.2 ("Constants") and is accepted here as a
//! forward-compatible extension to 1.3. Any other bare identifier is an error, because a
//! parameter is referenced as `$name`.
//! Functions: `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `sqrt`, `abs`, `sign`, `floor`,
//! `ceil`, `round`, `pow`, `min`, `max` (section 9.2, "Supported arithmetic operators").
//!
//! Section 9.2's general restrictions require an error, not `NaN` or infinity, wherever
//! [IEEE 754-2019] would produce one; [`evaluate_expression`] enforces this once on the final
//! numeric result, and the functions that can reach a non-finite value from a finite input
//! (`sqrt`, `asin`, `acos`, `pow`) also name the domain violation directly rather than letting
//! the caller see only "not finite".

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
    /// A named constant, already resolved to its value (only `pi` exists).
    Constant(f64),
    LeftParen,
    RightParen,
    Comma,
    Not,
    And,
    Or,
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
    /// A named constant, already resolved to its value (only `pi` exists).
    Constant(f64),
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
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
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
            return Err(Error::parse_error(
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
                        return Err(Error::parse_error(
                            input,
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
                        return Err(Error::parse_error(
                            input,
                            "single '!' not supported, use '!=' for inequality",
                        ));
                    }
                }
                '$' => {
                    // Parameter reference: ${paramName} or $paramName
                    chars.next();
                    if chars.peek() == Some(&'{') {
                        chars.next();
                        let param_name = Self::read_until_char(&mut chars, '}')?;
                        if chars.next() != Some('}') {
                            return Err(Error::parse_error(
                                input,
                                "missing closing brace in parameter reference",
                            ));
                        }
                        tokens.push(Token::Parameter(param_name));
                    } else {
                        // Simple $paramName format (deprecated but supported). read_identifier
                        // itself errors on an empty read (see its own empty check below), so a
                        // second empty check here can never be reached; it was uncovered and
                        // dead.
                        let param_name = Self::read_identifier(&mut chars)?;
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
                    } else if identifier == "not" {
                        tokens.push(Token::Not);
                    } else if identifier == "and" {
                        tokens.push(Token::And);
                    } else if identifier == "or" {
                        tokens.push(Token::Or);
                    } else if identifier == "pi" {
                        tokens.push(Token::Constant(std::f64::consts::PI));
                    } else {
                        return Err(Error::parse_error(
                            input,
                            &format!(
                                "unknown identifier '{identifier}' in expression; parameters are referenced as ${identifier}"
                            ),
                        ));
                    }
                }
                _ => {
                    return Err(Error::parse_error(
                        input,
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

        number_str
            .parse::<f64>()
            .map_err(|_| Error::parse_error(&number_str, "invalid number format"))
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

        if identifier.is_empty() {
            return Err(Error::parse_error(
                "",
                "Expected identifier but found empty string",
            ));
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

    /// Parse an expression with precedence handling. The Boolean operators sit above
    /// comparison in the grammar (lowest precedence first): `or`, then `and`, then `not`,
    /// matching section 9.2's precedence list.
    fn parse_expression(&mut self) -> Result<Expr> {
        self.parse_or()
    }

    /// Parse disjunction (`or`), the lowest-precedence Boolean operator.
    fn parse_or(&mut self) -> Result<Expr> {
        let mut left = self.parse_and()?;

        while self.current < self.tokens.len() && self.tokens[self.current] == Token::Or {
            self.current += 1;
            let right = self.parse_and()?;
            left = Expr::Or(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    /// Parse conjunction (`and`).
    fn parse_and(&mut self) -> Result<Expr> {
        let mut left = self.parse_not()?;

        while self.current < self.tokens.len() && self.tokens[self.current] == Token::And {
            self.current += 1;
            let right = self.parse_not()?;
            left = Expr::And(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    /// Parse negation (`not`), the highest-precedence Boolean operator. `not` is right-
    /// associative like unary minus, so `not not $a` parses as `not (not $a)`.
    fn parse_not(&mut self) -> Result<Expr> {
        if self.current < self.tokens.len() && self.tokens[self.current] == Token::Not {
            self.current += 1;
            let inner = self.parse_not()?;
            return Ok(Expr::Not(Box::new(inner)));
        }
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
            Token::Constant(value) => Ok(Expr::Constant(*value)),
            Token::Function(name) => {
                // Function call: function_name(arg1, arg2, ...)
                // The tokenizer only emits Token::Function when the identifier is
                // immediately followed by '(' (see the tokenizer's identifier arm),
                // and '(' always tokenizes to Token::LeftParen, so the next token
                // here is always LeftParen. No error branch is reachable.
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
            Expr::Constant(value) => Ok(*value),
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
            Expr::Not(_) | Expr::And(..) | Expr::Or(..) => Err(Error::parameter_error(
                "expression",
                "a Boolean expression (not/and/or) cannot be used where a numeric value is expected",
            )),
        }
    }

    /// Evaluate a Boolean expression AST to `true`/`false`.
    ///
    /// This is a separate entry point from [`evaluate`](Self::evaluate) rather than a shared
    /// result type because only `not`/`and`/`or` and the operands that feed them are
    /// Boolean-shaped; every other production in this grammar is numeric. A comparison
    /// (`>`, `==`, ...) is evaluated numerically and its `1.0`/`0.0` result treated as
    /// `true`/`false`, and a bare parameter is looked up as the literal text `"true"` or
    /// `"false"` the way a `Boolean`-typed attribute would write it.
    fn evaluate_bool(&self, expr: &Expr) -> Result<bool> {
        match expr {
            Expr::Not(inner) => Ok(!self.evaluate_bool(inner)?),
            Expr::And(left, right) => Ok(self.evaluate_bool(left)? && self.evaluate_bool(right)?),
            Expr::Or(left, right) => Ok(self.evaluate_bool(left)? || self.evaluate_bool(right)?),
            Expr::Parameter(param_name) => {
                let param_value = self
                    .parameters
                    .get(param_name)
                    .ok_or_else(|| Error::parameter_error(param_name, "parameter not found"))?;
                match param_value.as_str() {
                    "true" => Ok(true),
                    "false" => Ok(false),
                    other => Err(Error::parameter_error(
                        param_name,
                        &format!(
                            "parameter value '{}' is not a Boolean ('true' or 'false')",
                            other
                        ),
                    )),
                }
            }
            Expr::BinaryOp {
                operator:
                    Operator::Greater
                    | Operator::Less
                    | Operator::GreaterEqual
                    | Operator::LessEqual
                    | Operator::Equal
                    | Operator::NotEqual,
                ..
            } => Ok(self.evaluate(expr)? != 0.0),
            _ => Err(Error::parameter_error(
                "expression",
                "expected a Boolean expression (not/and/or, a comparison, or a Boolean parameter)",
            )),
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
            "round" => {
                if args.len() != 1 {
                    return Err(Error::parameter_error(
                        name,
                        "round() requires exactly 1 argument",
                    ));
                }
                let arg = self.evaluate(&args[0])?;
                // Section 9.2 declares `round: double -> int` but does not state a tie-breaking
                // rule, and defers "the definition of the arithmetic operators" to IEEE 754-2019
                // without naming which of that standard's several round-to-integral operations
                // applies. `f64::round` implements round-half-away-from-zero (2.5 -> 3.0,
                // -2.5 -> -3.0), the conventional meaning of "round" absent a stated rule, and is
                // used here for that reason rather than IEEE 754's default ties-to-even.
                Ok(arg.round())
            }
            "asin" => {
                if args.len() != 1 {
                    return Err(Error::parameter_error(
                        name,
                        "asin() requires exactly 1 argument",
                    ));
                }
                let arg = self.evaluate(&args[0])?;
                if !(-1.0..=1.0).contains(&arg) {
                    return Err(Error::parameter_error(
                        name,
                        &format!("asin({}) is undefined outside [-1, 1]", arg),
                    ));
                }
                Ok(arg.asin())
            }
            "acos" => {
                if args.len() != 1 {
                    return Err(Error::parameter_error(
                        name,
                        "acos() requires exactly 1 argument",
                    ));
                }
                let arg = self.evaluate(&args[0])?;
                if !(-1.0..=1.0).contains(&arg) {
                    return Err(Error::parameter_error(
                        name,
                        &format!("acos({}) is undefined outside [-1, 1]", arg),
                    ));
                }
                Ok(arg.acos())
            }
            "atan" => {
                if args.len() != 1 {
                    return Err(Error::parameter_error(
                        name,
                        "atan() requires exactly 1 argument",
                    ));
                }
                let arg = self.evaluate(&args[0])?;
                Ok(arg.atan())
            }
            "sign" => {
                if args.len() != 1 {
                    return Err(Error::parameter_error(
                        name,
                        "sign() requires exactly 1 argument",
                    ));
                }
                let arg = self.evaluate(&args[0])?;
                // Section 9.2: "sign(x) = -1, if x < 0; sign(x) = 0, if x = 0; sign(x) = 1, if
                // x > 0."
                Ok(if arg < 0.0 {
                    -1.0
                } else if arg > 0.0 {
                    1.0
                } else {
                    0.0
                })
            }
            "pow" => {
                if args.len() != 2 {
                    return Err(Error::parameter_error(
                        name,
                        "pow() requires exactly 2 arguments",
                    ));
                }
                let base = self.evaluate(&args[0])?;
                let exponent = self.evaluate(&args[1])?;
                // Section 9.2 gives `pow` no explicit domain, only the general restriction that
                // any operation IEEE 754-2019 defines as NaN or infinite must be an error. A
                // negative base raised to a non-integer exponent is exactly that case (the real
                // result does not exist); name it rather than let the caller see only "not
                // finite".
                if base < 0.0 && exponent.fract() != 0.0 {
                    return Err(Error::parameter_error(
                        name,
                        &format!(
                            "pow({}, {}) is not a real number: a negative base raised to a \
                             non-integer exponent",
                            base, exponent
                        ),
                    ));
                }
                let result = base.powf(exponent);
                if !result.is_finite() {
                    return Err(Error::parameter_error(
                        name,
                        &format!("pow({}, {}) is not finite", base, exponent),
                    ));
                }
                Ok(result)
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

/// Parse and evaluate an OpenSCENARIO expression to text, choosing the numeric or Boolean
/// evaluator by the **target attribute's type**, `target_is_bool`, not by scanning the parsed
/// AST for `not`/`and`/`or`. Section 9.2 makes a Boolean production out of three shapes that a
/// keyword scan cannot tell apart from a numeric one: `not`/`and`/`or`, a comparison
/// (`${$speed > 10}`), and a bare parameter reference (`${$flag}`) — the last two contain no
/// Boolean keyword at all, so a scan sent them down the numeric path, where `"true"` fails to
/// parse as `f64` and a comparison's `1`/`0` fails to parse as `bool` (whose `FromStr` accepts
/// only the literal words `"true"`/`"false"`). Reading the target type off the caller's `T`
/// instead — the same technique the `Double` empty-string check above already uses — resolves
/// unambiguously: an arithmetic expression in a Boolean attribute is still a type error, and a
/// Boolean parameter or comparison in a numeric attribute is still a type error, exactly as
/// section 9.2 requires either way.
///
/// This is the shared core behind [`evaluate_expression`] and `types::basic::resolve_expression`
/// (`src/types/basic.rs`). It is `pub(crate)`, not `pub`, because it returns text rather than a
/// typed result; callers that want a typed result call `evaluate_expression` instead.
pub(crate) fn evaluate_expression_text(
    expr: &str,
    params: &HashMap<String, String>,
    target_is_bool: bool,
) -> Result<String> {
    let mut parser = ExpressionParser::new(expr)?;
    let ast = parser.parse()?;
    let evaluator = ExpressionEvaluator::new(params.clone());

    if target_is_bool {
        Ok(evaluator.evaluate_bool(&ast)?.to_string())
    } else {
        let result = evaluator.evaluate(&ast)?;

        // Section 9.2, "General restrictions": "Because ASAM OpenSCENARIO does not use NaN
        // or infinity, all operations where [IEEE 754-2019] defines the result to be either
        // NaN or infinity shall instead result in an error." The functions most likely to
        // reach a non-finite value from a finite input (`sqrt`, `asin`, `acos`, `pow`)
        // already name the domain violation in `evaluate_function`; this is the backstop for
        // any other combination that reaches NaN or +-infinity, such as an overflowing
        // product.
        if !result.is_finite() {
            return Err(Error::parameter_error(
                expr,
                &format!("expression result {} is not finite", result),
            ));
        }

        Ok(result.to_string())
    }
}

/// Parse and evaluate an OpenSCENARIO expression
pub fn evaluate_expression<T>(expr: &str, params: &HashMap<String, String>) -> Result<T>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let target_is_bool = std::any::type_name::<T>() == "bool";
    let result_str = evaluate_expression_text(expr, params, target_is_bool)?;
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

        // Test: speed % 7, the remaining arithmetic operator of section 9.2
        let result: f64 = evaluate_expression("${speed} % 7", &params).unwrap();
        assert_eq!(result, 2.0);
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

    /// The only named constant is lowercase `pi`; a bare identifier is never a parameter.
    #[test]
    fn test_pi_constant_and_bare_identifiers() {
        let mut params = HashMap::new();
        params.insert("x".to_string(), "3".to_string());
        params.insert("a".to_string(), "2".to_string());
        let pi = std::f64::consts::PI;

        let values = [
            ("$x", 3.0),
            ("pi", pi),
            ("2*pi", 2.0 * pi),
            ("$a*pi", 2.0 * pi),
            ("${a}*pi", 2.0 * pi),
            ("-pi", -pi),
            ("65*pi/180", 65.0 * pi / 180.0),
            ("sin(pi / 2)", 1.0),
        ];
        for (expr, expected) in values {
            let got: f64 = evaluate_expression(expr, &params).expect(expr);
            assert!(
                (got - expected).abs() < 1e-12,
                "{expr}: expected {expected}, got {got}"
            );
        }

        let refusals = [
            (
                "PI",
                "unknown identifier 'PI' in expression; parameters are referenced as $PI",
            ),
            (
                "E",
                "unknown identifier 'E' in expression; parameters are referenced as $E",
            ),
            (
                "x",
                "unknown identifier 'x' in expression; parameters are referenced as $x",
            ),
            ("$a*e", "parameters are referenced as $e"),
        ];
        for (expr, cause) in refusals {
            let err = evaluate_expression::<f64>(expr, &params)
                .expect_err(expr)
                .to_string();
            assert!(err.contains(cause), "{expr}: expected {cause:?}, got {err}");
        }
    }

    #[test]
    fn test_trigonometric_functions() {
        let params = HashMap::new();

        // Test sin function
        let result: f64 = evaluate_expression("sin(0)", &params).unwrap();
        assert!((result - 0.0).abs() < f64::EPSILON);

        let result: f64 = evaluate_expression("sin(pi / 2)", &params).unwrap();
        assert!((result - 1.0).abs() < 1e-10);

        // Test cos function
        let result: f64 = evaluate_expression("cos(0)", &params).unwrap();
        assert!((result - 1.0).abs() < f64::EPSILON);

        let result: f64 = evaluate_expression("cos(pi)", &params).unwrap();
        assert!((result - (-1.0)).abs() < 1e-10);

        // Test tan function
        let result: f64 = evaluate_expression("tan(0)", &params).unwrap();
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    #[allow(clippy::approx_constant)] // 3.14 is a literal test input for abs(), not an approximation of PI
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

        // Test with constants: 2 * pi * radius
        let result: f64 = evaluate_expression("2 * pi * ${radius}", &params).unwrap();
        assert!((result - 2.0 * std::f64::consts::PI * 10.0).abs() < 1e-10);

        // Test nested functions: sqrt(abs(-16))
        let result: f64 = evaluate_expression("sqrt(abs(-16))", &params).unwrap();
        assert_eq!(result, 4.0);
    }

    /// Each malformed call or domain violation is refused with an error naming its cause.
    #[test]
    fn test_function_error_handling() {
        let params = HashMap::new();
        let cases = [
            // Wrong number of arguments
            ("sin(1, 2)", "sin() requires exactly 1 argument"),
            ("cos(1, 2)", "cos() requires exactly 1 argument"),
            ("tan(1, 2)", "tan() requires exactly 1 argument"),
            ("sqrt()", "sqrt() requires exactly 1 argument"),
            ("abs(1, 2)", "abs() requires exactly 1 argument"),
            ("floor(1, 2)", "floor() requires exactly 1 argument"),
            ("ceil(1, 2)", "ceil() requires exactly 1 argument"),
            ("round(1, 2)", "round() requires exactly 1 argument"),
            ("asin(1, 2)", "asin() requires exactly 1 argument"),
            ("acos(1, 2)", "acos() requires exactly 1 argument"),
            ("atan(1, 2)", "atan() requires exactly 1 argument"),
            ("sign(1, 2)", "sign() requires exactly 1 argument"),
            ("pow(2)", "pow() requires exactly 2 arguments"),
            ("min(5)", "min() requires exactly 2 arguments"),
            ("max(1, 2, 3)", "max() requires exactly 2 arguments"),
            // An unknown function name. A bare identifier is covered by
            // `test_pi_constant_and_bare_identifiers`.
            ("unknown_func(5)", "unknown function"),
            // Domain violations
            ("sqrt(-1)", "sqrt() of negative number"),
            ("acos(2)", "acos(2) is undefined outside [-1, 1]"),
            ("5 % 0", "modulo by zero"),
            ("pow(0, -1)", "pow(0, -1) is not finite"),
            // The generic non-finite backstop for a combination that overflows without going
            // through a function that already names its own domain violation.
            ("1e300 * 1e300", "is not finite"),
            // Tokenizer errors: single '=' / '!' (the two-character forms are the only ones
            // section 9.2 supports), an unterminated `${...}` reference, a character outside
            // the grammar, and a bare '$' with nothing that can be read as an identifier after
            // it.
            ("5 = 3", "single '=' not supported, use '==' for equality"),
            ("5 ! 3", "single '!' not supported, use '!=' for inequality"),
            ("${abc", "missing closing brace in parameter reference"),
            ("5 & 3", "unexpected character: '&'"),
            ("$+1", "Expected identifier but found empty string"),
            // Parser errors: an operator with nothing after it, an unterminated function call
            // (both while still reading arguments and with none at all), two arguments with no
            // separator, an unterminated parenthesized expression, and a token (here a leading
            // '+', which -- unlike '-' -- has no unary form) where an operand is expected.
            ("5 +", "unexpected end of expression"),
            ("sin(1", "missing closing parenthesis in function call"),
            ("sin(1 2)", "expected ',' or ')' in function call"),
            ("sin(", "missing closing parenthesis in function call"),
            ("(5", "missing closing parenthesis"),
            ("+5", "unexpected token: Operator(Add)"),
            // A Boolean-shaped expression (its AST root is Not/And/Or) evaluated where a numeric
            // result is required -- `evaluate`, not `evaluate_bool`, is used because the target
            // type here is `f64`.
            (
                "not $flag",
                "a Boolean expression (not/and/or) cannot be used where a numeric value is \
                 expected",
            ),
        ];
        for (expr, cause) in cases {
            let err = evaluate_expression::<f64>(expr, &params)
                .expect_err(expr)
                .to_string();
            assert!(err.contains(cause), "{expr}: expected {cause:?}, got {err}");
        }
    }

    /// `evaluate_bool` (used when the target attribute is `Boolean`) refuses a parameter whose
    /// bound text is not the literal `"true"`/`"false"`, and refuses any expression shape that
    /// is neither `not`/`and`/`or`, a comparison, nor a bare parameter -- an arithmetic
    /// expression is well-formed but not Boolean-shaped.
    #[test]
    fn test_boolean_error_handling() {
        let mut params = HashMap::new();
        params.insert("maybe".to_string(), "perhaps".to_string());

        let err = evaluate_expression::<bool>("$maybe", &params)
            .expect_err("$maybe")
            .to_string();
        assert!(
            err.contains("parameter value 'perhaps' is not a Boolean ('true' or 'false')"),
            "got: {err}"
        );

        let err = evaluate_expression::<bool>("5 + 3", &params)
            .expect_err("5 + 3")
            .to_string();
        assert!(
            err.contains(
                "expected a Boolean expression (not/and/or, a comparison, or a Boolean parameter)"
            ),
            "got: {err}"
        );
    }

    /// `evaluate_expression` names the target type's own parse failure, not just the numeric
    /// evaluator's result, when the evaluated text does not fit `T` -- here `u32`, which (unlike
    /// `f64`) rejects a negative literal.
    #[test]
    fn test_evaluate_expression_reports_target_type_parse_failure() {
        let params = HashMap::new();
        let err = evaluate_expression::<u32>("-1", &params)
            .expect_err("-1")
            .to_string();
        assert!(err.contains("failed to parse result '-1'"), "got: {err}");
    }
}
