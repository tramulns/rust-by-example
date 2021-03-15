macro_rules! calculate {
    (eval $e:expr) => {{
        {
            let val: usize = $e; // Заставим быть переменную целым числом.
            println!("{} = {}", stringify!{$e}, val);
        }
    }};
}

fn main() {
    calculate! {
        eval 1 + 2 // хе-хе, `eval` _не_ ключевое слово Rust!
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}
