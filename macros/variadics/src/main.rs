macro_rules! calculate {
    // Шаблон для единичного `eval`
    (eval $e:expr) => {{
        {
            let val: usize = $e; // Заставим быть переменную целым числом.
            println!("{} = {}", stringify!{$e}, val);
        }
    }};

    // Рекурсивно декомпозируем несколько `eval`
    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate! { eval $e }
        calculate! { $(eval $es),+ }
    }};
}

fn main() {
    calculate! { // Смотри, мама! Вариативный `calculate!`!
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}
