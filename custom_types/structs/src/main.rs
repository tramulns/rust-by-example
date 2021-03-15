#[derive(Debug)]
struct Person<'a> {
    // 'a определяет время жизни
    name: &'a str,
    age: u8,
}

// unit-структура
struct Nil;

// Кортежная структура
struct Pair(i32, f32);

// Структура с двумя полями
struct Point {
    x: f32,
    y: f32,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// Структуры могут быть использованы в качестве полей другой структуры
#[allow(dead_code)]
struct Rectangle {
    // Прямоугольник может быть определён по расположению в пространстве
    // его верхнего левого и нижнего правого углов
    top_left: Point,
    bottom_right: Point,
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ {{ {} }}, {{ {} }} }}",
            self.top_left, self.bottom_right
        )
    }
}

fn main() {
    // Создадим структуру при помощи сокращённой инициализации полей
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Распечатаем отладочную информацию о структуре
    println!("{:?}", peter);

    // Инициализаруем `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Доступ к полям структуры
    println!("координаты точки: ({}, {})", point.x, point.y);

    // Создадим новую точку используя синтаксис обновления структуры и нашу
    // существующую точку
    let bottom_right = Point { x: 5.2, ..point };
    // `bottom_right.y` будет тем же самым, что и `point.y`, так как мы взяли
    // это поле из `point`
    println!("вторая точка: ({}, {})", bottom_right.x, bottom_right.y);

    // Деструктурируем структуру при помощи `let`
    let Point {
        x: top_edge,
        y: left_edge,
    } = point;

    let rectangle = Rectangle {
        // создание структуры также является выражением
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // Создадим unit-структуру
    let _nil = Nil;

    // Создадим кортежную структуру
    let pair = Pair(1, 0.1);

    // Доступ к полям кортежной структуры
    println!("pair содержит {:?} и {:?}", pair.0, pair.1);

    // Деструктурируем кортежную структуру
    let Pair(integer, decimal) = pair;

    println!("pair содержит {:?} и {:?}", integer, decimal);

    println!("площадь {} равна {}", rectangle, rect_area(&rectangle));

    println!("{}", square(&point, 10.));
}

fn rect_area(
    Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    }: &Rectangle,
) -> f32 {
    ((x2 - x1) * (y2 - y1)).abs()
}

fn square(start: &Point, length: f32) -> Rectangle {
    Rectangle {
        top_left: Point { ..*start },
        bottom_right: Point {
            x: start.x + length,
            y: start.y + length,
        },
    }
}
