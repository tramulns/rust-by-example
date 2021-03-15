use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
}

fn double_first2(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));

    opt.map_or(Ok(None), |r| r.map(Some))
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("Первое удвоенное: {:?}", double_first(numbers));

    println!("Первое удвоенное: {:?}", double_first(empty));
    // Ошибка первая: исходный вектор пустой

    println!("Первое удвоенное {:?}", double_first(strings));
    // Ошибка вторая: элемент не переводится в число

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("Первое удвоенное: {:?}", double_first2(numbers));
    println!("Первое удвоенное: {:?}", double_first2(empty));
    println!("Первое удвоенное: {:?}", double_first2(strings));
}
