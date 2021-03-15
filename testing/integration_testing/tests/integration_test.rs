// мы тестируем extern crate, как и любой другой код.
extern crate integration_testing;

// импорт общего модуля.
mod common;

#[test]
fn test_add() {
    // использование общего кода.
    common::setup();
    assert_eq!(integration_testing::add(3, 2), 5);
}
