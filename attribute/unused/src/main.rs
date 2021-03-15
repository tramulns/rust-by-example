fn used_function() {}

// `#[allow(dead_code)]` - атрибут, который убирает проверку на неиспользуемый код
#[allow(dead_code)]
fn unused_function() {}

#[allow(dead_code)]
fn noisy_unused_function() {}

fn main() {
    used_function();
}
