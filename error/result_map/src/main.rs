use std::num::ParseIntError;

// Мы используем сопоставление с образцом без `unwrap()` и меняем тип результата.
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => match second_number_str.parse::<i32>() {
            Ok(second_number) => Ok(first_number * second_number),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

// Как и с `Option`, мы можем использовать комбинаторы, как `map()`.
// Эта функция в основном идентична предыдущей и читается как:
// изменяем n при валидном значении, иначе передаём ошибку.
fn multiply2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n равно {}", n),
        Err(e) => println!("Ошибка: {}", e),
    }
}

fn main() {
    // Это даёт разумный ответ.
    let twenty = multiply("10", "2");
    print(twenty);

    // Следующее теперь предоставляет более понятное сообщение об ошибке.
    let tt = multiply("t", "2");
    print(tt);

    // Это даёт разумный ответ.
    let twenty = multiply2("10", "2");
    print(twenty);

    // Следующее теперь предоставляет более понятное сообщение об ошибке.
    let tt = multiply2("t", "2");
    print(tt);
}
