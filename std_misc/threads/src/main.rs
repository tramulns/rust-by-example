use std::thread;

static NTHREADS: i32 = 10;

// Это главный поток `main`
fn main() {
    // Создаём вектор дочерних потоков.
    let mut children = vec![];

    for i in 0..NTHREADS {
        // Создаём очередной поток
        children.push(thread::spawn(move || {
            println!("этот поток номер {}", i);
        }));
    }

    for child in children {
        // Ждём пока поток завершится и вернёт результат.
        let _ = child.join();
    }
}
