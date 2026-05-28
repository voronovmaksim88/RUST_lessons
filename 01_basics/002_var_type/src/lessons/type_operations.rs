//! Преобразование (`as`) и вывод типов (type inference).
//!
//! ## Вывод типов (type inference)
//!
//! Rust автоматически определяет тип переменной по контексту:
//! - `let x = 42;` → `i32`
//! - `let x = 3.14;` → `f64`
//! - Можно указать тип явно: `let x: u32 = 42;`
//!
//! ## Преобразование типов (type casting)
//!
//! Оператор `as` выполняет явное преобразование:
//! - `3.14_f64 as i32` → `3` (отбрасывание дробной части)
//! - `255u8 as u16` → `255` (расширение)
//! - `300u16 as u8` → `44` (усечение при переполнении!)
//!
//! ⚠️ Rust **не** выполняет неявные преобразования. Даже `i32 → i64`
//! требует явного `as`.

// ========================================================================
// ПРЕОБРАЗОВАНИЕ И ВЫВОД ТИПОВ (type casting and inference)
// ========================================================================
// Rust не выполняет автоматическое преобразование типов
// Компилятор может автоматически выводить типы из контекста

pub fn show() {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║       ПРЕОБРАЗОВАНИЕ И ВЫВОД ТИПОВ (type operations)      ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // ═══════════════════════════════════════════════════════════════════
    // ВЫВОД ТИПОВ (type inference)
    // ═══════════════════════════════════════════════════════════════════
    println!("═══ ВЫВОД ТИПОВ (type inference) ═══\n");

    println!("  Rust умеет автоматически определять типы по контексту");

    let inferred_int = 100; // i32 по умолчанию
    let inferred_float = 3.14; // f64 по умолчанию
    let inferred_bool = true; // bool
    let inferred_char = 'x'; // char
    let inferred_string = "Hello"; // &str

    println!("\n  Без явного указания типа:");
    println!("    let inferred_int = 100;        // Rust выведет i32");
    println!("    let inferred_float = 3.14;     // Rust выведет f64");
    println!("    let inferred_bool = true;      // Rust выведет bool");
    println!("    let inferred_char = 'x';       // Rust выведет char");
    println!("    let inferred_string = \"Hello\"; // Rust выведет &str");

    println!("\n  Значения:");
    println!("    inferred_int: {}", inferred_int);
    println!("    inferred_float: {}", inferred_float);
    println!("    inferred_bool: {}", inferred_bool);
    println!("    inferred_char: {}", inferred_char);
    println!("    inferred_string: {}", inferred_string);

    // Вывод из использования
    println!("\n═══ Вывод типа из использования ═══");

    let numbers = [1, 2, 3]; // Rust выводит [i32; 3]
    let first = numbers[0]; // Rust выводит i32

    println!("  let numbers = [1, 2, 3];");
    println!("    Rust выводит: [i32; 3]");
    println!("    numbers: {:?}", numbers);

    println!("\n  let first = numbers[0];");
    println!("    Rust выводит: i32");
    println!("    first: {}", first);

    // Вывод из вызова функции
    println!("\n═══ Вывод из контекста функции ═══");

    let parsed = "42".parse::<i32>().unwrap();
    println!("  let parsed = \"42\".parse::<i32>().unwrap();");
    println!("    Тип указан явно в turbofish ::<i32>");
    println!("    parsed: {}", parsed);

    // Аннотация типа при необходимости
    let result: f32 = "3.14".parse().unwrap();
    println!("\n  let result: f32 = \"3.14\".parse().unwrap();");
    println!("    Тип выводится из аннотации");
    println!("    result: {}", result);

    println!("\n  💡 Rust выводит типы где возможно, но иногда нужна явная аннотация");

    // ═══════════════════════════════════════════════════════════════════
    // ПРЕОБРАЗОВАНИЕ ТИПОВ (type casting)
    // ═══════════════════════════════════════════════════════════════════
    println!("\n\n═══ ПРЕОБРАЗОВАНИЕ ТИПОВ (as) ═══\n");

    println!("  Rust НЕ выполняет автоматическое приведение типов");
    println!("  Используйте 'as' для явного преобразования\n");

    // Целочисленные преобразования
    println!("  1. Целочисленные преобразования:");

    let integer: i32 = 42;
    let float_from_int: f64 = integer as f64;
    println!("    i32 -> f64:");
    println!("      let integer: i32 = 42;");
    println!("      let float: f64 = integer as f64;");
    println!("      {} -> {}", integer, float_from_int);

    let small: i8 = 100;
    let big: i32 = small as i32;
    println!("\n    i8 -> i32:");
    println!("      {} -> {}", small, big);

    let unsigned: u32 = 42;
    let signed: i32 = unsigned as i32;
    println!("\n    u32 -> i32:");
    println!("      {} -> {}", unsigned, signed);

    // Преобразование с потерей точности
    println!("\n  2. Преобразование с потерей точности:");

    let big_float: f64 = 255.999;
    let truncated: u8 = big_float as u8;
    println!("    f64 -> u8 (дробная часть отбрасывается):");
    println!("      {} -> {}", big_float, truncated);

    let large_int: i32 = 300;
    let overflow: i8 = large_int as i8;
    println!("\n    i32 -> i8 (переполнение!):");
    println!("      {} -> {} (биты отбрасываются)", large_int, overflow);
    println!("      ⚠️ 300 не помещается в i8 (max 127)");

    // Символы и числа
    println!("\n  3. Символы и числа:");

    let character: char = 'A';
    let ascii_code: u32 = character as u32;
    println!("    char -> u32 (Unicode код):");
    println!("      '{}' -> {}", character, ascii_code);

    let code: u8 = 65;
    let from_code: char = code as char;
    println!("\n    u8 -> char:");
    println!("      {} -> '{}'", code, from_code);

    let emoji: char = '😊';
    let emoji_code: u32 = emoji as u32;
    println!("\n    char (emoji) -> u32:");
    println!("      '{}' -> {} (0x{:X})", emoji, emoji_code, emoji_code);

    // Логические значения
    println!("\n  4. Логические значения:");

    let true_val: bool = true;
    let false_val: bool = false;
    let true_as_int: i32 = true_val as i32;
    let false_as_int: i32 = false_val as i32;
    println!("    bool -> i32:");
    println!("      true -> {}", true_as_int);
    println!("      false -> {}", false_as_int);
    println!("      ⚠️ Обратное преобразование (i32 -> bool) запрещено!");

    // ═══════════════════════════════════════════════════════════════════
    // ПОЛУЧЕНИЕ ИНФОРМАЦИИ О ТИПАХ
    // ═══════════════════════════════════════════════════════════════════
    println!("\n\n═══ ПОЛУЧЕНИЕ ИНФОРМАЦИИ О ТИПАХ ═══\n");

    println!("  Размеры типов в байтах (std::mem::size_of):");
    println!("    i8:    {} байт", std::mem::size_of::<i8>());
    println!("    i16:   {} байт", std::mem::size_of::<i16>());
    println!("    i32:   {} байт", std::mem::size_of::<i32>());
    println!("    i64:   {} байт", std::mem::size_of::<i64>());
    println!("    i128:  {} байт", std::mem::size_of::<i128>());
    println!("    u8:    {} байт", std::mem::size_of::<u8>());
    println!("    u16:   {} байт", std::mem::size_of::<u16>());
    println!("    u32:   {} байт", std::mem::size_of::<u32>());
    println!("    u64:   {} байт", std::mem::size_of::<u64>());
    println!("    u128:  {} байт", std::mem::size_of::<u128>());
    println!("    f32:   {} байт", std::mem::size_of::<f32>());
    println!("    f64:   {} байт", std::mem::size_of::<f64>());
    println!("    bool:  {} байт", std::mem::size_of::<bool>());
    println!("    char:  {} байт", std::mem::size_of::<char>());
    println!("    usize: {} байт", std::mem::size_of::<usize>());
    println!("    isize: {} байт", std::mem::size_of::<isize>());

    println!("\n  Размеры составных типов:");
    println!("    &str:       {} байт", std::mem::size_of::<&str>());
    println!("    (i32, i32): {} байт", std::mem::size_of::<(i32, i32)>());
    println!("    [i32; 5]:   {} байт", std::mem::size_of::<[i32; 5]>());
    println!("    ():         {} байт", std::mem::size_of::<()>());

    // ═══════════════════════════════════════════════════════════════════
    // БЕЗОПАСНЫЕ ПРЕОБРАЗОВАНИЯ
    // ═══════════════════════════════════════════════════════════════════
    println!("\n═══ БЕЗОПАСНЫЕ ПРЕОБРАЗОВАНИЯ (try_from/try_into) ═══\n");

    println!("  Для проверяемых преобразований используйте try_from/try_into:");
    println!("\n  use std::convert::TryFrom;\n");

    println!("  Пример:");
    println!("    let big_number: i32 = 1000;");
    println!("    match i8::try_from(big_number) {{");
    println!("        Ok(small) => println!(\"Успех: {{}}\", small),");
    println!("        Err(_) => println!(\"Ошибка: не помещается в i8\"),");
    println!("    }}");

    use std::convert::TryFrom;
    let big_number: i32 = 1000;
    match i8::try_from(big_number) {
        Ok(small) => println!("\n  Результат: Успех: {}", small),
        Err(_) => println!("\n  Результат: Ошибка: не помещается в i8"),
    }

    let small_number: i32 = 100;
    match i8::try_from(small_number) {
        Ok(small) => println!("  100 -> i8: Успех: {}", small),
        Err(_) => println!("  100 -> i8: Ошибка"),
    }

    // ═══════════════════════════════════════════════════════════════════
    // ПРАКТИЧЕСКИЕ ПРИМЕРЫ
    // ═══════════════════════════════════════════════════════════════════
    println!("\n═══ ПРАКТИЧЕСКИЕ ПРИМЕРЫ ═══");

    // Пример 1: Работа с RGB цветами
    println!("\n  1. RGB цвета (работа с байтами):");
    let red: u8 = 255;
    let green: u8 = 128;
    let blue: u8 = 64;
    let rgb: u32 = ((red as u32) << 16) | ((green as u32) << 8) | (blue as u32);
    println!("    R={}, G={}, B={}", red, green, blue);
    println!("    RGB как u32: 0x{:06X}", rgb);

    // Пример 2: Вычисления с разными типами
    println!("\n  2. Вычисления с разными типами:");
    let a: i32 = 10;
    let b: f64 = 3.14;
    let result = (a as f64) * b; // Нужно привести к одному типу
    println!("    i32 * f64 = f64");
    println!("    {} * {} = {}", a, b, result);

    // Пример 3: Индексация с usize
    println!("\n  3. Индексация массивов (нужен usize):");
    let array = [10, 20, 30, 40, 50];
    let index: i32 = 2;
    let value = array[index as usize]; // Приводим к usize для индекса
    println!("    array[{}] = {}", index, value);

    // Пример 4: Округление и truncate
    println!("\n  4. Способы преобразования float -> int:");
    let float_val: f64 = 3.7;
    let truncated = float_val as i32; // Отбрасывает дробную часть
    let rounded = float_val.round() as i32; // Округляет
    let floor = float_val.floor() as i32; // Округляет вниз
    let ceil = float_val.ceil() as i32; // Округляет вверх
    println!("    Исходное: {}", float_val);
    println!("    as i32 (truncate): {}", truncated);
    println!("    round() as i32:    {}", rounded);
    println!("    floor() as i32:    {}", floor);
    println!("    ceil() as i32:     {}", ceil);

    // ═══════════════════════════════════════════════════════════════════
    // ОПАСНОСТИ И ПРЕДУПРЕЖДЕНИЯ
    // ═══════════════════════════════════════════════════════════════════
    println!("\n═══ ОПАСНОСТИ И ПРЕДУПРЕЖДЕНИЯ ═══\n");

    println!("  ⚠️ Потеря данных при преобразовании:");
    println!("    • f64 -> i32: дробная часть отбрасывается");
    println!("    • i32 -> i8: переполнение (биты обрезаются)");
    println!("    • u32 -> i32: может стать отрицательным");

    println!("\n  ⚠️ Переполнение:");
    let overflow_example: i32 = 300;
    let overflow_result: i8 = overflow_example as i8;
    println!("    300 as i8 = {} (переполнение!)", overflow_result);
    println!("    Биты просто обрезаются без проверки");

    println!("\n  ⚠️ Знаковость:");
    let unsigned_val: u32 = 4_294_967_295; // максимальный u32
    let signed_val: i32 = unsigned_val as i32;
    println!("    {} (u32) as i32 = {}", unsigned_val, signed_val);
    println!("    Старший бит интерпретируется как знак!");

    println!("\n  ✓ Используйте безопасные методы где возможно:");
    println!("    • try_from() / try_into() - с проверкой ошибок");
    println!("    • checked_*() методы - возвращают Option");
    println!("    • saturating_*() методы - ограничивают по краям");

    // ═══════════════════════════════════════════════════════════════════
    // РЕКОМЕНДАЦИИ
    // ═══════════════════════════════════════════════════════════════════
    println!("\n═══ РЕКОМЕНДАЦИИ ПО ПРЕОБРАЗОВАНИЮ ТИПОВ ═══\n");

    println!("  Используйте 'as' когда:");
    println!("    ✓ Преобразование гарантировано безопасно");
    println!("    ✓ Расширяете тип (i8 -> i32, u32 -> u64)");
    println!("    ✓ Работаете с char и числами");
    println!("    ✓ Знаете что делаете и контролируете диапазон");

    println!("\n  Используйте try_from/try_into когда:");
    println!("    ✓ Сужаете тип (i32 -> i8, u64 -> u32)");
    println!("    ✓ Работаете с пользовательским вводом");
    println!("    ✓ Нужна гарантия корректности");
    println!("    ✓ Не уверены в диапазоне значений");

    println!("\n  Избегайте:");
    println!("    ✗ Цепочек преобразований без необходимости");
    println!("    ✗ Преобразований больших типов в меньшие без проверки");
    println!("    ✗ Смешивания signed и unsigned без осторожности");
    println!("    ✗ Предположений о размере usize/isize");

    println!("\n📝 Лучшие практики:");
    println!("  • Явное лучше неявного - используйте 'as'");
    println!("  • Документируйте намерения при преобразовании");
    println!("  • Проверяйте диапазоны для критичных данных");
    println!("  • Используйте типы подходящего размера изначально");
    println!("  • Rust не скрывает стоимость операций - будьте внимательны!");
}
