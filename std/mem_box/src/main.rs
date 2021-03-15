use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

// `Rectangle` может быть определён по расположению в пространстве
// его верхнего левого и нижнего правого углов
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // Аллоцируем точку в куче и вернём указатель на неё
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
    // (все аннотации типов избыточны)
    // Переменные, аллоцированные на стеке
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    };

    // Прямоугольник, аллоцированный в куче
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    // Результат функции может быть упакован
    let boxed_point: Box<Point> = Box::new(origin());

    // Двойная косвенность
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!("Point занимает {} байт на стеке", mem::size_of_val(&point));
    println!(
        "Rectangle занимает {} байт на стеке",
        mem::size_of_val(&rectangle)
    );

    // box size == pointer size
    println!(
        "Упакованный Point занимает {} байт на стеке",
        mem::size_of_val(&boxed_point)
    );
    println!(
        "Упакованный Rectangle занимает {} байт на стеке",
        mem::size_of_val(&boxed_rectangle)
    );
    println!(
        "Упакованная 'упаковка' занимает {} байт на стеке",
        mem::size_of_val(&box_in_a_box)
    );

    // Копируем данные из `boxed_point` в `unboxed_point`
    let unboxed_point: Point = *boxed_point;
    println!(
        "Распакованный Point занимает {} байт на стеке",
        mem::size_of_val(&unboxed_point)
    );
}
