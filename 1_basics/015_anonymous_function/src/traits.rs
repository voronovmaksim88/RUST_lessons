//! Модуль типов замыканий (Fn, FnMut, FnOnce).
//!
//! Содержит примеры:
//! - различий между Fn, FnMut, FnOnce
//! - практического примера с калькулятором

/// Демонстрирует различия между типами замыканий.
pub fn demonstrate_closure_types() {
    println!("\n=== Пример 9: Типы замыканий (Fn, FnMut, FnOnce) ===");

    // Fn: замыкание, которое только читает захваченные переменные
    println!("Fn — только чтение захваченных переменных");

    // FnMut: замыкание, которое может изменять захваченные переменные
    println!("FnMut — изменение захваченных переменных");

    // FnOnce: замыкание, которое потребляет захваченные переменные
    let message = String::from("Это сообщение будет потреблено");
    let consume_and_print = move || {
        let consumed = message;
        println!("{}", consumed);
    };
    consume_and_print();
    // consume_and_print(); // Ошибка: замыкание уже потребило message
    println!("FnOnce — потребление захваченных переменных (вызывается один раз)");
}

/// Структура для хранения операций калькулятора.
struct Calculator {
    history: Vec<String>,
}

impl Calculator {
    fn new() -> Self {
        Calculator {
            history: Vec::new(),
        }
    }

    /// Метод, принимающий замыкание для выполнения операции.
    fn calculate<F>(&mut self, a: f64, b: f64, op_name: &str, operation: F) -> f64
    where
        F: Fn(f64, f64) -> f64,
    {
        let result = operation(a, b);
        self.history
            .push(format!("{} {} {} = {}", a, op_name, b, result));
        result
    }

    fn print_history(&self) {
        println!("\n=== История вычислений ===");
        for (i, entry) in self.history.iter().enumerate() {
            println!("{}. {}", i + 1, entry);
        }
    }
}

/// Демонстрирует практический пример с калькулятором.
pub fn demonstrate_calculator() {
    println!("\n=== Пример 10: Практический пример — калькулятор ===");

    let mut calc = Calculator::new();

    calc.calculate(10.0, 5.0, "+", |a, b| a + b);
    calc.calculate(10.0, 5.0, "-", |a, b| a - b);
    calc.calculate(10.0, 5.0, "*", |a, b| a * b);
    calc.calculate(
        10.0,
        5.0,
        "/",
        |a, b| if b != 0.0 { a / b } else { f64::NAN },
    );
    calc.calculate(2.0, 8.0, "^", |a, b| a.powf(b));

    calc.print_history();
}

/// Запускает все примеры типов замыканий.
pub fn run_all() {
    demonstrate_closure_types();
    demonstrate_calculator();
}
