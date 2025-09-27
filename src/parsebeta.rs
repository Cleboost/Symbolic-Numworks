#![no_std]

use heapless::{String, Vec, LinearMap};
use core::fmt::Write;

#[derive(Clone, Debug)]
struct Monomial {
    coeff: i64,
    powers: LinearMap<char, u8, 8>,
}

#[derive(Clone, Debug)]
struct Polynomial {
    terms: Vec<Monomial, 16>,
}

impl Polynomial {
    fn from_monomial(coeff: i64, var: Option<char>, exp: u8) -> Self {
        let mut powers = LinearMap::new();
        if let Some(v) = var {
            powers.insert(v, exp).ok();
        }
        let mono = Monomial { coeff, powers };
        let mut terms = Vec::new();
        terms.push(mono).ok();
        Polynomial { terms }
    }

    fn add(&self, other: &Polynomial) -> Polynomial {
        let mut result = self.clone();
        'outer: for m2 in other.terms.iter() {
            for m1 in result.terms.iter_mut() {
                if m1.powers == m2.powers {
                    m1.coeff = match m1.coeff.checked_add(m2.coeff) {
                        Some(result) => result,
                        None => {
                            if m1.coeff > 0 && m2.coeff > 0 {
                                i64::MAX
                            } else if m1.coeff < 0 && m2.coeff < 0 {
                                i64::MIN
                            } else {
                                m1.coeff + m2.coeff
                            }
                        }
                    };
                    continue 'outer;
                }
            }
            result.terms.push(m2.clone()).ok();
        }
        result.terms.retain(|m| m.coeff != 0);
        result
    }

    fn mul(&self, other: &Polynomial) -> Polynomial {
        let mut result = Polynomial { terms: Vec::new() };
        for m1 in self.terms.iter() {
            for m2 in other.terms.iter() {
                let mut powers = m1.powers.clone();
                for (v, e) in m2.powers.iter() {
                    let current = powers.get(v).copied().unwrap_or(0);
                    powers.insert(*v, current + *e).ok();
                }
                let coeff = match m1.coeff.checked_mul(m2.coeff) {
                    Some(result) => result,
                    None => {
                        if (m1.coeff > 0 && m2.coeff > 0) || (m1.coeff < 0 && m2.coeff < 0) {
                            i64::MAX
                        } else {
                            i64::MIN
                        }
                    }
                };
                let mono = Monomial { coeff, powers };
                result.terms.push(mono).ok();
            }
        }
        let mut simplified = Polynomial { terms: Vec::new() };
        'outer: for m in result.terms.iter() {
            for s in simplified.terms.iter_mut() {
                if s.powers == m.powers {
                    s.coeff = match s.coeff.checked_add(m.coeff) {
                        Some(result) => result,
                        None => {
                            if s.coeff > 0 && m.coeff > 0 {
                                i64::MAX
                            } else if s.coeff < 0 && m.coeff < 0 {
                                i64::MIN
                            } else {
                                s.coeff + m.coeff
                            }
                        }
                    };
                    continue 'outer;
                }
            }
            simplified.terms.push(m.clone()).ok();
        }
        simplified.terms.retain(|m| m.coeff != 0);
        simplified
    }

    fn to_string(&self) -> String<128> {
        let mut out = String::new();
        for (i, m) in self.terms.iter().enumerate() {
            if i > 0 {
                let _ = out.push('+');
            }
            if m.coeff != 1 || m.powers.is_empty() {
                let _ = write!(out, "{}", m.coeff);
            }
            for (v, e) in m.powers.iter() {
                let _ = out.push(*v);
                if *e > 1 {
                    let _ = out.push('^');
                    let _ = write!(out, "{}", e);
                }
            }
        }
        out
    }
}

fn parse_expression(s: &str) -> Polynomial {
    let mut parser = Parser::new(s);
    parser.parse_expr()
}

