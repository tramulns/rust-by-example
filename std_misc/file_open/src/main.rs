use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Создадим "путь" к нужному файлу
    let path = Path::new("hello.txt");
    let display = path.display();

    // Откроем "путь" в режиме "только чтение". Возвращается `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("невозможно открыть {}: {}", display, why),
        Ok(file) => file,
    };

    // Читаем содержимое файла в строку. Метод возвращает `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("невозможно прочесть {}: {}", display, why),
        Ok(_) => print!("{} содержит:\n{}", display, s),
    }

    // `file` выходит из области видимости и файл "hello.txt" закрывается
}
