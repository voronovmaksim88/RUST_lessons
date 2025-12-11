fn main() {
    // Базовые арифметические операции

    // Сложение
    let sum = 12 + 6;
    println!("12 + 6 = {}", sum); // 18

    // Вычитание
    let difference = 12 - 6;
    println!("12 - 6 = {}", difference); // 6

    // Умножение
    let product = 12 * 6;
    println!("12 * 6 = {}", product); // 72

    // Деление (целочисленное)
    let quotient = 12 / 6;
    println!("12 / 6 = {}", quotient); // 2

    // Деление с отбрасыванием дробной части
    let integer_division = 29 / 6;
    println!("29 / 6 = {}", integer_division); // 4 (дробная часть отбрасывается)

    // Остаток от деления
    let remainder = 15 % 6;
    println!("15 % 6 = {}", remainder); // 3

    println!(); // Пустая строка для разделения

    // Операции с присваиванием
    let mut number: i32;

    // Присваивание после сложения (+=)
    number = 12;
    number += 6;
    println!("12 += 6 → {}", number); // 18

    // Присваивание после вычитания (-=)
    number = 12;
    number -= 6;
    println!("12 -= 6 → {}", number); // 6

    // Присваивание после умножения (*=)
    number = 12;
    number *= 6;
    println!("12 *= 6 → {}", number); // 72

    // Присваивание после деления (/=)
    number = 12;
    number /= 6;
    println!("12 /= 6 → {}", number); // 2

    // Присваивание после деления по модулю (%=)
    number = 15;
    number %= 6;
    println!("15 %= 6 → {}", number); // 3

    println!(); // Пустая строка для разделения

    // Дополнительные примеры с разными типами чисел

    // Работа с числами с плавающей точкой
    let float_division = 29.0 / 6.0;
    println!("29.0 / 6.0 = {:.2}", float_division); // 4.83 (сохраняется дробная часть)

    // Работа с разными целочисленными типами
    let byte_number: u8 = 100;
    let result = byte_number + 28;
    println!("100 (u8) + 28 = {}", result); // 128

    // Использование разных числовых типов
    let int_number: i32 = 1000;
    let long_number: i64 = 500;
    let combined_result = int_number as i64 + long_number; // Приведение типа
    println!("1000 (i32) + 500 (i64) = {}", combined_result); // 1500
}