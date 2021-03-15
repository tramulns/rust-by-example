struct Sheep {}
struct Cow {}

trait Animal {
    // Сигнатура метода объекта
    fn noise(&self) -> &'static str;
}

// Реализуем типаж `Animal` для `Sheep`.
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// Реализуем типаж `Animal` для `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// Вернём некоторую структуру, которая реализует `Animal`, но которая не известна в момент компиляции.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!(
        "Вы выбрали случайное животное и оно говорит {}",
        animal.noise()
    );
}
