pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Это действительно плохая функция сложения, её назначение в данном // примере - потерпеть неудачу.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[allow(dead_code)]
fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("у отрицательного вещественного числа нет квадратного корня".to_owned())
    }
}

#[allow(dead_code)]
fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    } else if a < b {
        panic!("Divide result is zero");
    }
    a / b
}

#[allow(dead_code)]
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    // Обратите внимание на эту полезную идиому: импортирование имён из внешней (для mod - тестов) области видимости.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        // Это утверждение запустится и проверка не сработает.
        // Заметьте, что приватные функции также могут быть протестированы!
        assert_eq!(bad_add(1, 2), 3);
    }

    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }

    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 0);
    }

    #[test]
    #[should_panic(expected = "Divide result is zero")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 10);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(2, 2), 0);
    }

    #[test]
    fn test_sub_hundred() {
        assert_eq!(sub(100, 2), 98);
        assert_eq!(sub(2, 100), -98);
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        assert_eq!(sub(0, 0), 0);
    }
}
