#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` - это ссылка на строку, расположенную в неизменяемой памяти
    author: &'static str,
    title: &'static str,
    year: u32,
}

// Эта функция получает ссылку на книгу
fn borrow_book(book: &Book) {
    println!(
        "Я неизменяемо заимствовала {} - {} издания",
        book.title, book.year
    );
}

// Эта функция получает изменяемую ссылку на книгу и устанавливает поле `year` в 2014
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!(
        "Я изменяемо заимствовала {} - {} издания",
        book.title, book.year
    );
}

fn main() {
    // Создаём неизменяемую книгу в переменной `immutabook`
    let immutabook = Book {
        // строковый литерал имеет тип `&'static str`
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // Создаём изменяемую копию `immutabook` и называем её `mutabook`
    let mut mutabook = immutabook;
    // Неизменяемое заимствование неизменяемого объекта
    borrow_book(&immutabook);

    // Неизменяемое заимствование изменяемого объекта
    borrow_book(&mutabook);
    // Заимствование изменяемого объекта как изменяемого
    new_edition(&mut mutabook);
    // Ошибка! Нельзя заимствовать неизменяемый объект как изменяемый
    // new_edition(&mut immutabook);
}
