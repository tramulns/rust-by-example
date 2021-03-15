struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    // Данные могут быть доступны через ссылки и владельца этих данных
    println!(
        "Точка имеет координаты: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    // Ошибка! Нельзя заимствовать для изменения `point`, так как она уже
    // существует неизменяемая ссылка.
    // let mutable_borrow = &mut point;

    // Заимствованное значение снова используется
    println!(
        "Точка имеет координаты: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    // Неизменяемая ссылка больше не используется, так что можно перезаимствовать её
    // с помощью изменяемой ссылки.
    let mutable_borrow = &mut point;

    // Меняем при помощи изменяемой ссылки
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // Ошибка! Нельзя неизменяемо заимствовать `point` так как она уже
    // заимствована изменяемо.
    // let y = &point.y;

    // Ошибка! Нельзя вывести на экран, потому что `println!` берёт неизменяемую ссылку.
    // println!("Координата Z {}", point.z);

    // Ok! Изменяемая ссылка может быть передана `println!` как неизменяемая
    println!(
        "Точка имеет координаты: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );

    // Изменяемая ссылка больше не используется, так что можно перезаимствовать
    let new_borrowed_point = &point;
    println!(
        "Точка имеет координаты: ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    );
}
