//! Модуль паттернов использования замыканий.
//!
//! Содержит примеры:
//! - замыканий с несколькими операторами
//! - сохранения замыканий в структурах
//! - работы с Option/Result

/// Демонстрирует замыкания с несколькими операторами и локальными переменными.
pub fn demonstrate_complex_closures() {
    println!("\n=== Пример 11: Замыкания с несколькими операторами ===");

    let complex_calculation = |x: i32, y: i32| -> i32 {
        let step1 = x + y;
        let step2 = step1 * 2;
        let step3 = step2 - 10;

        if step3 > 0 {
            step3 * step3
        } else {
            step3.abs()
        }
    };

    println!("Сложное вычисление для (5, 3): {}", complex_calculation(5, 3));
    println!("Сложное вычисление для (2, 1): {}", complex_calculation(2, 1));
}

/// Демонстрирует сохранение замыканий в структурах.
pub fn demonstrate_closures_in_structs() {
    println!("\n=== Пример 12: Замыкания в структурах ===");

    struct Validator<F>
    where
        F: Fn(&str) -> bool,
    {
        validation_fn: F,
        error_message: String,
    }

    impl<F> Validator<F>
    where
        F: Fn(&str) -> bool,
    {
        fn new(validation_fn: F, error_message: &str) -> Self {
            Validator {
                validation_fn,
                error_message: error_message.to_string(),
            }
        }

        fn validate(&self, input: &str) -> Result<(), &str> {
            if (self.validation_fn)(input) {
                Ok(())
            } else {
                Err(&self.error_message)
            }
        }
    }

    // Создаём валидатор с замыканием
    let email_validator = Validator::new(
        |s: &str| s.contains('@') && s.contains('.'),
        "Некорректный email адрес!",
    );

    println!("\nВалидация email:");
    match email_validator.validate("user@example.com") {
        Ok(()) => println!("user@example.com — Валидный!"),
        Err(e) => println!("user@example.com — {}", e),
    }
    match email_validator.validate("invalid-email") {
        Ok(()) => println!("invalid-email — Валидный!"),
        Err(e) => println!("invalid-email — {}", e),
    }
}

/// Демонстрирует использование замыканий с Option/Result.
pub fn demonstrate_option_result() {
    println!("\n=== Пример 13: Замыкания с Option/Result ===");

    let maybe_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;

    // map — применяет функцию к значению внутри Some
    let doubled = maybe_number.map(|x| x * 2);
    println!("Option::map: {:?}", doubled); // Some(84)

    // unwrap_or_else — возвращает значение или вычисляет замыкание для None
    let value = no_number.unwrap_or_else(|| {
        println!("Значение отсутствует, возвращаем значение по умолчанию");
        0
    });
    println!("Полученное значение: {}", value);

    // and_then — для цепочки операций, которые могут вернуть None
    let result = maybe_number
        .and_then(|x| if x > 0 { Some(x * 10) } else { None })
        .and_then(|x| Some(x + 1));
    println!("Результат цепочки: {:?}", result); // Some(421)
}

/// Запускает все примеры паттернов.
pub fn run_all() {
    demonstrate_complex_closures();
    demonstrate_closures_in_structs();
    demonstrate_option_result();
}
