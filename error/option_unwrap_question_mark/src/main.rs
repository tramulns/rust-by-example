struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    // Получим из рабочего номера телефона код региона, если он существует.
    fn work_phone_area_code(&self) -> Option<u8> {
        // Мы можем не использовать оператор `?` и тогда здесь будет много вложенных операторов `match`.
        // С ним кода будет больше. Попробуйте использовать в этом коде `match` и посмотрите,
        // какой вариант проще.
        self.job?.phone_number?.area_code
    }
}

fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
}
