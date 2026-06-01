// ============================================================
// Урок 024. ENUM TYPE
//
// 1. ЧТО ТАКОЕ ENUM?
//    Enum (перечисление) — это пользовательский тип данных,
//    который может принимать ОДНО из нескольких заранее
//    заданных значений (вариантов).
//
//    В Rust enum'ы гораздо мощнее, чем в C/C++ или Java.
//    Каждый вариант enum'а может нести в себе данные,
//    причём разные варианты могут хранить данные РАЗНЫХ типов!
//
// 2. ДЛЯ ЧЕГО НУЖНЫ ENUM'Ы?
//    - Моделирование конечного набора состояний (дни недели,
//      статусы заказа, типы ошибок)
//    - Представление "или-типов" (значение может быть
//      либо одним, либо другим)
//    - Обработка ошибок (Result<T, E>)
//    - Опциональные значения (Option<T>)
//    - Полиморфизм без трейтов (разные варианты с разными данными)
//
// 3. КАКИЕ БЫВАЮТ ENUM'Ы?
//    а) Простые (C-подобные) — только имена вариантов
//    б) С встроенными данными — каждый вариант хранит свои поля
//    в) Со встроенными кортежами — данные без имён полей
//    г) С явными числовыми значениями (дискриминантами)
//
// ============================================================

// ============================================================
//  Пример 1: Простой C-подобный enum (как в C/C++/Java)
// ============================================================
// Варианты не содержат данных — это просто флаги/метки.

enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl DayOfWeek {
    fn is_weekend(&self) -> bool {
        match self {
            DayOfWeek::Saturday | DayOfWeek::Sunday => true,
            _ => false,
        }
    }

    fn russian_name(&self) -> &str {
        match self {
            DayOfWeek::Monday => "Понедельник",
            DayOfWeek::Tuesday => "Вторник",
            DayOfWeek::Wednesday => "Среда",
            DayOfWeek::Thursday => "Четверг",
            DayOfWeek::Friday => "Пятница",
            DayOfWeek::Saturday => "Суббота",
            DayOfWeek::Sunday => "Воскресенье",
        }
    }
}

// Ещё один простой C-подобный enum: арифметические операции.
enum OperationType {
    Add,      // сложение
    Subtract, // вычитание
    Multiply, // умножение
    Divide,   // деление
}

fn get_result(x: i32, y: i32, op: OperationType) -> i32 {
    match op {
        OperationType::Add => x + y,
        OperationType::Subtract => x - y,
        OperationType::Multiply => x * y,
        OperationType::Divide => x / y,
    }
}

// ============================================================
//  Пример 2: Enum с явными числовыми значениями (дискриминантами)
// ============================================================
// Каждый вариант может иметь явный числовой "тег".
// По умолчанию дискриминанты начинаются с 0.

#[derive(Debug)]
enum HttpStatus {
    Ok = 200,
    Created = 201,
    NotFound = 404,
    InternalServerError = 500,
}

// ============================================================
//  Пример 3: Enum с РАЗНЫМИ данными у каждого варианта
//  (самый интересный — то, чего нет в C/Java!)
// ============================================================
// Каждый вариант хранит СВОИ собственные данные.
// Это позволяет моделировать разные ситуации с разной
// структурой внутри одного типа.

enum Payment {
    /// Наличные — просто сумма (u32)
    Cash(u32),
    /// Карта — номер карты и сумма
    Card { card_number: String, amount: u32 },
    /// Криптовалюта — адрес кошелька, сумма и комиссия
    Crypto {
        wallet: String,
        amount: u32,
        fee: f64,
    },
    /// Перевод на телефон — номер телефона и сумма
    PhoneTransfer(String, u32),
}

impl Payment {
    fn total_amount(&self) -> u32 {
        match self {
            // Cash хранит просто число в кортеже (безымянное поле)
            Payment::Cash(amount) => *amount,

            // Card хранит именованные поля
            Payment::Card { amount, .. } => *amount,

            // Crypto хранит amount + fee (округлим комиссию)
            Payment::Crypto { amount, fee, .. } => *amount + (*fee).round() as u32,

            // PhoneTransfer хранит кортеж (String, u32)
            Payment::PhoneTransfer(_, amount) => *amount,
        }
    }

    fn description(&self) -> String {
        match self {
            Payment::Cash(amount) => {
                format!("Наличные: {} руб.", amount)
            }
            Payment::Card {
                card_number,
                amount,
            } => {
                format!(
                    "Карта {}: {} руб.",
                    &card_number[card_number.len().saturating_sub(4)..],
                    amount
                )
            }
            Payment::Crypto {
                wallet,
                amount,
                fee,
            } => {
                format!(
                    "Крипта ({}): {} руб. + комиссия {:.2} руб.",
                    &wallet[..6],
                    amount,
                    fee
                )
            }
            Payment::PhoneTransfer(phone, amount) => {
                format!("Перевод на телефон {}: {} руб.", phone, amount)
            }
        }
    }
}

// ============================================================
//  Пример 4: Enum с данными — геометрические фигуры
// ============================================================
// Ещё один наглядный пример, где у каждой фигуры
// свои параметры, а считаем мы площадь единообразно.

enum Shape {
    /// Круг — радиус
    Circle { radius: f64 },
    /// Прямоугольник — ширина и высота
    Rectangle { width: f64, height: f64 },
    /// Треугольник — основание и высота
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle { .. } => "круг",
            Shape::Rectangle { .. } => "прямоугольник",
            Shape::Triangle { .. } => "треугольник",
        }
    }
}

