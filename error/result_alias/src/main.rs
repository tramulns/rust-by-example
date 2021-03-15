use std::num::ParseIntError;

// Объявим обобщённый псевдоним для `Result` с типом ошибки `ParseIntError`.
type AliasedResult<T> = Result<T, ParseIntError>;

// Используем вышеуказанный псевдоним для обозначения
// нашего конкретного типа `Result`.
fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

// Здесь псевдоним снова позволяет нам сэкономить место.
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n) => println!("n это {}", n),
        Err(e) => println!("Ошибка: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
