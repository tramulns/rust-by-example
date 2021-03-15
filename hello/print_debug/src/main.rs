// Вывод и реализация `fmt::Debug` для `Structure`.
// `Structure` - это структура, которая содержит в себе один `i32`.
#[derive(Debug)]
struct Structure(i32);

// Добавим структуру `Structure` в структуру `Deep`.
// Реализуем для `Deep` возможность вывода с помощью fmt::Debug`.
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    // Вывод с помощью `{:?}` аналогичен `{}`.
    println!("{:?} месяцев в году.", 12);
    println!(
        "{1:?} {0:?} - это имя {actor:?}.",
        "Слэйтер",
        "Кристиан",
        actor = "актёра"
    );

    // `Structure` теперь можно напечатать!
    println!("Теперь {:?} будет выведена на экран!", Structure(3));

    // Проблема с `выводом (derive)`, в том, что у нас не будет контроля
    // над тем, как будет выглядеть результат.
    // Что если мы хотим напечатать просто `7`?
    println!("А теперь напечатаем {:?}", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}
