// ========================================================================
// КОНСТАНТЫ И СТАТИЧЕСКИЕ ПЕРЕМЕННЫЕ (constants and statics)
// ========================================================================
// Константы и статические переменные - это значения, которые существуют
// на протяжении всего времени выполнения программы

pub fn show() {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║         КОНСТАНТЫ И СТАТИЧЕСКИЕ ПЕРЕМЕННЫЕ                 ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // ═══════════════════════════════════════════════════════════════════
    // КОНСТАНТЫ (const)
    // ═══════════════════════════════════════════════════════════════════
    println!("═══ КОНСТАНТЫ (const) ═══\n");

    const MAX_POINTS: u32 = 100_000;
    const PI: f64 = 3.14159265358979323846;
    const GREETING: &str = "Добро пожаловать!";
    const IS_ENABLED: bool = true;

    println!("  Объявленные константы:");
    println!("    const MAX_POINTS: u32 = 100_000;");
    println!("    const PI: f64 = 3.14159265358979323846;");
    println!("    const GREETING: &str = \"Добро пожаловать!\";");
    println!("    const IS_ENABLED: bool = true;");

    println!("\n  Значения:");
    println!("    MAX_POINTS = {}", MAX_POINTS);
    println!("    PI = {}", PI);
    println!("    GREETING = \"{}\"", GREETING);
    println!("    IS_ENABLED = {}", IS_ENABLED);

    // Константы можно использовать в любом месте
    const SECONDS_IN_MINUTE: u32 = 60;
    const MINUTES_IN_HOUR: u32 = 60;
    const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;

    println!("\n  Вычисления с константами:");
    println!("    const SECONDS_IN_MINUTE: u32 = 60;");
    println!("    const MINUTES_IN_HOUR: u32 = 60;");
    println!("    const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;");
    println!("    SECONDS_IN_HOUR = {}", SECONDS_IN_HOUR);

    // Константы в массивах
    const ARRAY_SIZE: usize = 5;
    let array: [i32; ARRAY_SIZE] = [1, 2, 3, 4, 5];
    println!("\n  Использование в размере массива:");
    println!("    const ARRAY_SIZE: usize = 5;");
    println!("    let array: [i32; ARRAY_SIZE] = [1, 2, 3, 4, 5];");
    println!("    array = {:?}", array);

    println!("\n═══ Особенности констант ═══");
    println!("  ✓ Должны иметь явный тип");
    println!("  ✓ Имена пишутся ЗАГЛАВНЫМИ_БУКВАМИ");
    println!("  ✓ Значение должно быть известно на этапе компиляции");
    println!("  ✓ Могут быть объявлены в любой области видимости");
    println!("  ✓ Не занимают память во время выполнения");
    println!("  ✓ Встраиваются (inline) в места использования");
    println!("  ✓ Нельзя изменить (никогда и никак)");

    // ═══════════════════════════════════════════════════════════════════
    // СТАТИЧЕСКИЕ ПЕРЕМЕННЫЕ (static)
    // ═══════════════════════════════════════════════════════════════════
    println!("\n\n═══ СТАТИЧЕСКИЕ ПЕРЕМЕННЫЕ (static) ═══\n");

    static LANGUAGE: &str = "Rust";
    static VERSION: &str = "1.0.0";
    static MAX_CONNECTIONS: u32 = 1000;

    println!("  Объявленные статические переменные:");
    println!("    static LANGUAGE: &str = \"Rust\";");
    println!("    static VERSION: &str = \"1.0.0\";");
    println!("    static MAX_CONNECTIONS: u32 = 1000;");

    println!("\n  Значения:");
    println!("    LANGUAGE = \"{}\"", LANGUAGE);
    println!("    VERSION = \"{}\"", VERSION);
    println!("    MAX_CONNECTIONS = {}", MAX_CONNECTIONS);

    // Адрес в памяти
    println!("\n  Адрес в памяти:");
    println!("    &LANGUAGE = {:p}", &LANGUAGE);
    println!("    &MAX_CONNECTIONS = {:p}", &MAX_CONNECTIONS);
    println!("    💡 Статические переменные имеют фиксированный адрес");

    println!("\n═══ Особенности статических переменных ═══");
    println!("  ✓ Имеют фиксированный адрес в памяти");
    println!("  ✓ Живут всё время выполнения программы");
    println!("  ✓ Могут быть изменяемыми (но это unsafe!)");
    println!("  ✓ Имена пишутся ЗАГЛАВНЫМИ_БУКВАМИ");
    println!("  ✓ Значение должно быть известно на этапе компиляции");
    println!("  ✓ Занимают память в сегменте данных программы");

    // ═══════════════════════════════════════════════════════════════════
    // РАЗНИЦА МЕЖДУ const И static
    // ═══════════════════════════════════════════════════════════════════
    println!("\n\n═══ РАЗНИЦА МЕЖДУ const И static ═══\n");

    println!("  const (константа):");
    println!("    • Встраивается (inline) в код");
    println!("    • Нет адреса в памяти");
    println!("    • Копируется в каждое место использования");
    println!("    • Быстрее (нет обращения к памяти)");
    println!("    • Не может быть изменяемой");
    println!("    • Используйте для магических чисел и значений");

    println!("\n  static (статическая переменная):");
    println!("    • Хранится в памяти программы");
    println!("    • Имеет фиксированный адрес");
    println!("    • Одна копия на всю программу");
    println!("    • Может быть изменяемой (с unsafe)");
    println!("    • Используйте для глобального состояния");

    // ═══════════════════════════════════════════════════════════════════
    // ПРИМЕРЫ ИСПОЛЬЗОВАНИЯ
    // ═══════════════════════════════════════════════════════════════════
    println!("\n═══ ПРАКТИЧЕСКИЕ ПРИМЕРЫ ═══");

    // Пример 1: Математические константы
    println!("\n  1. Математические константы:");
    const E: f64 = 2.71828182845904523536;
    const PHI: f64 = 1.61803398874989484820;
    println!("    const E: f64 = 2.71828...;   // число Эйлера");
    println!("    const PHI: f64 = 1.61803...; // золотое сечение");
    println!("    E = {:.10}", E);
    println!("    PHI = {:.10}", PHI);

    // Пример 2: Конфигурационные значения
    println!("\n  2. Конфигурация приложения:");
    const APP_NAME: &str = "MyApp";
    const APP_VERSION: &str = "2.0.1";
    const MAX_RETRIES: u32 = 3;
    const TIMEOUT_MS: u64 = 5000;
    println!("    APP_NAME: {}", APP_NAME);
    println!("    APP_VERSION: {}", APP_VERSION);
    println!("    MAX_RETRIES: {}", MAX_RETRIES);
    println!("    TIMEOUT_MS: {} мс", TIMEOUT_MS);

    // Пример 3: Размеры буферов
    println!("\n  3. Размеры буферов:");
    const BUFFER_SIZE: usize = 8192;
    const MAX_PACKET_SIZE: usize = 1500;
    let buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    println!("    const BUFFER_SIZE: usize = 8192;");
    println!("    let buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];");
    println!("    Размер буфера: {} байт", buffer.len());
    println!("    MAX_PACKET_SIZE: {} байт", MAX_PACKET_SIZE);

    // Пример 4: Цвета (обычно как константы)
    println!("\n  4. Цветовые константы (RGB):");
    const COLOR_RED: (u8, u8, u8) = (255, 0, 0);
    const COLOR_GREEN: (u8, u8, u8) = (0, 255, 0);
    const COLOR_BLUE: (u8, u8, u8) = (0, 0, 255);
    const COLOR_WHITE: (u8, u8, u8) = (255, 255, 255);
    println!("    COLOR_RED: {:?}", COLOR_RED);
    println!("    COLOR_GREEN: {:?}", COLOR_GREEN);
    println!("    COLOR_BLUE: {:?}", COLOR_BLUE);
    println!("    COLOR_WHITE: {:?}", COLOR_WHITE);

    // Пример 5: Битовые флаги
    println!("\n  5. Битовые флаги:");
    const FLAG_READ: u8 = 0b0001;
    const FLAG_WRITE: u8 = 0b0010;
    const FLAG_EXECUTE: u8 = 0b0100;
    const FLAG_ALL: u8 = FLAG_READ | FLAG_WRITE | FLAG_EXECUTE;
    println!("    FLAG_READ: 0b{:04b}", FLAG_READ);
    println!("    FLAG_WRITE: 0b{:04b}", FLAG_WRITE);
    println!("    FLAG_EXECUTE: 0b{:04b}", FLAG_EXECUTE);
    println!("    FLAG_ALL: 0b{:04b}", FLAG_ALL);

    // ═══════════════════════════════════════════════════════════════════
    // ИЗМЕНЯЕМЫЕ СТАТИЧЕСКИЕ ПЕРЕМЕННЫЕ (небезопасно!)
    // ═══════════════════════════════════════════════════════════════════
    println!("\n\n═══ ИЗМЕНЯЕМЫЕ СТАТИЧЕСКИЕ ПЕРЕМЕННЫЕ ═══");
    println!("\n  ⚠️ Изменяемые static требуют unsafe блока!");
    println!("  ⚠️ Не рекомендуется для обычного использования!");
    println!("\n  Пример (только для демонстрации):");
    println!("    static mut COUNTER: u32 = 0;");
    println!("\n    unsafe {{");
    println!("        COUNTER += 1;");
    println!("        println!(\"COUNTER = {{}}\", COUNTER);");
    println!("    }}");
    println!("\n  💡 Вместо этого используйте:");
    println!("     • Mutex<T> для многопоточности");
    println!("     • AtomicUsize для атомарных операций");
    println!("     • RefCell<T> для одного потока");

    // ═══════════════════════════════════════════════════════════════════
    // РЕКОМЕНДАЦИИ
    // ═══════════════════════════════════════════════════════════════════
    println!("\n═══ КОГДА ЧТО ИСПОЛЬЗОВАТЬ ═══\n");

    println!("  Используйте const когда:");
    println!("    ✓ Нужны магические числа (PI, E, и т.д.)");
    println!("    ✓ Конфигурационные значения");
    println!("    ✓ Размеры массивов");
    println!("    ✓ Значения не меняются никогда");
    println!("    ✓ Нужна максимальная производительность");
    println!("    ✓ Большинство случаев!");

    println!("\n  Используйте static когда:");
    println!("    ✓ Нужен фиксированный адрес в памяти");
    println!("    ✓ Глобальное состояние (с осторожностью!)");
    println!("    ✓ Большие данные (чтобы не дублировать)");
    println!("    ✓ Взаимодействие с C библиотеками");

    println!("\n  Избегайте:");
    println!("    ✗ static mut без крайней необходимости");
    println!("    ✗ Глобального изменяемого состояния");
    println!("    ✗ Использования вместо параметров функций");

    println!("\n📝 Лучшие практики:");
    println!("  • Используйте const для неизменяемых значений");
    println!("  • Группируйте связанные константы в модули");
    println!("  • Документируйте назначение констант");
    println!("  • Имена ЗАГЛАВНЫМИ_БУКВАМИ с подчёркиваниями");
    println!("  • Избегайте глобального изменяемого состояния");
}
