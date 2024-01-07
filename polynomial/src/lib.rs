//                                                       |@.......@.......|
//   lib.rs                                              |................|
//                                                       |.ELF............|
//   By: 0xSpyC <rtazlaou@student.42mulhouse.fr>         |..>..... .......|
//                                                       |@........4......|
//   Created: 2024/01/07 18:14:12 by 0xSpyC              |....@.8...@.....|
//   Updated: 2024/01/07 18:19:33 by 0xSpyC              |........@.......|

extern crate regex;

use logos::Logos;
use num::Integer;

trait Int: Integer {}

impl Int for i64 {}

#[derive(Debug, PartialEq)]
pub struct Polynomial {
    pub terms: Vec<(f64, u32)>,
}

impl Polynomial {
    pub fn new() -> Self {
        Polynomial { terms: Vec::new() }
    }

    pub fn from_str(raw_input: &str) -> Result<Self, ParsingError> {
        let input: &str = &raw_input.replace(" ", "");
        let mut terms: Vec<(f64, u32)> = Vec::new();
        let mut sign: u8 = 0;
        let tokens: logos::Lexer<'_, Token> = tokenize(input);
        for token in tokens {
            match token {
                Ok(Token::Term(n)) => match is_valid_term(n, sign) {
                    Ok(term) => terms.push(term),
                    Err(e) => return Err(e),
                },
                Ok(Token::Equal) => {
                    sign += 1;
                    if sign > 1 {
                        return Err(ParsingError::TooManyEquals);
                    }
                }
                Err(()) => return Err(ParsingError::Polynome),
            }
        }

        Ok(Polynomial { terms })
    }

    pub fn reduce(&mut self) {
        let mut reduced_terms: Vec<(f64, u32)> = Vec::new();

        for &(coefficient, exponent) in &self.terms {
            let mut found = false;

            for &mut (ref mut reduced_coefficient, ref mut reduced_exponent) in &mut reduced_terms {
                if *reduced_exponent == exponent {
                    *reduced_coefficient += coefficient;
                    found = true;
                    break;
                }
            }

            if !found {
                reduced_terms.push((coefficient, exponent));
            }
        }

        self.terms = reduced_terms;
    }

    pub fn solve(&self) -> Result<(), PolynomeError> {
        let degree: u32 = self.highest_exponent().unwrap_or(0);

        match degree {
            0 => Err(PolynomeError::PolynomeNull),
            1 => self.solve_linear(),
            2 => self.solve_quadratic(),
            _ => Err(PolynomeError::PolynomeDegree),
        }
    }

    fn solve_linear(&self) -> Result<(), PolynomeError> {
        let a: f64 = self.coefficient_for_exponent(1).unwrap_or(0.0);
        let b: f64 = self.coefficient_for_exponent(0).unwrap_or(0.0);
        let solution: f64 = -b / a;
        //display explanation
        println!("Solving Linear Equation : {} * X + {} = 0", a, b);
        println!("                          {} * X = -{}", a, b);
        println!("                              X = -{} / {}", b, a);
        println!("                              X = {}\n", solution);
        if a == (a as i32) as f64 && b == (b as i32) as f64 {
            let (num, den): (i64, i64) = minimal_fraction(b as i64, a as i64);
            if num.abs() < 11 && den.abs() < 11 {
                println!("Fractional form of {}: {}/{}", solution, -num, den);
            }
        }
        Ok(())
    }

