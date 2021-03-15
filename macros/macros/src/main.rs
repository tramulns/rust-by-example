// Этот простой макрос называется `say_hello`.
macro_rules! say_hello {
    // `()` указывает, что макрос не принимает аргументов.
    () => {
        // Макрос будет раскрываться с содержимым этого блока.
        println!("Hello!");
    };
}

fn main() {
    // Этот вызов будет раскрыт в код `println!("Hello");`
    say_hello!()
}
