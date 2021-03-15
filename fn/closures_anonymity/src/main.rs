// `F` должен реализовать `Fn` для замыкания, которое
// ничего не принимает и не возвращает - именно то,
// что нужно для `print`.
fn apply<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn main() {
    let x = 7;

    // Захватываем `x` в анонимный тип и реализуем
    // `Fn` для него. Сохраняем его как `print`.
    let print = || println!("{}", x);

    apply(print);
}
