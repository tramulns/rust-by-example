// raii.rs
fn create_box() {
    // Выделить память для целого число в куче
    let _box1 = Box::new(3i32);

    // `_box1` здесь уничтожается, а память освобождается
}

fn main() {
    // Выделить память для целого числа в куче
    let _box2 = Box::new(5i32);

    // Вложенная область видимости:
    {
        // Выделить память для ещё одного целого числа в куче
        let _box3 = Box::new(4i32);

        // `_box3` здесь уничтожается, а память освобождается
    }

    // Создаём большое количество упаковок. Просто потому что можем.
    // Здесь нет необходимости освобождать память вручную!
    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2` здесь уничтожается, а память освобождается
}