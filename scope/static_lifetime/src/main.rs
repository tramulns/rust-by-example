// Создадим константу со временем жизни `'static`.
static NUM: i32 = 18;

// Вернём ссылку на `NUM`, у которой собственное время жизни `'static`
// приводится ко времени жизни аргумента.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // Создадим *строковый* литерал и выведем его:
        let static_string = "Я в неизменяемой памяти";
        println!("static_string: {}", static_string);

        // Когда `static_string` выходит из области видимости, ссылка
        // на неё больше не может быть использована, но данные остаются в бинарном файле.
    }
    {
        // Создадим число для использования в `coerce_static`:
        let lifetime_num = 9;

        // Приведём `NUM` ко времени жизни `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }
    println!("NUM: {} остаётся доступным!", NUM);
}
