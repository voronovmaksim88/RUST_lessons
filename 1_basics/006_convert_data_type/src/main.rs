fn main() {
    /*
        В этом уроке разбираем:
        1) Явное приведение целочисленных типов (`as`)
        2) Потенциальное переполнение при сужающем преобразовании
        3) Преобразование чисел в строку и обратно
        4) Преобразование между целыми и числами с плавающей запятой
        5) Примеры с логикой: суммирование, ввод пользователя (эмуляция), разные типы

        ВНИМАНИЕ:
        - В Rust ПРЯМОЕ присваивание разных числовых типов запрещено (u8 -> u16, i32 -> u64 и т.п.).
        - Нужно использовать оператор `as` или функции парсинга/форматирования.
        - При приведении большего типа к меньшему (например, u16 -> u8) возможна потеря данных и переполнение.
    */

    println!("=== 1. Базовый пример: расширяющее преобразование (u8 -> u16) ===");

    // u8 может хранить числа от 0 до 255
    let number_u8: u8 = 10;

    // Нельзя так:
    // let number_u16: u16 = number_u8; // ошибка: mismatched types
    //
    // Правильно — использовать оператор `as`:
    let number_u16: u16 = number_u8 as u16;

    println!("number_u8  (u8)  = {}", number_u8);
    println!("number_u16 (u16) = {}", number_u16);
    println!("Комментарий: расширяющее преобразование (u8 -> u16) безопасно, т.к. диапазон u16 шире.\n");

    println!("=== 2. Сужающее преобразование (u16 -> u8) и переполнение ===");

    let big_u16: u16 = 257;
    /*
        Диапазон:
        - u16: 0..=65535
        - u8 : 0..=255

        Число 257 больше максимального значения u8 (255),
        поэтому при приведении произойдёт «обрезание» старших битов.

        В двоичном виде:
        257_u16 = 0b0000_0001_0000_0001

        u8 хранит только 8 младших битов:
        0b0000_0001_0000_0001 -> 0b0000_0001 = 1_u8

        То есть 257_u16 as u8 == 1_u8.
    */
    let small_u8: u8 = big_u16 as u8;
    println!("big_u16   (u16) = {}", big_u16);
    println!(
        "small_u8  (u8)  = {}  <-- произошло переполнение (обрезались старшие биты)",
        small_u8
    );
    println!();

    println!("=== 3. Примеры с другими целочисленными типами ===");

    let a_i32: i32 = -42;
    let b_u32: u32 = 100;

    // Нельзя так:
    // let c = a_i32 + b_u32; // ошибка: разные типы (i32 и u32)
    //
    // Вариант 1: привести оба к i64 (широкий тип со знаком)
    let sum_i64: i64 = a_i32 as i64 + b_u32 as i64;
    println!(
        "a_i32 = {}, b_u32 = {}, sum_i64 = {} (i64)",
        a_i32, b_u32, sum_i64
    );

    // Вариант 2: если точно знаем, что число неотрицательное
    // и хотим получить u32 — привести со знаком к беззнаковому типу:
    // ВНИМАНИЕ: при отрицательных значениях произойдёт переполнение!
    let a_i32_positive: i32 = 42;
    let sum_u32: u32 = (a_i32_positive as u32) + b_u32;
    println!(
        "a_i32_positive (i32) = {}, b_u32 = {}, sum_u32 = {} (u32)",
        a_i32_positive, b_u32, sum_u32
    );
    println!(
        "Комментарий: при приведении отрицательного i32 к u32 получится ОЧЕНЬ большое число (двоичное переполнение).\n"
    );

    println!("=== 4. Преобразование целых в числа с плавающей запятой и обратно ===");

    let int_value: i32 = 5;
    let float_value: f32 = 2.5;

    // Расширяющее преобразование: i32 -> f32
    let int_to_float: f32 = int_value as f32;
    println!(
        "int_value (i32) = {}, int_to_float (f32) = {}",
        int_value, int_to_float
    );

    // Можно спокойно складывать f32 + f32
    let float_sum: f32 = int_to_float + float_value;
    println!(
        "float_value (f32) = {}, float_sum (f32) = {}",
        float_value, float_sum
    );

    // Обратное преобразование: f32 -> i32
    /*
        ВНИМАНИЕ:
        - Дробная часть будет отброшена (по сути — усечение к нулю).
        - 2.9_f32 as i32 == 2
        - -2.9_f32 as i32 == -2
    */
    let float_example: f32 = 3.9;
    let truncated_int: i32 = float_example as i32;
    println!(
        "float_example (f32) = {}, truncated_int (i32) = {} (дробная часть отброшена)",
        float_example, truncated_int
    );
    println!();

    println!("=== 5. Преобразование числа в строку (to_string) ===");

    let n: i32 = 123;
    // Любой тип, реализующий трейт Display, можно превратить в String через to_string()
    let n_str: String = n.to_string(); // "123"
    println!("n (i32) = {}, n_str (String) = \"{}\"", n, n_str);

    let float_num: f64 = 3.14159;
    let float_str: String = float_num.to_string(); // обычно "3.14159"
    println!(
        "float_num (f64) = {}, float_str (String) = \"{}\"",
        float_num, float_str
    );
    println!(
        "Комментарий: метод to_string() удобен для подготовки данных к выводу или сериализации.\n"
    );

    println!("=== 6. Преобразование строки в число (parse) ===");

    let text_number = "42"; // &str
    /*
        Метод parse() — обобщённый (generic).
        Нужно явно указать в какой тип мы хотим преобразовать:
        - явный тип переменной: let x: i32 = text_number.parse().unwrap();
        - или оператор турбо-рыбы: let x = text_number.parse::<i32>().unwrap();

        parse() возвращает Result<T, E>, поэтому нужно обработать ошибку:
        - unwrap() — паникует при ошибке
        - expect("сообщение") — тоже паникует, но с нашим текстом
        - match или if let — безопасная обработка
    */
    let parsed_i32: i32 = text_number.parse().expect("Не удалось преобразовать строку в i32");
    println!(
        "text_number (\"{}\") -> parsed_i32 (i32) = {}",
        text_number, parsed_i32
    );

    let bad_text = "не число";
    let parsed_result: Result<i32, _> = bad_text.parse();
    match parsed_result {
        Ok(value) => println!("Успешно распарсили: {}", value),
        Err(e) => println!(
            "Ошибка при парсинге строки \"{}\" в i32: {}",
            bad_text, e
        ),
    }
    println!();

    println!("=== 7. Преобразование между &str, String и числовыми типами ===");

    let literal_str: &str = "100"; // строковый литерал
    let owned_string: String = String::from("200"); // отдельный объект String (во владении)

    // parse можно вызвать и на &str, и на String
    let value_from_literal: u32 = literal_str
        .parse()
        .expect("Ошибка парсинга literal_str");
    let value_from_owned: u32 = owned_string
        .parse()
        .expect("Ошибка парсинга owned_string");

    println!(
        "literal_str = \"{}\" -> value_from_literal (u32) = {}",
        literal_str, value_from_literal
    );
    println!(
        "owned_string = \"{}\" -> value_from_owned (u32) = {}",
        owned_string, value_from_owned
    );

    // Обратное преобразование: число -> String
    let result_sum: u32 = value_from_literal + value_from_owned;
    let result_str: String = result_sum.to_string();
    println!(
        "result_sum (u32) = {}, result_str (String) = \"{}\"",
        result_sum, result_str
    );
    println!();

    println!("=== 8. Безопасная логика с приведением типов ===");

    /*
        Частая задача: сложить значения разных типов и получить результат в "широком" типе.
        Например, есть:
        - количество элементов как u16
        - цена как u32
        Нам нужен итог как u64 (чтобы избежать переполнения).
    */
    let count: u16 = 300;
    let price_per_item: u32 = 1_000;

    // Приводим оба аргумента к u64 перед умножением.
    let total_cost: u64 = (count as u64) * (price_per_item as u64);
    println!(
        "count (u16) = {}, price_per_item (u32) = {}, total_cost (u64) = {}",
        count, price_per_item, total_cost
    );
    println!(
        "Комментарий: приведение к более широкому типу перед арифметикой помогает избежать переполнения.\n"
    );

    println!("=== 9. Ручной пример с переполнением при приведении u32 -> u8 ===");

    let values: [u32; 4] = [0, 10, 255, 256];
    for v in values {
        let as_u8: u8 = v as u8;
        println!(
            "v (u32) = {:>3} -> as_u8 (u8) = {:>3}",
            v, as_u8
        );
        /*
            Ожидаемый вывод:
            - 0   -> 0   (влезает в диапазон u8)
            - 10  -> 10  (влезает)
            - 255 -> 255 (максимум для u8)
            - 256 -> 0   (переполнение: 256 == 0b1_0000_0000, младшие 8 бит — все нули)
        */
    }
    println!();

    println!("=== 10. Итог ===");
    println!("
В этом примере мы увидели:
- как выполнять явное преобразование между числовыми типами с помощью оператора `as`;
- чем опасно сужающее преобразование (потеря данных, переполнение);
- как переводить числа в строки (`to_string`) и обратно (`parse`);
- как избегать переполнения, используя более широкие типы для вычислений.
");
}
