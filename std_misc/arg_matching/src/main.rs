use std::env;

fn increase(number: i32) {
    println!("{}", number + 1);
}

fn decrease(number: i32) {
    println!("{}", number - 1);
}

fn help() {
    println!(
        "usage:
match_args <string>
    Проверяет является ли данная строка ответом.
match_args {{increase|decrease}} <integer>
    Увеличивает или уменьшает число на 1."
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // аргументы не переданы
        1 => {
            println!("Я - 'match_args'. Попробуйте передать аргументы!");
        }
        // передан один аргумент
        2 => match args[1].parse() {
            Ok(42) => println!("Это ответ!"),
            _ => println!("Это не ответ."),
        },
        // переданы одна команда и один аргумент
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            // parse the number
            let number: i32 = match num.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("ошибка: второй аргумент не является числом");
                    help();
                    return;
                }
            };
            // парсим команду
            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    eprintln!("ошибка: неизвестная команда");
                    help();
                }
            }
        }
        // все остальные случаи
        _ => {
            // показываем сообщение с помощью
            help();
        }
    }
}
