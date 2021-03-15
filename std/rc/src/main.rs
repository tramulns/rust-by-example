use std::rc::Rc;

fn main() {
    let rc_examples = "Пример с Rc".to_string();
    {
        println!("--- Создана rc_a ---");
        let rc_a: Rc<String> = Rc::new(rc_examples);
        println!("Количество ссылок на rc_a: {}", Rc::strong_count(&rc_a));
        {
            println!("--- rc_a клонировано в rc_b ---");
            let rc_b: Rc<String> = Rc::clone(&rc_a);
            println!("Количество ссылок на rc_b: {}", Rc::strong_count(&rc_b));
            println!("Количество ссылок на rc_a: {}", Rc::strong_count(&rc_a));
            // Два `Rc` равны, если равны их внутренние значения
            println!("rc_a и rc_b равны: {}", rc_a.eq(&rc_b));
            // Мы можем напрямую использовать методы внутреннего значения
            println!("Размер значения внутри rc_a: {}", rc_a.len());
            println!("Значение rc_b: {}", rc_b);
            println!("--- rc_b удаляется ---");
        }
        println!("Количество ссылок на rc_a: {}", Rc::strong_count(&rc_a));
        println!("--- rc_a удаляется ---");
    }
    // Ошибка! `rc_examples` уже перемещена в `rc_a`
    // И когда `rc_a` удалилась, `rc_examples` удалилась вместе с ней
    // println!("rc_examples: {}", rc_examples);
}
