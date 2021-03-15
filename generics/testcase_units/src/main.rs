use std::marker::PhantomData;
use std::ops::Add;

/// Создаём пустые перечисления для определения типов единиц измерения.
#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

/// `Length` - тип с параметром фантомного типа `Unit`,
/// и не обобщён для типа длины (который `f64`).
///
/// Для `f64` уже реализованы типажи `Clone` и `Copy`.
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

/// Типаж `Add` объявляет поведение оператора `+`.
impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    // add() возвращает новую структуру `Length`, содержащую сумму.
    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        // `+` вызывает реализацию `Add` для `f64`.
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    // Объявим, что `one_foot` имеет парамет фантомного типа `Inch`.
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    // `one_meter` имеет параметр фантомного типа `Mm`.
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    // `+` вызывает метод `add()`, который мы реализовали для `Length<Unit>`.
    //
    // Так как `Length` реализует `Copy`, `add()` не поглощает
    // `one_foot` и `one_meter`, а копирует их в `self` и `rhs`.
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    // Сложение работает.
    println!("один фут + один фут = {:?} фута", two_feet.0);
    println!("один метр + один метр = {:?} метра", two_meters.0);
}
