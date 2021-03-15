fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("Программисты Rust вокруг нас!"),
            _ => println!("Привет {}", name),
        }
    }
}
