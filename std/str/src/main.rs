fn main() {
    // (все аннотации типов избыточны)
    // Ссылка на строку, размещённую в read-only памяти
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    // Итерируемся по словам в обратном прядке, новая строка не аллоцируется
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // Копируем символы в вектор, сортируем и удаляем дубликаты
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // Создаём пустую расширяемую `String`
    let mut string = String::new();
    for c in chars {
        // Добавляем символ в конец строки
        string.push(c);
        // Добавляем в конец строки другую строку
        string.push_str(", ");
    }

    // Усечённая строка - это срез оригинальной строки, а значит новых
    // аллокаций не производится
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // Строка, аллоцированная в куче
    let alice = String::from("I like dogs");
    // Выделяется новая память, в которую сохраняется модифицированная строка
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}
