fn main() {
    // Целочисленное сложение
    println!("1 + 2 = {}", 1u32 + 2);

    // Целочисленное вычитание
    println!("1 - 2 = {}", 1i32 - 2);
    // ЗАДАНИЕ ^ Попробуйте изменить `1i32` на `1u32`
    // чтобы убедится насколько важен тип данных

    // Булева логика
    println!("true И false будет {}", true && false);
    println!("true ИЛИ false будет {}", true || false);
    println!("НЕ true будет {}", !true);

    // Побитовые операции
    println!("0011 И 0101 будет {:04b}", 0b0011u32 & 0b0101);
    println!("0011 ИЛИ 0101 будет {:04b}", 0b0011u32 | 0b0101);
    println!("0011 исключающее ИЛИ 0101 будет {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 будет {}", 1u32 << 5);
    println!("0x80 >> 2 будет 0x{:x}", 0x80u32 >> 2);

    // Использование подчёркивания для улучшения читаемости!
    println!("Один миллион записан как {}", 1_000_000u32);
}