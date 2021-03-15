// Структура с аннотированным временем жизни.
#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

// Аннотированное время жизни для реализации.
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self { x: &10 }
    }
}

fn main() {
    let b: Borrowed = Default::default();
    println!("b равно {:?}", b);
}
