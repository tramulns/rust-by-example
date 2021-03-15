use std::path::Path;

fn main() {
    // Создаём `Path` из `&'static str`
    let path = Path::new(".");

    // Метод `display` возвращает показываемую структуру
    let _display = path.display();

    // `join` соединяет `path` с байтовым контейнером, используя ОС-специфичный
    // разделитель, и возвращает новый путь
    let new_path = path.join("a").join("b");

    // Конвертируем путь в строковый срез
    match new_path.to_str() {
        None => panic!("новый путь не является действительной UTF-8 последовательностью"),
        Some(s) => println!("новый путь {}", s),
    }
}
