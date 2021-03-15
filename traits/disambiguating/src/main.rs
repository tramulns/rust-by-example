trait UsernameWidget {
    // Получить из виджета имя пользователя
    fn get(&self) -> String;
}

trait AgeWidget {
    // Получить из виджета возраст
    fn get(&self) -> u8;
}

// Форма, реализующая оба трейта: и `UsernameWidget`, и `AgeWidget`
struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };

    // Если вы раскомментируете эту строку, вы получите ошибку, которая говорит
    // "multiple `get` found". Потому что это, в конце концов, несколько методов
    // с именем `get`.
    // println!("{}", form.get());

    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);
}