    fn solve_quadratic(&self) -> Result<(), PolynomeError> {
        let a: f64 = self.coefficient_for_exponent(2).unwrap_or(0.0);
        let b: f64 = self.coefficient_for_exponent(1).unwrap_or(0.0);
        let c: f64 = self.coefficient_for_exponent(0).unwrap_or(0.0);

        println!(
            "Solving Quadratic Equation : {} * X^2 + {} * X + {} = 0\n",
            a, b, c
        );

        let discriminant: f64 = b * b - 4.0 * a * c;
        //display explanation
        println!("Delta Calculation :\n");
        println!("Δ = B^2 - 4 * A * C");
        println!("Δ = {}^2 - 4 * {} * {}", b, a, c);
        println!("Δ = {}\n", discriminant);

        if discriminant == 0.0 {
            let solution: f64 = -b / (2.0 * a);
            //display explanation
            println!("Null Delta Admit only 1 real solution");
            println!("X = -B / 2 * A");
            println!("X = -{} / 2 * {}", b, a);
            println!("X = {}", solution);
            let fract: f64 = solution * 100.0;
            if fract == (fract as i32) as f64 {
                let (num, den): (i64, i64) = minimal_fraction(fract as i64, 100);
                if num.abs() < 11 && den.abs() < 11 {
                    println!("Fractional form of {}: {}/{}", solution, -num, den);
                }
            }
            Ok(())
        } else if discriminant > 0.0 {
            let sqrt_discriminant: f64 = discriminant.sqrt();
            let solution1: f64 = (-b + sqrt_discriminant) / (2.0 * a);
            let solution2: f64 = (-b - sqrt_discriminant) / (2.0 * a);
            //display explanation
            println!("Positive Admit 2 real solution");
            println!(" √Δ = {}", sqrt_discriminant);
            println!("X1 = (-B + √Δ)/ 2 * A");
            println!("X2 = (-B - √Δ)/ 2 * A");
            println!("\n");
            println!("X1 = (-{} + √Δ)/ 2 * {}", b, a);
            println!("X1 = {}", solution1);
            println!("X2 = (-{} - √Δ)/ 2 * {}", b, a);
            println!("X2 = {}", solution2);
            let fract1: f64 = solution1 * 100.0;
            if fract1 == (fract1 as i32) as f64 {
                let (num, den): (i64, i64) = minimal_fraction(fract1 as i64, 100);
                if num.abs() < 11 && den.abs() < 11 {
                    println!("Fractional form of {}: {}/{}", solution1, -num, den);
                }
            }
            let fract2: f64 = solution2 * 100.0;
            if fract2 == (fract2 as i32) as f64 {
                let (num, den): (i64, i64) = minimal_fraction(fract2 as i64, 100);
                if num.abs() < 11 && den.abs() < 11 {
                    println!("Fractional form of {}: {}/{}", solution2, -num, den);
                }
            }
            Ok(())
        } else {
            let real_part: f64 = -b / (2.0 * a);
            let imaginary_part: f64 = discriminant.abs().sqrt() / (2.0 * a);
            //display explanation
            println!("Negative Delta Admit 2 complex solution\n");

            println!(
                "Complex solution : Z = X + iY => X is the Real Part, Y is the Imaginary Part\n"
            );
            println!("Z1 = X + iY");
            println!("Z2 = X - iY\n");
            println!("X = -B / 2 * A");
            println!("X = -{} / 2 * {}", b, a);
            println!("X = {}\n", real_part);
            println!("Y = √|Δ| / 2 * A");
            println!("Y = -{} / 2 * {}", discriminant.abs().sqrt(), a);
            println!("Y = {}\n", imaginary_part);
            println!(
                "Solution 1: Z1 = {} + i{}\nSolution 2: Z2 = {} - i{}",
                real_part, imaginary_part, real_part, imaginary_part
            );
            Ok(())
        }
    }

    pub fn highest_exponent(&self) -> Option<u32> {
        let mut max_exponent: Option<u32> = None;

        for &(coefficient, exponent) in &self.terms {
            match max_exponent {
                Some(max) if exponent > max && coefficient != 0.0 => max_exponent = Some(exponent),
                None if coefficient != 0.0 => max_exponent = Some(exponent),
                _ => {}
            }
        }
        if max_exponent == None {
            return Some(0);
        }
        max_exponent
    }
    fn coefficient_for_exponent(&self, exponent: u32) -> Option<f64> {
        for &(coefficient, exp) in &self.terms {
            if exp == exponent {
                return Some(coefficient);
            }
        }
        None
    }
    pub fn display(&self) {
        print!("\nReduced Polynomial : ");

        let mut polynomial_display: Vec<String> = Vec::new();

        for &(coefficient, exponent) in &self.terms {
            if coefficient != 0.0 {
                if exponent == 0 {
                    polynomial_display.push(format!("{}", coefficient));
                } else if coefficient == 1.0 {
                    polynomial_display.push(format!("X^{}", exponent));
                } else {
                    polynomial_display.push(format!("{}X^{}", coefficient, exponent));
                }
            }
        }

        let polynomial_string: String = polynomial_display.join(" + ");

        println!("{} = 0\n", polynomial_string);
        println!("Polynomial Degree: {}\n", self.highest_exponent().unwrap());
    }
}

#[derive(Debug, PartialEq)]
pub enum PolynomeError {
    PolynomeNull,
    PolynomeDegree,
}

