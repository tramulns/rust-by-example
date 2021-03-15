struct Container(i32, i32);

// Типаж, который проверяет, сохранены ли 2 элемента в контейнере.
// Также он может вернуть первое или последнее значение.
trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool; // Явно требует `A` и `B`.
    fn first(&self) -> i32; // Не требует явного `A` или `B`.
    fn last(&self) -> i32; // Не требует явного `A` или `B`.
}

impl Contains<i32, i32> for Container {
    // Истина, если сохранённые цифры равны.
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // Берём первую цифру.
    fn first(&self) -> i32 {
        self.0
    }

    // Берём последнюю цифру.
    fn last(&self) -> i32 {
        self.1
    }
}

// `C` содержит `A` и `B`. В свете этого, необходимость снова явно указывать `A` и
// `B` огорчает.
fn difference<A, B, C>(container: &C) -> i32
where
    C: Contains<A, B>,
{
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Содержатся ли в контейнере {} и {}? {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("Первое число: {}", container.first());
    println!("Последнее число: {}", container.last());

    println!("Разница: {}", difference(&container));
}
