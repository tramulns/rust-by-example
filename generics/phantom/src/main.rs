use std::marker::PhantomData;

// Фантомная кортежная структура, которая имеет обобщение `A` со скрытым параметром `B`.
#[derive(PartialEq)] // Разрешаем для данного типа сравнения.
struct PhantomTuple<A, B>(A, PhantomData<B>);

// Фантомная структура, которая имеет обобщение `A` со скрытым параметром `B`.
#[derive(PartialEq)] // Разрешаем для данного типа сравнения.
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

// Заметьте: память выделена для обобщённого типа `A`, но не для `B`.
//           Следовательно, `B` не может быть использована в вычислениях.

fn main() {
    // Здесь `f32` и `f64` - скрытые параметры.
    // Тип PhantomTuple объявлен с `<char, f32>`.
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // Тип PhantomTuple объявлен с `<char, f64>`.
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    // Тип определён как `<char, f32>`.
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    // Тип определён как `<char, f64>`.
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
}
