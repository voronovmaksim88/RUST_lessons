//! Модуль захвата переменных в замыканиях.
//!
//! Содержит примеры:
//! - захвата по ссылке
//! - захвата по изменяемой ссылке
//! - принудительного перемещения с `move`

/// Демонстрирует захват переменных по ссылке и изменяемой ссылке.
pub fn demonstrate_capture_by_ref() {
    println!("\n=== Пример 4: Захват переменных из окружения ===");

    // Захват по ссылке (заимствование)
    let greeting = String::from("Привет");
    let greet = || {
        println!("{}, пользователь!", greeting);
    };
    greet();
    println!("Переменная greeting всё ещё доступна: {}", greeting);

    // Захват по изменяемой ссылке
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        println!("Счётчик: {}", counter);
    };
    increment();
    increment();
    increment();
    println!("Финальное значение счётчика: {}", counter);
}

/// Демонстрирует принудительное перемещение с ключевым словом `move`.
pub fn demonstrate_move_closure() {
    println!("\n=== Пример 5: Принудительное перемещение (move) ===");

    let name = String::from("Алексей");

    // С move замыкание забирает владение переменной name
    let print_name = move || {
        println!("Имя: {}", name);
    };

    print_name();
    // name больше недоступна здесь — она была перемещена в замыкание
    println!("Переменная name была перемещена в замыкание и больше недоступна");
}

/// Демонстрирует использование замыканий как аргументов функций.
pub fn demonstrate_closures_as_args() {
    println!("\n=== Пример 6: Замыкания как аргументы функций ===");

    fn apply_operation<F>(a: i32, b: i32, operation: F) -> i32
    where
        F: Fn(i32, i32) -> i32,
    {
        operation(a, b)
    }

    let sum = apply_operation(10, 20, |x, y| x + y);
    let product = apply_operation(10, 20, |x, y| x * y);
    let max = apply_operation(10, 20, |x, y| if x > y { x } else { y });

    println!("Сумма: {}", sum);
    println!("Произведение: {}", product);
    println!("Максимум: {}", max);
}

/// Запускает все примеры захвата переменных.
pub fn run_all() {
    demonstrate_capture_by_ref();
    demonstrate_move_closure();
    demonstrate_closures_as_args();
}
