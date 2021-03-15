// #![feature(never_type)]

#[allow(dead_code)]
fn foo() -> ! {
    panic!("Этот вызов никогда не вернёт управление.");
}

fn some_fn() {
    ()
}

fn main() {
    let _a: () = some_fn();
    println!("Эта функция возвращает управление и вы можете увидеть эту строку.");

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Обратите внимание, что возвращаемый тип этого выражения match должен быть u32
            // потому что такой тип в переменной "addition" .
            let addition: u32 = match i % 2 == 1 {
                // Переменная "i" типа u32, что совершенно нормально.
                true => i,
                // С другой стороны выражение "continue" не возвращает
                // u32, но это тоже нормально, потому что это тип не возвращающий управление,
                // не нарушает требования к типу выражения match.
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!(
        "Сумма нечётных чисел до 9 (исключая): {}",
        sum_odd_numbers(9)
    );

    // let x: ! = panic!("Этот вызов никогда не вернёт управление.");
    // println!("вы никогда не увидете эту строку!");
}
