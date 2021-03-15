// Типаж, который реализует маркер печати: `{:?}`.
use std::fmt::{Debug, Display};

// Определим функцию `printer`, которая принимает обобщённый тип `T`,
// который должен реализовать типаж `Display`
#[allow(dead_code)]
fn printer<T: Display>(t: &T) {
    println!("{}", t);
}

// Обобщённый тип `T` должен реализовать `Debug`. Независимо
// от типа, это будет работать правильно.
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

// `T` должен реализовать `HasArea`. Любая функция, которая удовлетворяет
// ограничению может получить доступ к функции `area` из `HasArea`.
fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn main() {
    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));
}
