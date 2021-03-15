use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => {
            "Абонент выключен или находится вне зоны действия сети. 
            Пожалуйста, позвоните позднее."
        }
        "645-7689" => {
            "Здравствуйте, это Mr. Awesome's Pizza. Меня зовут Фред.
            Что я могу сделать для вас?"
        }
        _ => "Привет! Кто это опять?",
    }
}

fn main() {
    let mut contacts = HashMap::new();

    contacts.insert("Даниель", "798-1364");
    contacts.insert("Эшли", "645-7689");
    contacts.insert("Кейти", "435-8291");
    contacts.insert("Роберт", "956-1745");

    // Возьмём ссылку и вернём `Option<&V>`
    match contacts.get(&"Даниель") {
        Some(&number) => println!("Звоним Даниелю: {}", call(number)),
        _ => println!("У нас нет номера Даниеля."),
    }

    // `HashMap::insert()` вернёт `None`, если мы добавляем
    // новое значение, иначе - `Some(value)`
    contacts.insert("Даниель", "164-6743");

    match contacts.get(&"Эшли") {
        Some(&number) => println!("Звоним Эшли: {}", call(number)),
        _ => println!("У нас нет номера Эшли."),
    }

    contacts.remove(&"Эшли");

    // `HashMap::iter()` возвращает итератор, который в произвольном
    // порядке отдаёт пары `(&'a key, &'a value)`.
    for (contact, &number) in contacts.iter() {
        println!("Звоним {}: {}", contact, call(number));
    }
}
