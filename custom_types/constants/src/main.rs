// Константы объявлены в глобальной области видимости.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Получаем доступ к константе внутри функции
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Получаем доступ к константе внутри функции main
    println!("Это язык {}", LANGUAGE);
    println!("Установим предел, равный {}", THRESHOLD);
    println!(
        "Число {} {} предела",
        n,
        if is_big(n) {
            "больше"
        } else {
            "меньше"
        }
    );

    // Ошибка! `const` нельзя изменить.
    // THRESHOLD = 5;
    // ИСПРАВЬТЕ ^ Закомментируйте эту строчку
}
