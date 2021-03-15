fn main() {
    let pair = (2, -2);
    // ЗАДАНИЕ ^ Попробуйте разные значения `pair`

    println!("Расскажи мне о {:?}", pair);
    match pair {
        (x, y) if x == y => println!("Близнецы"),
        // Данное ^ `условие if` является ограничителем шаблонов
        (x, y) if x + y == 0 => println!("Антиматерия, бабах!"),
        (x, _) if x % 2 == 1 => println!("Первое число нечётно"),
        _ => println!("Нет корреляции..."),
    }
}