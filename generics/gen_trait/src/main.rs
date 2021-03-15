// Некопируемые типы.
struct Empty;
struct Null;

// Обобщённый типаж от `T`.
trait DoubleDrop<T> {
    // Определим метод для типа вызывающего объекта,
    // который принимает один дополнительный параметр `T` и ничего с ним не делает.
    fn double_drop(self, _: T);
}

// Реализация `DoubleDrop<T>` для любого общего параметра `T` и
// вызывающего объекта `U`.
impl<T, U> DoubleDrop<T> for U {
    // Этот метод получает право владения на оба переданных аргумента и
    // освобождает их.
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    // Освободить `empty` и `null`.
    empty.double_drop(null);

    // empty;
    // null;
    // ^ TODO: Попробуйте раскомментировать эти строки.
}
