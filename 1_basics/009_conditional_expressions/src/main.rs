// cSpell:disable
fn main() {
    println!("=== УРОК: УСЛОВНЫЕ ВЫРАЖЕНИЯ В RUST ===\n");

    // ============================================
    // 1. ОПЕРАТОРЫ СРАВНЕНИЯ (COMPARISON OPERATORS)
    // ============================================
    println!("1. ОПЕРАТОРЫ СРАВНЕНИЯ:");
    println!("------------------------");
    
    let x = 5;
    let y = 7;
    
    // Оператор равенства (==)
    let result: bool = x == y; // result = false - x не равно y
    println!("x={}, y={}, выражение x==y имеет значение {}", x, y, result);

    // Оператор неравенства (!=)
    let result: bool = x != y; // result = true
    println!("x={}, y={}, выражение x!=y имеет значение {}", x, y, result);

    // Оператор больше (>)
    let result: bool = x > y; // result = false - x не больше y
    println!("x={}, y={}, выражение x>y имеет значение {}", x, y, result);

    // Оператор меньше (<)
    let result: bool = x < y; // result = true - x меньше y
    println!("x={}, y={}, выражение x<y имеет значение {}", x, y, result);

    // Оператор больше или равно (>=)
    let result: bool = x >= y; // result = false - x не больше или равно y
    println!("x={}, y={}, выражение x>=y имеет значение {}", x, y, result);

    // Оператор меньше или равно (<=)
    let result: bool = x <= y; // result = true - x меньше или равно y
    println!("x={}, y={}, выражение x<=y имеет значение {}", x, y, result);

    // Сравнение с одинаковыми значениями
    let a = 10;
    let b = 10;
    println!("\nСравнение одинаковых значений:");
    println!("a={}, b={}, a==b = {}", a, b, a == b);
    println!("a={}, b={}, a>=b = {}", a, b, a >= b);
    println!("a={}, b={}, a<=b = {}", a, b, a <= b);

    // Сравнение чисел с плавающей точкой
    println!("\nСравнение чисел с плавающей точкой:");
    let f1: f64 = 3.14;
    let f2: f64 = 2.71;
    println!("f1={}, f2={}, f1>f2 = {}", f1, f2, f1 > f2);

    // Сравнение символов (char)
    println!("\nСравнение символов:");
    let ch1 = 'A';
    let ch2 = 'B';
    println!("ch1='{}', ch2='{}', ch1<ch2 = {}", ch1, ch2, ch1 < ch2);

    // Сравнение строк
    println!("\nСравнение строк:");
    let str1 = "apple";
    let str2 = "banana";
    println!("str1=\"{}\", str2=\"{}\", str1<str2 = {}", str1, str2, str1 < str2);
    println!("str1=\"{}\", str2=\"{}\", str1==str2 = {}", str1, str2, str1 == str2);

    // ============================================
    // 2. ЛОГИЧЕСКИЕ ОПЕРАТОРЫ (LOGICAL OPERATORS)
    // ============================================
    println!("\n\n2. ЛОГИЧЕСКИЕ ОПЕРАТОРЫ:");
    println!("------------------------");

    // Логическое НЕ (!)
    let z: bool = false;
    println!("z = {}", z);
    println!("!z = {}", !z);
    println!("!true = {}", !true);

    // Логическое И (&&)
    let a: bool = false;
    let b: bool = true;
    println!("\nТаблица истинности для && (И):");
    println!("a = {}", a);
    println!("b = {}", b);
    println!("a && b = {}", a && b);
    println!("false && false = {}", false && false);
    println!("false && true = {}", false && true);
    println!("true && false = {}", true && false);
    println!("true && true = {}", true && true);

    // Логическое ИЛИ (||)
    println!("\nТаблица истинности для || (ИЛИ):");
    println!("a = {}", a);
    println!("b = {}", b);
    println!("a || b = {}", a || b);
    println!("false || false = {}", false || false);
    println!("false || true = {}", false || true);
    println!("true || false = {}", true || false);
    println!("true || true = {}", true || true);

    // Комбинированные логические выражения
    println!("\nКомбинированные логические выражения:");
    let p = true;
    let q = false;
    let r = true;
    println!("p={}, q={}, r={}", p, q, r);
    println!("p && q || r = {}", p && q || r);
    println!("(p && q) || r = {}", (p && q) || r);
    println!("p && (q || r) = {}", p && (q || r));
    println!("!p || q && r = {}", !p || q && r);

    // ============================================
    // 3. УСЛОВНЫЕ ВЫРАЖЕНИЯ IF-ELSE
    // ============================================
    println!("\n\n3. УСЛОВНЫЕ ВЫРАЖЕНИЯ IF-ELSE:");
    println!("------------------------");

    // Простое условие if
    let number = 15;
    println!("number = {}", number);
    if number > 10 {
        println!("  -> number больше 10");
    }

    // Условие if-else
    let age = 18;
    println!("\nage = {}", age);
    if age >= 18 {
        println!("  -> Вы совершеннолетний");
    } else {
        println!("  -> Вы несовершеннолетний");
    }

    // Множественные условия if-else if-else
    let score = 85;
    println!("\nscore = {}", score);
    if score >= 90 {
        println!("  -> Оценка: Отлично (A)");
    } else if score >= 80 {
        println!("  -> Оценка: Хорошо (B)");
    } else if score >= 70 {
        println!("  -> Оценка: Удовлетворительно (C)");
    } else {
        println!("  -> Оценка: Неудовлетворительно (F)");
    }

    // IF КАК ВЫРАЖЕНИЕ (возвращает значение)
    println!("\nIF как выражение (возвращает значение):");
    let temperature = 25;
    let status = if temperature > 20 {
        "тепло"
    } else {
        "холодно"
    };
    println!("temperature = {}, status = \"{}\"", temperature, status);

    // Более сложное условное выражение
    let x = 5;
    let y = 10;
    let max_value = if x > y { x } else { y };
    println!("\nx={}, y={}, max_value={}", x, y, max_value);

    // Условное выражение с разными типами (не работает - нужен один тип)
    // let result = if true { 5 } else { "пять" }; // ОШИБКА!

    // ============================================
    // 4. ПРАКТИЧЕСКИЕ ПРИМЕРЫ
    // ============================================
    println!("\n\n4. ПРАКТИЧЕСКИЕ ПРИМЕРЫ:");
    println!("------------------------");

    // Проверка четности числа
    let num = 7;
    let is_even = num % 2 == 0;
    println!("num = {}, четное? {}", num, is_even);
    if is_even {
        println!("  -> Число четное");
    } else {
        println!("  -> Число нечетное");
    }

    // Проверка диапазона
    let value = 15;
    let in_range = value >= 10 && value <= 20;
    println!("\nvalue = {}, в диапазоне [10, 20]? {}", value, in_range);

    // Проверка нескольких условий
    let username = "admin";
    let password = "12345";
    let is_valid = username == "admin" && password == "12345";
    println!("\nusername = \"{}\", password = \"{}\"", username, password);
    println!("is_valid = {}", is_valid);

    // Использование в циклах (подготовка)
    println!("\nПроверка чисел от 1 до 5:");
    for i in 1..=5 {
        let is_positive = i > 0;
        let is_small = i < 3;
        println!("  i={}, положительное? {}, маленькое? {}", i, is_positive, is_small);
    }

    // ============================================
    // 5. КОРОТКОЕ ЗАМЫКАНИЕ (SHORT-CIRCUIT EVALUATION)
    // ============================================
    println!("\n\n5. КОРОТКОЕ ЗАМЫКАНИЕ:");
    println!("------------------------");
    println!("В Rust логические операторы используют короткое замыкание:");
    println!("  - && останавливается на первом false");
    println!("  - || останавливается на первом true");

    // Пример с функцией (для демонстрации концепции)
    fn check_value(x: i32) -> bool {
        println!("    Проверка значения: {}", x);
        x > 5
    }

    println!("\nПример с &&:");
    let result1 = check_value(3) && check_value(7);
    println!("  Результат: {} (вторая проверка не выполнилась)", result1);

    println!("\nПример с ||:");
    let result2 = check_value(7) || check_value(3);
    println!("  Результат: {} (вторая проверка не выполнилась)", result2);

    println!("\n=== КОНЕЦ УРОКА ===");
}
