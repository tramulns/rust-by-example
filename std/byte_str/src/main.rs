use std::str;

fn main() {
    // Обратите внимание, что в действительности это не `&str`
    let bytestring: &[u8; 21] = b"this is a byte string";

    // Для массива байтов не реализован типаж `Display`, поэтому способы его печати ограничены
    println!("Строка байтов: {:?}", bytestring);

    // Байтовые строки могут содержать экранированные байты...
    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ...но не Unicode
    // let escaped = b"\u{211D} is not allowed";
    println!("Экранированные байты: {:?}", escaped);

    // Преобразование массива байт в `str` может завершиться ошибкой
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    // Но из-за этого они не всегда могут быть преобразованы в `str`
    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("И то же самое в виде текста: '{}'", my_str);
    }

    let _quotes = br#"You can also use "fancier" formatting, \
                    like with normal raw strings"#;

    // Байтовые строки не обязаны быть UTF-8
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // "ようこそ" in SHIFT-JIS

    // Но из-за этого они не всегда могут быть преобразованы в `str`
    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Удачное преобразование: '{}'", my_str),
        Err(e) => println!("Неудачное преобразование: {:?}", e),
    };
}
