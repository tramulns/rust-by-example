#![crate_name = "doc"]

/// Эта структура представляет человека
pub struct Person {
    /// Человек должен иметь имя вне зависимости от того, на сколько Джульетта его ненавидит
    name: String,
}

impl Person {
    /// Возвращает человека с данным ему именем
    ///
    /// # Аргументы
    ///
    /// * `name` - Срез строки, содержащий имя человека
    ///
    /// # Пример
    ///
    /// ```
    /// // Мы можете писать код на Rust внутри комментариев.
    /// // Если вы передадите `--test` в `rustdoc`, то он проверит его!
    /// use doc::Person;
    /// let person = Person::new("name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// Дружественное приветствие!
    ///
    /// Говорит "Привет, [name]" для `Person` у которого он вызывается.
    pub fn hello(&self) {
        println!("Привет, {}!", self.name);
    }
}

fn main() {
    let john = Person::new("John");

    john.hello();
}