impl PolynomeError {
    pub fn message(&self) -> String {
        match self {
            PolynomeError::PolynomeNull => "Polynomial is a constant, no solutions.".to_owned(),
            PolynomeError::PolynomeDegree => {
                "Polynomial degree is higher than 2, cannot solve.".to_owned()
            }
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum ParsingError {
    Polynome,
    TooManyEquals,
    TooManyVariables,
    TooManyExponents,
    TooManyMultiplicationSigns,
    Number,
    Coefficient,
    Exponent,
    Character,
    ExponentWithoutVariable,
    MissingExponent,
    ExponentPosition,
    EmptyExponent,
}

impl ParsingError {
    pub fn message(&self) -> String {
        match self {
            ParsingError::Polynome => "Invalid Polynome Term Must Be An * X^n".to_owned(),
            ParsingError::TooManyEquals => "Too Many Equals".to_owned(),
            ParsingError::TooManyVariables => "Too Many Variables".to_owned(),
            ParsingError::TooManyExponents => "Too Many Exponents".to_owned(),
            ParsingError::TooManyMultiplicationSigns => "Too Many Multiplication Signs".to_owned(),
            ParsingError::Number => "Invalid Integer Found".to_owned(),
            ParsingError::Coefficient => "Invalid Coefficient Found".to_owned(),
            ParsingError::Exponent => "Invalid Coefficient Found".to_owned(),
            ParsingError::Character => "Invalid Character Found".to_owned(),
            ParsingError::ExponentWithoutVariable => "Invalid Exponent Without Variable".to_owned(),
            ParsingError::MissingExponent => "Missing Exponent Character".to_owned(),
            ParsingError::ExponentPosition => "Invalid Exponent Position".to_owned(),
            ParsingError::EmptyExponent => "Invalid Value After Exponent".to_owned(),
        }
    }
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[regex("([+-]?[^-=+]+)", priority = 2, callback = |lex| lex.slice().to_owned() )]
    Term(String),

    #[regex("([=])")]
    Equal,
}

pub fn tokenize(input: &str) -> logos::Lexer<'_, Token> {
    Token::lexer(input)
}

pub fn is_valid_term(term: String, sign: u8) -> Result<(f64, u32), ParsingError> {
    let coefficient: f64;
    let mut exponent: u32 = 0;
    let variable: bool;

    if term.matches(&['x', 'X']).count() > 1 {
        return Err(ParsingError::TooManyVariables);
    }
    if term.matches('^').count() > 1 {
        return Err(ParsingError::TooManyExponents);
    }
    if term.matches('*').count() > 1 {
        return Err(ParsingError::TooManyMultiplicationSigns);
    }
    if contains_invalid_characters(term.clone()) {
        return Err(ParsingError::Character);
    }

    variable = term.matches(&['x', 'X']).count() == 1; // Check 'x' or 'X' is present in string one time strict

    if variable {
        let (mut left_part, mut right_part): (String, String) =
            split_case_insensitive(term.clone());
        let left_valid = remove_multiplication_sign(left_part.clone());

        if left_part.matches('^').count() > 0 {
            return Err(ParsingError::ExponentPosition);
        }

        if left_valid == None {
            return Err(ParsingError::Character);
        }
        left_part = left_valid.unwrap();

        if left_part.is_empty() {
            coefficient = 1.0;
        } else {
            let part_sign: f64 = get_sign(left_part.clone());

            if left_part.len() == 1 && part_sign != 0.0 {
                coefficient = part_sign;
            } else {
                match left_part.parse::<f64>() {
                    Ok(parsed_value) => coefficient = parsed_value,
                    Err(_) => return Err(ParsingError::Coefficient),
                }
            }
        }

        if right_part.matches('*').count() > 0 {
            return Err(ParsingError::Character);
        }

        if right_part.matches('^').count() > 0 {
            if right_part.len() == 1 {
                return Err(ParsingError::EmptyExponent);
            } else {
                if right_part.find('^').unwrap() != 0 {
                    return Err(ParsingError::ExponentPosition);
                } else {
                    right_part.replace_range(..1, "");
                    match right_part.parse::<u32>() {
                        Ok(parsed_value) => exponent = parsed_value,
                        Err(_) => return Err(ParsingError::Exponent),
                    }
                }
            }
        } else {
            if right_part.is_empty() {
                exponent = 1;
            } else {
                return Err(ParsingError::MissingExponent);
            }
        }
    } else {
        if term.matches('^').count() > 0 {
            return Err(ParsingError::ExponentWithoutVariable);
        } else {
            let part_sign = get_sign(term.clone());

            if part_sign != 0.0 && term.len() == 1 {
                println!("{}", term.len());
                return Err(ParsingError::Coefficient);
            } else {
                match term.parse::<f64>() {
                    Ok(parsed_value) => coefficient = parsed_value,
                    Err(_) => return Err(ParsingError::Coefficient),
                }
            }
        }
    }
    if sign == 1 {
        return Ok((coefficient * -1.0, exponent));
    }
    Ok((coefficient, exponent))
}

fn split_case_insensitive(input: String) -> (String, String) {
    let lowercase_input: String = input.to_lowercase();
    let parts: Vec<&str> = lowercase_input.split('x').collect();

    let left_part: String = parts[0].to_string();
    let right_part: String = parts[1].to_string();
    (left_part, right_part)
}

fn contains_invalid_characters(input: String) -> bool {
    for c in input.chars() {
        match c {
            '0'..='9' | '+' | '-' | '*' | '^' | '.' | 'x' | 'X' | ',' => {}
            _ => return true, // Found Character
        }
    }
    false // No Character in String
}
fn get_sign(input: String) -> f64 {
    for c in input.chars() {
        match c {
            '+' => return 1.0,
            '-' => return -1.0,
            _ => {}
        }
    }
    0.0
}

fn remove_multiplication_sign(input: String) -> Option<String> {
    if let Some(index) = input.rfind('*') {
        let end_string = input[index + 1..].trim_start().to_string();
        let substring = input[..index].trim_end().to_string();
        if end_string.chars().all(char::is_whitespace) {
            Some(substring)
        } else {
            None
        }
    } else {
        Some(input)
    }
}

fn minimal_fraction(numerator: i64, denominator: i64) -> (i64, i64) {
    let gcd = numerator.gcd(&denominator);
    let simplified_numerator = numerator / gcd;
    let simplified_denominator = denominator / gcd;
    (simplified_numerator, simplified_denominator)
}
