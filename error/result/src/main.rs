use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    // Давайте попробуем использовать `unwrap()` чтобы получить число. Он нас укусит?
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;
    Ok(first_number * second_number)
}

fn main() -> Result<(), ParseIntError> {
    let twenty = multiply("10", "2")?;
    println!("удовоенное {}", twenty);

    let tt = multiply("t", "2")?;
    println!("удвоенное {}", tt);

    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
