mod checked {
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NegativeLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    fn op(x: f64, y: f64) -> MathResult {
        sqrt(ln(div(x, y)?)?)
    }

    pub fn operator(x: f64, y: f64) {
        match op(x, y) {
            Err(e) => panic!(match e {
                MathError::DivisionByZero => "division by zero",
                MathError::NegativeSquareRoot => "square root of negative number",
                MathError::NegativeLogarithm => "logarithm of negative number"
            }),
            Ok(v) => println!("{}", v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_19_5_1_question_mark() {
        checked::operator(1.0, 1.0);
    }
}