mod example_19_5_1_question_mark;

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
}

fn operator(x: f64, y: f64) -> f64 {
    match checked::div(x, y) {
        Err(e) => panic!("{:?}", e),
        Ok(ratio) => match checked::ln(ratio) {
            Err(e) => panic!("{:?}", e),
            Ok(ln) => match checked::sqrt(ln) {
                Err(e) => panic!("{:?}", e),
                Ok(sqrt) => sqrt
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_19_5_result() {
        println!("{:?}", operator(1.0, 1.0));
    }
}