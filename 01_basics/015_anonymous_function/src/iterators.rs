//! Модуль работы замыканий с итераторами.
//!
//! Содержит примеры:
//! - метода map
//! - метода filter
//! - метода fold
//! - комбинирования методов итераторов

/// Демонстрирует использование замыканий с итераторами.
pub fn demonstrate_iterators() {
    println!("\n=== Пример 7: Замыкания с итераторами ===");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map — применяет замыкание к каждому элементу
    let squares: Vec<i32> = numbers
        .iter() // Создаёт ИТЕРАТОР по ссылкам на элементы вектора
        .map(|x| x * x) // метод итератора, который применяет ЗАМЫКАНИЕ к каждому элементу
        .collect(); // "Собирает" все элементы из итератора в коллекцию
    println!("Квадраты чисел: {:?}", squares);

    // filter — оставляет только элементы, для которых замыкание вернёт true
    let even_numbers: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("Чётные числа: {:?}", even_numbers);

    // fold — «сворачивает» коллекцию в одно значение
    let sum_of_numbers: i32 = numbers
        .iter()
        .fold(0, |acc, x| if x % 2 == 0 { acc + x } else { acc });
    println!("Сумма всех чётных чисел: {}", sum_of_numbers);

    // Комбинирование нескольких методов в цепочку
    let result: i32 = numbers
        .iter()
        .filter(|x| *x % 2 == 1) // Берём только нечётные
        .map(|x| x * 2) // Умножаем на 2
        .fold(0, |acc, x| acc + x); // Суммируем
    println!("Сумма удвоенных нечётных чисел: {}", result);
}

/// Демонстрирует замыкания, возвращающие замыкания.
pub fn demonstrate_returning_closures() {
    println!("\n=== Пример 8: Замыкания, возвращающие замыкания ===");

    fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
        move |x| x * factor
    }

    let triple = create_multiplier(3);
    let quintuple = create_multiplier(5);

    println!("3 * 7 = {}", triple(7));
    println!("5 * 7 = {}", quintuple(7));
}

/// Запускает все примеры с итераторами.
pub fn run_all() {
    demonstrate_iterators();
    demonstrate_returning_closures();
}
