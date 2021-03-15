#![allow(dead_code)]

#[derive(Debug)]
enum Food {
    CordonBleu,
    Steak,
    Sushi,
}
#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
}

// У нас нет ингридиентов для приготовления Sushi.
fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _ => Some(food),
    }
}

// У нас есть рецепты для всего, за исключением Cordon Bleu.
fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _ => Some(food),
    }
}

// Для приготовления блюда нам необходимы и рецепт, и ингредиент.
// Мы можем представить логику, как цепочку из`match`:
fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None => None,
        Some(food) => match have_ingredients(food) {
            None => None,
            Some(food) => Some(food),
        },
    }
}

// Для удобства это может быть переписано с использованием более компактного `and_then()`:
fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
}

fn eat(food: Food, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! В {:?} мы будем есть {:?}.", day, food),
        None => println!("О, нет. Мы не будем есть в {:?}?", day),
    }
}

fn main() {
    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}
