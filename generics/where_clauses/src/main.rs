use std::fmt::Debug;

trait MyTrait<A, D> {}
trait TraitB {}
trait TraitC {}
trait TraitE {}
trait TraitF {}
struct YourType;

// impl<A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

// Выражение ограничений типажей через утверждение `where`
impl<A, D> MyTrait<A, D> for YourType
where
    A: TraitB + TraitC,
    D: TraitE + TraitF,
{
}

trait PrintInOption {
    fn print_in_option(self);
}

// Потому что в противном случае мы должны были бы выразить это как
// `T: Debug` или использовать другой метод косвенного подхода,
// для этого требуется утверждение `where`:
impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    // Мы хотим использовать `Option<T>: Debug` как наше ограничение
    // типажа, потому то это то, что будет напечатано. В противном случае
    // использовалось бы неправильное ограничение типажа.
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}
