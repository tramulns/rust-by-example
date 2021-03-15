fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("Алиса"),
        age: 20,
    };

    // `name` перемещается из Person, но `age` ссылается
    let Person { name, ref age } = person;

    println!("Возраст человека {}", age);

    println!("Имя человека {}", name);

    // Ошибка! Частичное заимствование перемещенного значения: происходит частичное перемещение `person`
    // println!("The person struct is {:?}", person);

    // `person` не может быть использован, но `person.age` может быть использован, так как он не перемещается
    println!("Возраст человека из структуры Person {}", person.age);
}
