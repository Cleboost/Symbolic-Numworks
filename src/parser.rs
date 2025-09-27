#[cfg(target_os = "none")]
use alloc::string::{String, ToString};
#[cfg(target_os = "none")]
use alloc::boxed::Box;
#[cfg(target_os = "none")]
use alloc::format;

#[cfg(not(target_os = "none"))]
use std::string::{String, ToString};
#[cfg(not(target_os = "none"))]
use std::boxed::Box;
#[cfg(not(target_os = "none"))]
use std::format;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Constant(f64),
    Variable(String),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Pow(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn simplify(self) -> Self {
        let simplified = self.simplify_recursive();
        let combined = simplified.combine_like_terms();
        combined.simplify_globally()
    }
    
    fn simplify_recursive(self) -> Self {
        match self {
            Expression::Add(l, r) => {
                let left = l.simplify_recursive();
                let right = r.simplify_recursive();

                if let (Expression::Constant(cl), Expression::Constant(cr)) = (&left, &right) {
                    return Expression::Constant(cl + cr);
                }
                if let Expression::Constant(0.0) = right {
                    return left;
                }
                if let Expression::Constant(0.0) = left {
                    return right;
                }
                
                if let (Expression::Mul(l1, r1), Expression::Mul(l2, r2)) = (&left, &right) {
                    if r1 == r2 {
                        if let (Expression::Constant(c1), Expression::Constant(c2)) = (l1.as_ref(), l2.as_ref()) {
                            let sum = c1 + c2;
                            if sum == 0.0 {
                                return Expression::Constant(0.0);
                            } else if sum == 1.0 {
                                return *r1.clone();
                            } else {
                                return Expression::Mul(Box::new(Expression::Constant(sum)), r1.clone());
                            }
                        }
                    }
                }
                
                if let (Expression::Mul(l1, r1), Expression::Mul(l2, r2)) = (&left, &right) {
                    if let (Expression::Constant(c1), Expression::Constant(c2)) = (l1.as_ref(), l2.as_ref()) {
                        if let (Expression::Variable(v1), Expression::Variable(v2)) = (r1.as_ref(), r2.as_ref()) {
                            if v1 == v2 {
                                let sum = c1 + c2;
                                if sum == 0.0 {
                                    return Expression::Constant(0.0);
                                } else if sum == 1.0 {
                                    return Expression::Variable(v1.clone());
                                } else {
                                    return Expression::Mul(Box::new(Expression::Constant(sum)), Box::new(Expression::Variable(v1.clone())));
                                }
                            }
                        }
                    }
                }
                
                if left == right {
                    return Expression::Mul(Box::new(Expression::Constant(2.0)), Box::new(left));
                }

                Expression::Add(Box::new(left), Box::new(right))
            }
            Expression::Sub(l, r) => {
                let left = l.simplify_recursive();
                let right = r.simplify_recursive();

                if let (Expression::Constant(cl), Expression::Constant(cr)) = (&left, &right) {
                    return Expression::Constant(cl - cr);
                }
                if let Expression::Constant(0.0) = right {
                    return left;
                }

                if let (Expression::Mul(l1, r1), Expression::Mul(l2, r2)) = (&left, &right) {
                    if r1 == r2 {
                        if let (Expression::Constant(c1), Expression::Constant(c2)) = (l1.as_ref(), l2.as_ref()) {
                            let diff = c1 - c2;
                            if diff == 0.0 {
                                return Expression::Constant(0.0);
                            } else if diff == 1.0 {
                                return *r1.clone();
                            } else {
                                return Expression::Mul(Box::new(Expression::Constant(diff)), r1.clone());
                            }
                        }
                    }
                }
                
                if let (Expression::Mul(l1, r1), Expression::Mul(l2, r2)) = (&left, &right) {
                    if let (Expression::Constant(c1), Expression::Constant(c2)) = (l1.as_ref(), l2.as_ref()) {
                        if let (Expression::Variable(v1), Expression::Variable(v2)) = (r1.as_ref(), r2.as_ref()) {
                            if v1 == v2 {
                                let diff = c1 - c2;
                                if diff == 0.0 {
                                    return Expression::Constant(0.0);
                                } else if diff == 1.0 {
                                    return Expression::Variable(v1.clone());
                                } else {
                                    return Expression::Mul(Box::new(Expression::Constant(diff)), Box::new(Expression::Variable(v1.clone())));
                                }
                            }
                        }
                    }
                }

                Expression::Sub(Box::new(left), Box::new(right))
            }
            Expression::Mul(l, r) => {
                let left = l.simplify_recursive();
                let right = r.simplify_recursive();

                if let (Expression::Constant(cl), Expression::Constant(cr)) = (&left, &right) {
                    return Expression::Constant(cl * cr);
                }
                if let Expression::Constant(0.0) = left {
                    return Expression::Constant(0.0);
                }
                if let Expression::Constant(0.0) = right {
                    return Expression::Constant(0.0);
                }

                if let Expression::Constant(1.0) = left {
                    return right;
                }
                if let Expression::Constant(1.0) = right {
                    return left;
                }

                Expression::Mul(Box::new(left), Box::new(right))
            }
            Expression::Pow(b, e) => {
                let base = b.simplify_recursive();
                let exp = e.simplify_recursive();


                Expression::Pow(Box::new(base), Box::new(exp))
            }
            e @ Expression::Constant(_) | e @ Expression::Variable(_) => e,
        }
    }
    
    fn combine_like_terms(self) -> Self {
        match self {
            Expression::Add(l, r) => {
                let left = l.combine_like_terms();
                let right = r.combine_like_terms();
                
                if let (Expression::Mul(l1, r1), Expression::Mul(l2, r2)) = (&left, &right) {
                    if r1 == r2 {
                        if let (Expression::Constant(c1), Expression::Constant(c2)) = (l1.as_ref(), l2.as_ref()) {
                            let sum = c1 + c2;
                            if sum == 0.0 {
                                return Expression::Constant(0.0);
                            } else if sum == 1.0 {
                                return *r1.clone();
                            } else {
                                return Expression::Mul(Box::new(Expression::Constant(sum)), r1.clone());
                            }
                        }
                    }
                }
                
                if let (Expression::Mul(l1, r1), Expression::Mul(l2, r2)) = (&left, &right) {
                    if let (Expression::Constant(c1), Expression::Constant(c2)) = (l1.as_ref(), l2.as_ref()) {
                        if let (Expression::Variable(v1), Expression::Variable(v2)) = (r1.as_ref(), r2.as_ref()) {
                            if v1 == v2 {
                                let sum = c1 + c2;
                                if sum == 0.0 {
                                    return Expression::Constant(0.0);
                                } else if sum == 1.0 {
                                    return Expression::Variable(v1.clone());
                                } else {
                                    return Expression::Mul(Box::new(Expression::Constant(sum)), Box::new(Expression::Variable(v1.clone())));
                                }
                            }
                        }
                    }
                }
                
                Expression::Add(Box::new(left), Box::new(right))
            }
            Expression::Sub(l, r) => {
                let left = l.combine_like_terms();
                let right = r.combine_like_terms();
                
                if let (Expression::Mul(l1, r1), Expression::Mul(l2, r2)) = (&left, &right) {
                    if r1 == r2 {
                        if let (Expression::Constant(c1), Expression::Constant(c2)) = (l1.as_ref(), l2.as_ref()) {
                            let diff = c1 - c2;
                            if diff == 0.0 {
                                return Expression::Constant(0.0);
                            } else if diff == 1.0 {
                                return *r1.clone();
                            } else {
                                return Expression::Mul(Box::new(Expression::Constant(diff)), r1.clone());
                            }
                        }
                    }
                }
                
                if let (Expression::Mul(l1, r1), Expression::Mul(l2, r2)) = (&left, &right) {
                    if let (Expression::Constant(c1), Expression::Constant(c2)) = (l1.as_ref(), l2.as_ref()) {
                        if let (Expression::Variable(v1), Expression::Variable(v2)) = (r1.as_ref(), r2.as_ref()) {
                            if v1 == v2 {
                                let diff = c1 - c2;
                                if diff == 0.0 {
                                    return Expression::Constant(0.0);
                                } else if diff == 1.0 {
                                    return Expression::Variable(v1.clone());
                                } else {
                                    return Expression::Mul(Box::new(Expression::Constant(diff)), Box::new(Expression::Variable(v1.clone())));
                                }
                            }
                        }
                    }
                }
                
                Expression::Sub(Box::new(left), Box::new(right))
            }
            e => e,
        }
    }
    
    fn simplify_globally(self) -> Self {
        match self {
            Expression::Add(l, r) => {
                let left = l.simplify_globally();
                let right = r.simplify_globally();
                
                if let (Expression::Mul(l1, r1), Expression::Mul(l2, r2)) = (&left, &right) {
                    if r1 == r2 {
                        if let (Expression::Constant(c1), Expression::Constant(c2)) = (l1.as_ref(), l2.as_ref()) {
                            let sum = c1 + c2;
                            if sum == 0.0 {
                                return Expression::Constant(0.0);
                            } else if sum == 1.0 {
                                return *r1.clone();
                            } else {
                                return Expression::Mul(Box::new(Expression::Constant(sum)), r1.clone());
                            }
                        }
                    }
                }
                
                if let (Expression::Mul(l1, r1), Expression::Mul(l2, r2)) = (&left, &right) {
                    if let (Expression::Constant(c1), Expression::Constant(c2)) = (l1.as_ref(), l2.as_ref()) {
                        if let (Expression::Variable(v1), Expression::Variable(v2)) = (r1.as_ref(), r2.as_ref()) {
                            if v1 == v2 {
                                let sum = c1 + c2;
                                if sum == 0.0 {
                                    return Expression::Constant(0.0);
                                } else if sum == 1.0 {
                                    return Expression::Variable(v1.clone());
                                } else {
                                    return Expression::Mul(Box::new(Expression::Constant(sum)), Box::new(Expression::Variable(v1.clone())));
                                }
                            }
                        }
                    }
                }
                
                if let Expression::Sub(rl, rr) = &right {
                    let right_left = rl.clone().simplify_globally();
                    let right_right = rr.clone().simplify_globally();
                    
                    if let (Expression::Mul(l1, r1), Expression::Mul(l2, r2)) = (&left, &right_left) {
                        if r1 == r2 {
                            if let (Expression::Constant(c1), Expression::Constant(c2)) = (l1.as_ref(), l2.as_ref()) {
                                let sum = c1 + c2;
                                if sum == 0.0 {
                                    return Expression::Sub(Box::new(Expression::Constant(0.0)), Box::new(right_right));
                                } else if sum == 1.0 {
                                    return Expression::Sub(Box::new(*r1.clone()), Box::new(right_right));
                                } else {
                                    return Expression::Sub(
                                        Box::new(Expression::Mul(Box::new(Expression::Constant(sum)), r1.clone())),
                                        Box::new(right_right)
                                    );
                                }
                            }
                        }
                    }
                    
                    if let (Expression::Mul(l1, r1), Expression::Mul(l2, r2)) = (&left, &right_left) {
                        if let (Expression::Constant(c1), Expression::Constant(c2)) = (l1.as_ref(), l2.as_ref()) {
                            if let (Expression::Variable(v1), Expression::Variable(v2)) = (r1.as_ref(), r2.as_ref()) {
                                if v1 == v2 {
                                    let sum = c1 + c2;
                                    if sum == 0.0 {
                                        return Expression::Sub(Box::new(Expression::Constant(0.0)), Box::new(right_right));
                                    } else if sum == 1.0 {
                                        return Expression::Sub(Box::new(Expression::Variable(v1.clone())), Box::new(right_right));
                                    } else {
                                        return Expression::Sub(
                                            Box::new(Expression::Mul(Box::new(Expression::Constant(sum)), Box::new(Expression::Variable(v1.clone())))),
                                            Box::new(right_right)
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
                
                Expression::Add(Box::new(left), Box::new(right))
            }
            Expression::Sub(l, r) => {
                let left = l.simplify_globally();
                let right = r.simplify_globally();
                
                if let (Expression::Mul(l1, r1), Expression::Mul(l2, r2)) = (&left, &right) {
                    if r1 == r2 {
                        if let (Expression::Constant(c1), Expression::Constant(c2)) = (l1.as_ref(), l2.as_ref()) {
                            let diff = c1 - c2;
                            if diff == 0.0 {
                                return Expression::Constant(0.0);
                            } else if diff == 1.0 {
                                return *r1.clone();
                            } else {
                                return Expression::Mul(Box::new(Expression::Constant(diff)), r1.clone());
                            }
                        }
                    }
                }
                
                if let (Expression::Mul(l1, r1), Expression::Mul(l2, r2)) = (&left, &right) {
                    if let (Expression::Constant(c1), Expression::Constant(c2)) = (l1.as_ref(), l2.as_ref()) {
                        if let (Expression::Variable(v1), Expression::Variable(v2)) = (r1.as_ref(), r2.as_ref()) {
                            if v1 == v2 {
                                let diff = c1 - c2;
                                if diff == 0.0 {
                                    return Expression::Constant(0.0);
                                } else if diff == 1.0 {
                                    return Expression::Variable(v1.clone());
                                } else {
                                    return Expression::Mul(Box::new(Expression::Constant(diff)), Box::new(Expression::Variable(v1.clone())));
                                }
                            }
                        }
                    }
                }
                
                Expression::Sub(Box::new(left), Box::new(right))
            }
            e => e,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Expression::Constant(c) => {
                if *c == (*c as i32) as f64 {
                    format!("{}", *c as i32)
                } else {
                    c.to_string()
                }
            },
            Expression::Variable(v) => v.clone(),
            Expression::Add(l, r) => {
                let left = l.to_string();
                let right = r.to_string();
                format!("{} + {}", left, right)
            },
            Expression::Sub(l, r) => {
                let left = l.to_string();
                let right = r.to_string();
                format!("{} - {}", left, right)
            },
            Expression::Mul(l, r) => {
                let left = l.to_string();
                let right = r.to_string();
                
                match (l.as_ref(), r.as_ref()) {
                    (Expression::Constant(c), Expression::Variable(v)) => {
                        if *c == 1.0 {
                            v.clone()
                        } else if *c == -1.0 {
                            format!("-{}", v)
                        } else if *c == (*c as i32) as f64 {
                            format!("{}{}", *c as i32, v)
                        } else {
                            format!("{}{}", c, v)
                        }
                    },
                    (Expression::Variable(v), Expression::Constant(c)) => {
                        if *c == 1.0 {
                            v.clone()
                        } else if *c == -1.0 {
                            format!("-{}", v)
                        } else if *c == (*c as i32) as f64 {
                            format!("{}{}", *c as i32, v)
                        } else {
                            format!("{}{}", c, v)
                        }
                    },
                    _ => format!("{} * {}", left, right)
                }
            },
            Expression::Pow(b, e) => {
                let base = b.to_string();
                let exp = e.to_string();
                format!("{}^{}", base, exp)
            },
        }
    }
}

pub struct Parser {
    input: String,
    pos: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    fn next(&mut self) -> Option<char> {
        let ch = self.peek();
        if ch.is_some() {
            self.pos += 1;
        }
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.next();
            } else {
                break;
            }
        }
    }

    fn parse_number(&mut self) -> Result<Expression, String> {
        let start = self.pos;
        let mut has_dot = false;
        
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() || ch == '.' {
                if ch == '.' {
                    if has_dot {
                        return Err("Invalid number format".to_string());
                    }
                    has_dot = true;
                }
                self.next();
            } else {
                break;
            }
        }
        
        let num_str = &self.input[start..self.pos];
        match num_str.parse::<f64>() {
            Ok(num) => Ok(Expression::Constant(num)),
            Err(_) => Err("Invalid number".to_string()),
        }
    }

    fn parse_variable(&mut self) -> Result<Expression, String> {
        let start = self.pos;
        
        while let Some(ch) = self.peek() {
            if ch.is_alphabetic() {
                self.next();
            } else {
                break;
            }
        }
        
        if self.pos > start {
            let var_name = &self.input[start..self.pos];
            Ok(Expression::Variable(var_name.to_string()))
        } else {
            Err("Expected variable".to_string())
        }
    }

    fn parse_factor(&mut self) -> Result<Expression, String> {
        self.skip_whitespace();
        
        match self.peek() {
            Some('(') => {
                self.next();
                let expr = self.parse_expression()?;
                self.skip_whitespace();
                if self.next() != Some(')') {
                    return Err("Expected ')'".to_string());
                }
                Ok(expr)
            }
            Some(ch) if ch.is_ascii_digit() => self.parse_number(),
            Some(ch) if ch.is_alphabetic() => self.parse_variable(),
            _ => Err("Expected factor".to_string()),
        }
    }

    fn parse_power(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_factor()?;
        
        self.skip_whitespace();
        while self.peek() == Some('^') {
            self.next();
            let right = self.parse_factor()?;
            left = Expression::Pow(Box::new(left), Box::new(right));
            self.skip_whitespace();
        }
        
        Ok(left)
    }

    fn parse_multiplication(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_power()?;
        
        self.skip_whitespace();
        loop {
            match self.peek() {
                Some('*') => {
                    self.next();
                    let right = self.parse_power()?;
                    left = Expression::Mul(Box::new(left), Box::new(right));
                }
                Some(ch) if ch.is_alphabetic() => {
                    let right = self.parse_variable()?;
                    left = Expression::Mul(Box::new(left), Box::new(right));
                }
                _ => break,
            }
            self.skip_whitespace();
        }
        
        Ok(left)
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_multiplication()?;
        
        self.skip_whitespace();
        while let Some(op) = self.peek() {
            match op {
                '+' => {
                    self.next();
                    let right = self.parse_multiplication()?;
                    left = Expression::Add(Box::new(left), Box::new(right));
                }
                '-' => {
                    self.next();
                    let right = self.parse_multiplication()?;
                    left = Expression::Sub(Box::new(left), Box::new(right));
                }
                _ => break,
            }
            self.skip_whitespace();
        }
        
        Ok(left)
    }

    pub fn parse(&mut self) -> Result<Expression, String> {
        let result = self.parse_expression()?;
        self.skip_whitespace();
        if self.pos < self.input.len() {
            return Err("Unexpected characters at end".to_string());
        }
        Ok(result)
    }
}

pub fn simplify_expression(input: &str) -> String {
    let mut parser = Parser::new(input);
    match parser.parse() {
        Ok(expr) => {
            let simplified = expr.simplify();
            let result = simplified.to_string();
            
            let cleaned = result.replace("+ -", "- ")
                               .replace("  ", " ")
                               .trim()
                               .to_string();
            
            cleaned
        },
        Err(_) => "Erreur".to_string()
    }
}
