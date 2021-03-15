fn main() {
    let mut _mutable_integer = 7i32;

    {
        // Затенение неизменяемой переменной `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // Ошибка! `_mutable_integer` заморожен в этой области видимости
        // _mutable_integer = 50;

        // `_mutable_integer` выход из области видимости
    }

    // Ok! `_mutable_integer` не заморожен в этой области видимости
    _mutable_integer = 3;
}
