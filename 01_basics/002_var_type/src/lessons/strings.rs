//! Строковый срез `&str`.
//!
//! `&str` — это ссылка на строковые данные (строковый литерал или срез `String`).
//! Записывается в двойных кавычках: `"текст"`.
//!
//! Особенности:
//! - **Неизменяемый** (immutable) по умолчанию.
//! - Всегда валидный UTF-8.
//! - Строковые литералы имеют тип `&'static str`.
//! - Индексация по байтам, а не по символам (осторожно с Unicode!).
//! - Экранирование: `\n`, `\t`, `\\`, `\"`.
//! - Сырые строки: `r"..."` (без экранирования), `r#"..."#` (с кавычками внутри).
//!
//! Полезные методы: `.len()`, `.trim()`, `.contains()`, `.chars()`, `.bytes()`.

// ========================================================================
// СТРОКОВЫЙ СРЕЗ (&str)
// ========================================================================
// &str — это ссылка на строковые данные (срез строки)
// Записывается в двойных кавычках ""
// Это НЕ примитивный тип, но очень важен для работы со строками

pub fn show() {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║                  СТРОКОВЫЙ СРЕЗ (&str)                     ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    println!("Базовая информация:");
    println!("  Размер ссылки &str: {} байт", std::mem::size_of::<&str>());
    println!("  Записывается в двойных кавычках: \"текст\"");
    println!("  Неизменяемый (immutable) по умолчанию");

    // Базовые строки
    println!("\n═══ Базовые строковые срезы ═══");
    let greeting: &str = "Привет, мир!";
    let language = "Rust"; // Тип &str выводится автоматически
    println!("  greeting: \"{}\"", greeting);
    println!("  language: \"{}\"", language);

    // Многострочные строки
    println!("\n═══ Многострочные строки ═══");
    let multiline = "Это первая строка
Это вторая строка
Это третья строка";
    println!("  Обычный способ:");
    println!("{}", multiline);

    // Сырые строки (raw strings)
    let raw_string = r"Это \n не \t будет \\ экранировано";
    println!("\n  Сырая строка r\"...\":");
    println!("  {}", raw_string);

    let raw_multiline = r#"
    {
        "name": "John",
        "age": 30
    }
    "#;
    println!("\n  Сырая строка с разделителями r#\"...\"#:");
    println!("{}", raw_multiline);

    // Экранирование специальных символов
    println!("\n═══ Экранирование (escape-последовательности) ═══");
    let with_quotes = "Он сказал: \"Привет!\"";
    let with_newline = "Первая строка\nВторая строка";
    let with_tab = "Колонка1\tКолонка2";
    let with_backslash = "Путь: C:\\Users\\Name";
    println!("  С кавычками: {}", with_quotes);
    println!("  С переносом:\n{}", with_newline);
    println!("  С табуляцией: {}", with_tab);
    println!("  С обратным слэшем: {}", with_backslash);

    // Все escape-последовательности
    println!("\n═══ Все escape-последовательности ═══");
    println!(r#"  \n  - новая строка"#);
    println!(r#"  \r  - возврат каретки"#);
    println!(r#"  \t  - табуляция"#);
    println!(r#"  \\  - обратный слэш"#);
    println!(r#"  \"  - двойная кавычка"#);
    println!(r#"  \'  - одинарная кавычка"#);
    println!(r#"  \0  - нулевой байт"#);

    // Unicode в строках
    println!("\n═══ Unicode в строках ═══");
    let unicode_string = "Привет 🌍! Hello 世界!";
    println!("  {}", unicode_string);
    println!("  Длина в байтах: {}", unicode_string.len());
    println!("  Количество символов: {}", unicode_string.chars().count());

    // Методы строк
    println!("\n═══ Полезные методы строк ═══");
    let sample = "  Rust Programming  ";
    println!("  Исходная: \"{}\"", sample);
    println!("  len(): {} байт", sample.len());
    println!("  is_empty(): {}", sample.is_empty());
    println!("  trim(): \"{}\"", sample.trim());
    println!("  to_lowercase(): \"{}\"", sample.to_lowercase());
    println!("  to_uppercase(): \"{}\"", sample.to_uppercase());
    println!("  contains(\"Rust\"): {}", sample.contains("Rust"));
    println!("  starts_with(\"  Ru\"): {}", sample.starts_with("  Ru"));
    println!("  ends_with(\"ing  \"): {}", sample.ends_with("ing  "));

    // Конкатенация
    println!("\n═══ Объединение строк ═══");
    let first = "Hello";
    let second = "World";
    let combined = format!("{} {}", first, second);
    println!("  format!(\"{{}} {{}}\", \"{}\", \"{}\")", first, second);
    println!("  Результат: \"{}\"", combined);

    let with_number = format!("Число: {}, Результат: {:.2}", 42, 3.14159);
    println!("  С форматированием: \"{}\"", with_number);

    // Срезы строк
    println!("\n═══ Срезы строк (slicing) ═══");
    let text = "Hello, Rust!";
    let hello = &text[0..5]; // или &text[..5]
    let rust = &text[7..11];
    let all = &text[..]; // вся строка
    println!("  Исходная: \"{}\"", text);
    println!("  &text[0..5]: \"{}\"", hello);
    println!("  &text[7..11]: \"{}\"", rust);
    println!("  &text[..]: \"{}\"", all);

    println!("\n  ⚠️ ОСТОРОЖНО: срезы должны быть на границах символов!");
    println!("     Для кириллицы и эмодзи индексы могут быть неочевидны");

    // Итерация по символам
    println!("\n═══ Итерация по строке ═══");
    let word = "Rust";
    print!("  Символы в \"{}\": ", word);
    for ch in word.chars() {
        print!("'{}' ", ch);
    }
    println!();

    print!("  Байты в \"{}\": ", word);
    for byte in word.bytes() {
        print!("{} ", byte);
    }
    println!();

    // Разница между String и &str
    println!("\n═══ Разница между String и &str ═══");
    println!("  &str:");
    println!("    • Неизменяемая ссылка на строковые данные");
    println!("    • Хранится на стеке (если литерал) или как срез");
    println!("    • Размер известен на этапе компиляции (для литералов)");
    println!("    • Записывается как \"текст\"");
    println!("    • Не владеет данными");
    println!("\n  String:");
    println!("    • Изменяемая, растущая строка");
    println!("    • Хранится в куче (heap)");
    println!("    • Может изменять размер во время выполнения");
    println!("    • Создаётся как String::from(\"текст\")");
    println!("    • Владеет своими данными");

    // Строковые литералы - статические
    println!("\n═══ Строковые литералы (статические) ═══");
    let static_str: &'static str = "Я существую всю программу";
    println!("  &'static str: \"{}\"", static_str);
    println!("  Хранится в бинарнике программы");
    println!("  Живёт всё время выполнения программы");

    println!("\n📝 Важные моменты:");
    println!("  • &str - это срез (view) строковых данных, не владелец");
    println!("  • Строки в Rust всегда валидный UTF-8");
    println!("  • Индексация по байтам, не по символам: text[0] - ОШИБКА!");
    println!("  • Используйте .chars() для работы с символами");
    println!("  • Используйте .bytes() для работы с байтами");
    println!("  • Литералы имеют тип &'static str");
    println!("  • Для изменяемых строк используйте String");
}
