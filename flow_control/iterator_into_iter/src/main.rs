fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("Программисты Rust вокруг нас!"),
            _ => println!("Привет {}", name),
        }
    }
}
