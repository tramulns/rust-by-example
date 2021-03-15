// cargo run --features some_condition

#[cfg(feature = "some_condition")]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    conditional_function();
}
