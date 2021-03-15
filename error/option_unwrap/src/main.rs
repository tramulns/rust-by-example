// Простолюдин видел всё это, и может справиться с любым подарком хорошо.
// Все подарки обрабатываются с помощью `match`.
fn give_commoner(gift: Option<&str>) {
    // Укажите порядок действий для каждого случая.
    match gift {
        Some("змея") => println!("Фу! Я унесу эту змею обратно в лес."),
        Some(inner) => println!("{}? Как хороший.", inner),
        None => println!("Нет подарка? Ну что же."),
    }
}

// Наша защищённая принцесса будет паниковать при виде змей.
// Все подарки обрабатываются неявно через `unwrap`.
fn give_princess(gift: Option<&str>) {
    // `unwrap` вызовет `panic` когда получит `None`.
    let inside = gift.unwrap();
    if inside == "змея" {
        panic!("AAAaaaaa!!!!");
    }

    println!("Я люблю {}!!!!!", inside);
}

fn main() {
    let food = Some("капуста");
    let snake = Some("змея");
    let void = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("малиновка");
    let nothing = None;

    give_princess(bird);
    give_princess(nothing);
}
