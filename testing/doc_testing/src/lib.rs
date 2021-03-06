/// Первая строка - это краткое описание функции.
///
/// Следующие строки представляют детальную документацию. Блоки кода /// начинаются трёх обратных кавычек и внутри содержат неявные
/// `fn main()` и `extern crate <cratename>`. Предположим, мы
/// тестируем крейт `doc_testing`:
///
/// ```
/// let result = doc_testing::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Ообычно документационные комментарии могут содержат секции "Examples", "Panics" and "Failures".
///
/// Следующая функция делит два числа.
///
/// # Examples
///
/// ```
/// let result = doc_testing::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// Функция паникует, если второй аргумент равен нулю.
///
/// ```rust,should_panic
/// // паникует при делении на 0
/// doc_testing::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Ошибка деления на 0");
    }

    a / b
}

/// Использование скрытой `try_main` в документационных тестах.
///
/// ```
/// # // скрытые строки начинаются с символа `#`, но они всё ещё компилируемы!
/// # fn try_main() -> Result<(), String> { // эта линия оборачивает тело функции, которое отображается в документации
/// let res = doc_testing::try_div(10, 2)?;
/// # Ok(()) // возвращается из try_main
/// # }
/// # fn main() { // начало `main` которая выполняет `unwrap()`
/// #    try_main().unwrap(); // вызов `try_main` и извлечение результата
/// #                         // так что в случае ошибки этот тест запаникует
/// # }
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Деление на 0"))
    } else {
        Ok(a / b)
    }
}