// ============================================================
//  Пример 5: Встроенный enum Option<T> — "может быть, а может не быть"
// ============================================================
// Option — это enum из стандартной библиотеки:
//
//   enum Option<T> {
//       Some(T),  // значение есть
//       None,     // значения нет
//   }
//
// Используется ВЕЗДЕ вместо null из других языков.
// Компилятор ЗАСТАВЛЯЕТ обработать оба случая — это
// устраняет целый класс ошибок!

fn find_in_vector(vec: &[i32], target: i32) -> Option<usize> {
    for (index, &value) in vec.iter().enumerate() {
        if value == target {
            return Some(index);
        }
    }
    None
}

// ============================================================
//  Пример 6: Встроенный enum Result<T, E> — обработка ошибок
// ============================================================
// Result — это ещё один стандартный enum:
//
//   enum Result<T, E> {
//       Ok(T),   // успех
//       Err(E),  // ошибка
//   }
//
// Это основной способ обработки ошибок в Rust (без исключений!).

fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Деление на ноль!".to_string())
    } else {
        Ok(a / b)
    }
}

// ============================================================
//  main — запускаем все примеры
// ============================================================

fn main() {
    println!("========== ПРИМЕР 1: enum DayOfWeek ==========");
    // Перебираем все дни недели
    let days = [
        DayOfWeek::Monday,
        DayOfWeek::Tuesday,
        DayOfWeek::Wednesday,
        DayOfWeek::Thursday,
        DayOfWeek::Friday,
        DayOfWeek::Saturday,
        DayOfWeek::Sunday,
    ];
    for day in &days {
        println!("{} — выходной? {}", day.russian_name(), day.is_weekend());
    }
    println!();

    // Ещё один простой C-подобный enum: арифметические операции.
    println!("--- OperationType ---");
    let a = 10;
    let b = 5;
    let mut op = OperationType::Add;
    let mut result = get_result(a, b, op);
    println!("{} + {} = {}", a, b, result); // 15

    op = OperationType::Subtract;
    result = get_result(a, b, op);
    println!("{} - {} = {}", a, b, result); // 5

    result = get_result(a, b, OperationType::Multiply);
    println!("{} * {} = {}", a, b, result); // 50

    result = get_result(a, b, OperationType::Divide);
    println!("{} / {} = {}", a, b, result); // 2

    println!();

    // ======================================================
    println!("========== ПРИМЕР 2: enum с дискриминантами ==========");
    // Дискриминанты можно привести к числу через `as`
    println!("  Ok  = {}", HttpStatus::Ok as u16);
    println!("  Created = {}", HttpStatus::Created as u16);
    println!("  NotFound = {}", HttpStatus::NotFound as u16);
    println!(
        "  InternalServerError = {}",
        HttpStatus::InternalServerError as u16
    );

    // Можно использовать в match для сопоставления с числом
    let code = 200;
    match code {
        200 => println!("Код 200 — Ура, успех!"),
        404 => println!("Код 404 — Не найдено..."),
        500 => println!("Код 500 — Ошибка сервера"),
        _ => println!("Какой-то другой статус"),
    }

    println!();

    // ======================================================
    println!("========== ПРИМЕР 3: enum Payment с разными данными ==========");
    let payments = vec![
        Payment::Cash(5000),
        Payment::Card {
            card_number: "1234-5678-9012-3456".to_string(),
            amount: 2500,
        },
        Payment::Crypto {
            wallet: "0xABC123DEF456".to_string(),
            amount: 10000,
            fee: 0.5,
        },
        Payment::PhoneTransfer("+7-912-345-67-89".to_string(), 1500),
    ];

    let mut grand_total = 0;
    for payment in &payments {
        println!("  {}", payment.description());
        grand_total += payment.total_amount();
    }
    println!("  Итого: {} руб.", grand_total);

    println!();

    // ======================================================
    println!("========== ПРИМЕР 4: enum Shape с разными данными ==========");
    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle {
            width: 4.0,
            height: 6.0,
        },
        Shape::Triangle {
            base: 3.0,
            height: 8.0,
        },
    ];

    for shape in &shapes {
        println!("Площадь {} = {:.2}", shape.name(), shape.area());
    }

    println!();

    // ======================================================
    println!("========== ПРИМЕР 5: Option<T> ==========");
    let numbers = vec![10, 20, 30, 40, 50];
    let target = 30;

    match find_in_vector(&numbers, target) {
        Some(index) => println!("Число {} найдено на позиции {}", target, index),
        None => println!("Число {} не найдено", target),
    }

    // find_in_vector вернёт None
    match find_in_vector(&numbers, 999) {
        Some(index) => println!("Число {} найдено на позиции {}", 999, index),
        None => println!("Число {} не найдено", 999),
    }

    // Комбинаторы Option: map, unwrap_or, and_then и т.д.
    let index = find_in_vector(&numbers, 20)
        .map(|i| i + 1) // нумерация с 1
        .unwrap_or(0);
    println!("Позиция числа 20 (счёт с 1): {}", index);

    println!();

    // ======================================================
    println!("========== ПРИМЕР 6: Result<T, E> ==========");
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Ошибка: {}", e),
    }

    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Ошибка: {}", e),
    }

    // Комбинаторы Result: unwrap_or, unwrap_or_else, map_err и т.д.
    let result = safe_divide(100.0, 3.0).unwrap_or(0.0);
    println!("100 / 3 (с запасным значением): {:.2}", result);
}
