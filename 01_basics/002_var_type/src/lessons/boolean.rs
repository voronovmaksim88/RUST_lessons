//! Логический тип `bool`.
//!
//! Принимает два значения: `true` или `false`. Занимает 1 байт.
//!
//! Поддерживаемые операции:
//! - Сравнение: `>`, `<`, `==`, `!=`, `>=`, `<=`
//! - Логические: `&&` (И), `||` (ИЛИ), `!` (НЕ)
//! - Короткое замыкание (short-circuit evaluation)
//! - Преобразование в число: `true as i32` → 1, `false as i32` → 0
//!
//! ⚠️ Rust **не** выполняет автоматическое преобразование в `bool`:
//! `if 1 { ... }` — ошибка, нужно `if true { ... }`.

// ========================================================================
// ЛОГИЧЕСКИЙ ТИП (boolean)
// ========================================================================
// Тип bool может принимать только два значения: true или false
// Занимает 1 байт в памяти

pub fn show() {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║                 ЛОГИЧЕСКИЙ ТИП (boolean)                   ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    let is_rust_awesome: bool = true;
    let is_learning_hard = false; // Тип выводится автоматически

    println!("Базовые значения:");
    println!("  Rust крутой? {}", is_rust_awesome);
    println!("  Учить сложно? {}", is_learning_hard);
    println!("  Размер bool: {} байт", std::mem::size_of::<bool>());

    // Логические выражения
    println!("\n═══ Операции сравнения ═══");
    let result = 10 > 5;
    let equal = 10 == 10;
    let not_equal = 10 != 5;
    let greater_or_equal = 10 >= 10;
    let less = 5 < 10;

    println!("  10 > 5:   {}", result);
    println!("  10 == 10: {}", equal);
    println!("  10 != 5:  {}", not_equal);
    println!("  10 >= 10: {}", greater_or_equal);
    println!("  5 < 10:   {}", less);

    // Логические операторы
    println!("\n═══ Логические операторы ═══");
    let a = true;
    let b = false;

    println!("  a = {}, b = {}", a, b);
    println!("  a && b (И):     {}", a && b); // логическое И
    println!("  a || b (ИЛИ):   {}", a || b); // логическое ИЛИ
    println!("  !a (НЕ):        {}", !a); // логическое НЕ
    println!("  !b (НЕ):        {}", !b);

    // Сложные выражения
    println!("\n═══ Сложные логические выражения ═══");
    let x = 5;
    let y = 10;
    let z = 15;

    let complex1 = (x < y) && (y < z);
    let complex2 = (x > y) || (y < z);
    let complex3 = !(x == y);

    println!("  x = {}, y = {}, z = {}", x, y, z);
    println!("  (x < y) && (y < z):  {} (5 < 10 И 10 < 15)", complex1);
    println!("  (x > y) || (y < z):  {} (5 > 10 ИЛИ 10 < 15)", complex2);
    println!("  !(x == y):           {} (НЕ (5 == 10))", complex3);

    // Короткое замыкание
    println!("\n═══ Короткое замыкание (short-circuit) ═══");
    println!("  && и || используют короткое замыкание:");
    println!("  • false && X -> не вычисляет X (уже false)");
    println!("  • true || X  -> не вычисляет X (уже true)");

    let short1 = false && {
        println!("    Это НЕ выполнится!");
        true
    };
    println!("  Результат false && ...: {}", short1);

    let short2 = true || {
        println!("    Это тоже НЕ выполнится!");
        false
    };
    println!("  Результат true || ...: {}", short2);

    // Использование в условиях
    println!("\n═══ Использование в условных конструкциях ═══");
    let age = 18;
    let has_permission = true;

    if age >= 18 && has_permission {
        println!("  Доступ разрешён ✓");
    } else {
        println!("  Доступ запрещён ✗");
    }

    // Преобразование в число
    println!("\n═══ Преобразование bool в число ═══");
    let true_as_num = true as i32;
    let false_as_num = false as i32;
    println!("  true as i32:  {}", true_as_num);
    println!("  false as i32: {}", false_as_num);

    println!("\n📝 Важные моменты:");
    println!("  • bool занимает 1 байт, хотя нужен только 1 бит");
    println!("  • Rust не выполняет автоматическое преобразование в bool");
    println!("  • Нельзя: if 1 {{ ... }} (только if true/false)");
    println!("  • && и || используют короткое замыкание");
    println!("  • bool можно преобразовать в число: true=1, false=0");
}
