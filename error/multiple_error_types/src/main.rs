fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // Генерирует ошибку 1
    2 * first.parse::<i32>().unwrap() // Генерирует ошибку 2
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("Первое удвоенное {}", double_first(numbers));

    println!("Первое удвоенное {}", double_first(empty));
    // Ошибка 1: входной вектор пустой

    println!("Первое удвоенное {}", double_first(strings));
    // Ошибка 2: элемент не может быть преобразован в число
}
