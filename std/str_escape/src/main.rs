fn main() {
    // Вы можете использовать экранирование для записи байтов
    // при помощи их шестнадцатиричных значений...
    let byte_escape = "Я пишу на \x52\x75\x73\x74!";
    println!("Что ты делашь\x3F (\\x3F означает ?) {}", byte_escape);

    // ... или кодов Unicode.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode символ {} (U+211D) называется {}",
        unicode_codepoint, character_name
    );

    let long_string = "Строковый литерал
                       может занимать несколько строк.
                       Разрыв строки и отступ ->\
                       <- также можно экранировать!";
    println!("{}", long_string);
}
