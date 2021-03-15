mod checked {
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    // Промежуточная функция
    fn op_(x: f64, y: f64) -> MathResult {
        // Если `div` "упадёт", тогда будет "возвращено" `DivisionByZero`
        let ratio = div(x, y)?;

        // если `ln` "упадёт", тогда будет "возвращено" `NonPositiveLogarithm`
        let ln = ln(ratio)?;

        sqrt(ln)
    }

    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Err(why) => panic!(match why {
                MathError::NonPositiveLogarithm => "логарифм не положительного числа",
                MathError::DivisionByZero => "деление на ноль",
                MathError::NegativeSquareRoot => "квадратный корень от отрицательного числа",
            }),
            Ok(value) => println!("{}", value),
        }
    }
}

fn main() {
    checked::op(1.0, 10.0);
}
