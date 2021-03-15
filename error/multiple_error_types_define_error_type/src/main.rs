use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

// Определите типы ошибок. Они могут быть настроены для наших случаев обработки ошибок.
// Теперь мы сможем написать наши собственные ошибки, реализовать приведение до основной ошибки
// или сделать что-то ещё между приведениями.
#[derive(Debug, Clone)]
struct DoubleError;

// Генерация ошибки полностью отделена от того, как она отображается.
// Нет необходимости в загромождении сложной логикой построения отображения ошибки.
//
// Мы не храним дополнительной информации об ошибках. Это означает, что мы не можем вывести строку, которую не удалось обработать, без изменения наших типов.
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "неверный первый элемент")
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        // Изменим ошибку на наш новый тип.
        .ok_or(DoubleError)
        .and_then(|s| {
            s.parse::<i32>()
                // Обновим тип ошибки также здесь.
                .map_err(|_| DoubleError)
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("Первое удвоение {}", n),
        Err(e) => println!("Ошибка: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
