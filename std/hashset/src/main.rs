use std::collections::HashSet;

fn main() {
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // `HashSet::insert()` вернёт `false`
    // если элемент уже содержится в наборе.
    // assert!(b.insert(4), "Значение 4 уже есть в наборе B!");

    b.insert(5);

    // Если элементы коллекции реализуют `Debug`,
    // то и сама коллекция реализует `Debug`.
    // Обычно, элементы выводятся в формате `[elem1, elem2, ...]`
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // Выведет [1, 2, 3, 4, 5] в произвольном порядке
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

    // Выведет только [1]
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

    // Выведет [2, 3, 4] в произвольном порядке.
    println!(
        "Intersection: {:?}",
        a.intersection(&b).collect::<Vec<&i32>>()
    );

    // Выведет [1, 5]
    println!(
        "Symmetric Difference: {:?}",
        a.symmetric_difference(&b).collect::<Vec<&i32>>()
    );
}
