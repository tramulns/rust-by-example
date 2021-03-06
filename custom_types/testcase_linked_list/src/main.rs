use List::*;

enum List {
    // Cons: Кортежная структура, которая хранит элемент
    // и указатель на следующий узел
    Cons(u32, Box<List>),
    // Nil: Узел, обозначающий конец связанного списка
    Nil,
}

// Методы могут быть присоединены к перечислению
impl List {
    // Создаём пустой список
    fn new() -> List {
        // `Nil` имеет тип `List`
        Nil
    }

    // Функция, которая принимает список и возвращает тот же список,
    // но с новым элементом в начале
    fn prepend(self, elem: u32) -> List {
        // `Cons` также имеет тип `List`
        Cons(elem, Box::new(self))
    }

    // Возвращаем длину списка
    fn len(&self) -> u32 {
        // `self` должен быть сопоставлен (проверен на соответствие),
        // поскольку поведение этого метода зависит от варианта `self`
        // `self` имеет тип `&List`, а `*self` имеет тип `List`, сопоставление на
        // конкретном типе `T` предпочтительнее, чем сопоставление по ссылке `&T`
        match *self {
            // Мы не можем завладеть `tail`, т.к. `self` заимствован;
            // вместо этого возьмём ссылку на `tail`
            Cons(_, ref tail) => 1 + tail.len(),
            // Базовый случай: Пустой список имеет нулевую длину
            Nil => 0,
        }
    }

    // Возвращаем представление списка в виде (размещённой в куче) строки
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` похож на `print!`, но возвращает строку
                // размещённую в куче, вместо вывода на консоль
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    // Создаём пустой связанный список
    let mut list = List::new();

    // Присоединяем несколько элементов
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Отображаем окончательное состояние списка
    println!("размер связанного списка: {}", list.len());
    println!("{}", list.stringify());
}
