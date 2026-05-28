//! Символьный тип `char`.
//!
//! Хранит один Unicode-символ. Занимает **4 байта** (UTF-32).
//! Записывается в одинарных кавычках: `'A'`, `'😊'`, `'я'`.
//!
//! Особенности:
//! - Любой Unicode: кириллица, китайский, арабский, эмодзи.
//! - Unicode escape: `\u{1F601}` → 😊
//! - Методы: `.is_alphabetic()`, `.is_numeric()`, `.to_digit()`, `.to_uppercase()` и др.
//! - Байтовый литерал `b'X'` — это `u8`, а не `char`!

// ========================================================================
// СИМВОЛЬНЫЙ ТИП (char)
// ========================================================================
// char в Rust — это Unicode скалярное значение (4 байта)
// Записывается в одинарных кавычках ''
// Может представлять любой символ Unicode!

pub fn show() {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║                  СИМВОЛЬНЫЙ ТИП (char)                     ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    println!("Базовая информация:");
    println!("  Размер char: {} байта", std::mem::size_of::<char>());
    println!("  Кодировка: Unicode (UTF-32)");
    println!("  Записывается в одинарных кавычках: 'X'");

    // Латинские буквы
    println!("\n═══ Латинские буквы ═══");
    let letter: char = 'a';
    let uppercase = 'Z';
    let digit_char = '7';
    println!("  Строчная: {}", letter);
    println!("  Заглавная: {}", uppercase);
    println!("  Цифра как символ: {}", digit_char);

    // Русские буквы
    println!("\n═══ Кириллица ═══");
    let russian_lower = 'я';
    let russian_upper = 'Я';
    let russian_yo = 'ё';
    println!("  Строчная: {}", russian_lower);
    println!("  Заглавная: {}", russian_upper);
    println!("  Буква ё: {}", russian_yo);

    // Различные языки
    println!("\n═══ Другие языки ═══");
    let chinese = '你';
    let japanese = 'あ';
    let arabic = 'ع';
    let hebrew = 'א';
    println!("  Китайский иероглиф: {}", chinese);
    println!("  Японская хирагана: {}", japanese);
    println!("  Арабский: {}", arabic);
    println!("  Иврит: {}", hebrew);

    // Эмодзи
    println!("\n═══ Эмодзи (тоже символы!) ═══");
    let smile = '😁';
    let heart = '❤';
    let rocket = '🚀';
    let fire = '🔥';
    let thumbs_up = '👍';
    println!("  Смайл: {}", smile);
    println!("  Сердце: {}", heart);
    println!("  Ракета: {}", rocket);
    println!("  Огонь: {}", fire);
    println!("  Лайк: {}", thumbs_up);

    // Специальные символы
    println!("\n═══ Специальные символы ═══");
    let space = ' ';
    let _newline = '\n';
    let _tab = '\t';
    let _backslash = '\\';
    let _quote = '\'';
    println!("  Пробел: '{}' (ASCII 32)", space);
    println!("  Новая строка: '\\n' (escape-последовательность)");
    println!("  Табуляция: '\\t'");
    println!("  Обратный слэш: '\\\\'");
    println!("  Одинарная кавычка: '\\''");

    // Unicode escape-последовательности
    println!("\n═══ Unicode escape-последовательности ═══");
    let unicode_smile = '\u{1F601}'; // 😁
    let unicode_omega = '\u{03A9}'; // Ω
    let unicode_heart = '\u{2764}'; // ❤
    println!("  \\u{{1F601}} = {} (смайл)", unicode_smile);
    println!("  \\u{{03A9}} = {} (греческая омега)", unicode_omega);
    println!("  \\u{{2764}} = {} (сердце)", unicode_heart);

    // ASCII коды
    println!("\n═══ Работа с ASCII кодами ═══");
    let char_a = 'A';
    let ascii_code = char_a as u32;
    println!("  Символ 'A' -> код: {}", ascii_code);

    let code_65 = 65 as u8 as char;
    println!("  Код 65 -> символ: '{}'", code_65);

    // Математические символы
    println!("\n═══ Математические и специальные символы ═══");
    let plus = '+';
    let minus = '−'; // не дефис!
    let multiply = '×';
    let divide = '÷';
    let pi = 'π';
    let sum = '∑';
    let infinity = '∞';
    println!("  Плюс: {}", plus);
    println!("  Минус: {}", minus);
    println!("  Умножить: {}", multiply);
    println!("  Разделить: {}", divide);
    println!("  Пи: {}", pi);
    println!("  Сумма: {}", sum);
    println!("  Бесконечность: {}", infinity);

    // Методы для работы с char
    println!("\n═══ Полезные методы для char ═══");
    let test_char = 'A';
    println!("  Символ: '{}'", test_char);
    println!("  is_alphabetic(): {}", test_char.is_alphabetic());
    println!("  is_numeric(): {}", test_char.is_numeric());
    println!("  is_alphanumeric(): {}", test_char.is_alphanumeric());
    println!("  is_lowercase(): {}", test_char.is_lowercase());
    println!("  is_uppercase(): {}", test_char.is_uppercase());
    println!("  is_whitespace(): {}", test_char.is_whitespace());
    println!("  to_lowercase(): {}", test_char.to_lowercase());
    println!("  to_uppercase(): {}", test_char.to_uppercase());

    let digit = '5';
    println!("\n  Символ: '{}'", digit);
    println!("  is_numeric(): {}", digit.is_numeric());
    println!("  to_digit(10): {:?}", digit.to_digit(10));

    // Разница между char и &str
    println!("\n═══ Разница между char и &str ═══");
    let character: char = 'A'; // одинарные кавычки, 1 символ
    let string: &str = "A"; // двойные кавычки, строка
    println!(
        "  char 'A': {} (размер: {} байт)",
        character,
        std::mem::size_of::<char>()
    );
    println!(
        "  &str \"A\": {} (размер ссылки: {} байт)",
        string,
        std::mem::size_of::<&str>()
    );

    // Разница между char и u8 (байт)
    println!("\n═══ Разница между char и u8 (байт) ═══");
    let char_b: char = 'B'; // Unicode символ, 4 байта
    let byte_b: u8 = b'B'; // ASCII код, 1 байт
    println!(
        "  char 'B': {} (размер: {} байт, значение как u32: {})",
        char_b,
        std::mem::size_of::<char>(),
        char_b as u32
    );
    println!(
        "  u8 b'B': {} (размер: {} байт, ASCII код)",
        byte_b,
        std::mem::size_of::<u8>()
    );

    println!("\n📝 Важные моменты:");
    println!("  • char - это ОДИН Unicode символ (включая эмодзи)");
    println!("  • char занимает 4 байта (UTF-32)");
    println!("  • Одинарные кавычки '' для char, двойные \"\" для строк");
    println!("  • Может хранить любой Unicode символ (более 1 млн символов)");
    println!("  • Байтовый литерал b'X' - это u8, не char!");
    println!("  • char != u8: 'A' (4 байта) vs b'A' (1 байт)");
}