struct Parser<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(s: &'a str) -> Self {
        Self { input: s.as_bytes(), pos: 0 }
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.pos).copied()
    }

    fn eat(&mut self) -> Option<u8> {
        let c = self.peek()?;
        self.pos += 1;
        Some(c)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_ascii_whitespace() {
                self.eat();
            } else {
                break;
            }
        }
    }

    fn parse_expr(&mut self) -> Polynomial {
        self.skip_whitespace();
        let mut result = self.parse_term();
        
        while let Some(c) = self.peek() {
            if c == b'+' {
                self.eat();
                self.skip_whitespace();
                let term = self.parse_term();
                result = result.add(&term);
            } else if c == b'-' {
                self.eat();
                self.skip_whitespace();
                let term = self.parse_term();
                let neg_term = term.mul(&Polynomial::from_monomial(-1, None, 1));
                result = result.add(&neg_term);
            } else {
                break;
            }
        }
        result
    }

    fn parse_term(&mut self) -> Polynomial {
        self.skip_whitespace();
        let mut result = self.parse_factor();
        
        while let Some(c) = self.peek() {
            if c == b'*' {
                self.eat();
                self.skip_whitespace();
                let factor = self.parse_factor();
                result = result.mul(&factor);
            } else if c.is_ascii_alphanumeric() || c == b'(' {
                let factor = self.parse_factor();
                result = result.mul(&factor);
            } else {
                break;
            }
        }
        result
    }

    fn parse_factor(&mut self) -> Polynomial {
        self.skip_whitespace();
        
        if let Some(c) = self.peek() {
            if c == b'(' {
                self.eat();
                let result = self.parse_expr();
                self.skip_whitespace();
                if let Some(b')') = self.peek() {
                    self.eat();
                }
                result
            } else if c.is_ascii_digit() {
                self.parse_number()
            } else if c.is_ascii_alphabetic() {
                self.parse_variable()
            } else {
                Polynomial::from_monomial(1, None, 1)
            }
        } else {
            Polynomial::from_monomial(1, None, 1)
        }
    }

    fn parse_number(&mut self) -> Polynomial {
        let mut val = 0i64;
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                val = val * 10 + (c - b'0') as i64;
                self.eat();
            } else {
                break;
            }
        }
        
        if let Some(b'^') = self.peek() {
            self.eat();
            let mut exp = 0u8;
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    exp = exp * 10 + (c - b'0') as u8;
                    self.eat();
                } else {
                    break;
                }
            }
            let result = val.pow(exp as u32);
            Polynomial::from_monomial(result, None, 1)
        } else {
            Polynomial::from_monomial(val, None, 1)
        }
    }

    fn parse_variable(&mut self) -> Polynomial {
        if let Some(c) = self.eat() {
            let var = c as char;
            let mut exp = 1u8;
            
            if let Some(next_c) = self.peek() {
                let next_char = next_c as char;
                exp = match next_char {
                    '²' => { self.eat(); 2 },
                    '³' => { self.eat(); 3 },
                    '⁴' => { self.eat(); 4 },
                    '⁵' => { self.eat(); 5 },
                    '⁶' => { self.eat(); 6 },
                    '⁷' => { self.eat(); 7 },
                    '⁸' => { self.eat(); 8 },
                    '⁹' => { self.eat(); 9 },
                    _ => {
                        if next_c == b'^' {
                            self.eat();
                            let mut val = 0u8;
                            while let Some(c) = self.peek() {
                                if c.is_ascii_digit() {
                                    val = val * 10 + (c - b'0') as u8;
                                    self.eat();
                                } else {
                                    break;
                                }
                            }
                            val
                        } else {
                            1
                        }
                    }
                };
            }
            
            Polynomial::from_monomial(1, Some(var), exp)
        } else {
            Polynomial::from_monomial(1, None, 1)
        }
    }
}

pub fn simplify(input: &str) -> Result<String<128>, &'static str> {
    let poly = parse_expression(input);
    Ok(poly.to_string())
}
