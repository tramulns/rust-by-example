fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "Программисты Rust вокруг нас!",
            _ => "Привет",
        }
    }

    println!("имена: {:?}", names);
}
