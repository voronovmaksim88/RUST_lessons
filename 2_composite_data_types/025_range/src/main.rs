// ============================================================
// Урок 025. RANGE

fn main() {
    // Range: start..end - конец не включается.
    println!("1) Range: 1..9");
    for num in 1..9 {
        print!("{} ", num); // 1 2 3 4 5 6 7 8
    }
    println!();

    // То же самое, но через явную структуру std::ops::Range.
    let numbers = std::ops::Range { start: 1, end: 9 };
    for num in numbers {
        print!("{} ", num); // 1 2 3 4 5 6 7 8
    }
    println!("\n");

    // RangeInclusive: start..=end - конец включается.
    println!("2) RangeInclusive: 1..=9");
    for num in 1..=9 {
        print!("{} ", num); // 1 2 3 4 5 6 7 8 9
    }
    println!("\n");

    // RangeFrom: start.. - бесконечный диапазон, поэтому обычно ограничивают take().
    println!("3) RangeFrom: (5..).take(5)");
    for num in (5..).take(5) {
        print!("{} ", num); // 5 6 7 8 9
    }
    println!("\n");

    // RangeTo, RangeToInclusive и RangeFull чаще всего используются для срезов.
    println!("4) Срезы массива через разные виды range");
    let numbers = [10, 20, 30, 40, 50, 60, 70, 80];

    println!("&numbers[..]    = {:?}", &numbers[..]); // весь массив
    println!("&numbers[2..]   = {:?}", &numbers[2..]); // с индекса 2 до конца
    println!("&numbers[..3]   = {:?}", &numbers[..3]); // индексы 0, 1, 2
    println!("&numbers[..=3]  = {:?}", &numbers[..=3]); // индексы 0, 1, 2, 3
    println!("&numbers[1..5]  = {:?}", &numbers[1..5]); // индексы 1, 2, 3, 4
    println!();

    // Пример со строковым массивом: [1..5] берет элементы с индексами 1, 2, 3, 4.
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight",
    ];
    print!("words[1..5] = ");
    for word in words[1..5].iter() {
        print!("{} ", word); // two three four five
    }
    println!("\n");

    // Полезные методы и адаптеры итератора.
    println!("5) Адаптеры итератора");

    print!("(1..9).rev() = ");
    for num in (1..9).rev() {
        print!("{} ", num); // 8 7 6 5 4 3 2 1
    }
    println!();

    print!("(1..10).step_by(2) = ");
    for num in (1..10).step_by(2) {
        print!("{} ", num); // 1 3 5 7 9
    }
    println!();

    println!("(1..9).contains(&5) = {}", (1..9).contains(&5)); // true
    println!("(1..9).contains(&9) = {}", (1..9).contains(&9)); // false

    let collected: Vec<_> = (1..=5).collect();
    println!("(1..=5).collect::<Vec<_>>() = {:?}", collected); // [1, 2, 3, 4, 5]
    println!();

    // Диапазоны хорошо комбинируются с другими методами итераторов.
    println!("6) map(), filter(), sum(), fold()");

    // map() применяет замыкание к каждому элементу диапазона.
    // Здесь каждое число умножается само на себя:
    // 1 -> 1, 2 -> 4, 3 -> 9, 4 -> 16, 5 -> 25.
    // collect() собирает получившиеся значения в Vec.
    let squares: Vec<_> = (1..=5).map(|num| num * num).collect();
    println!("squares = {:?}", squares); // [1, 4, 9, 16, 25]

    // filter() оставляет только те элементы, для которых условие вернуло true.
    // num % 2 == 0 означает "число делится на 2 без остатка", то есть оно четное.
    // Из диапазона 1..=10 останутся 2, 4, 6, 8, 10.
    // sum() сложит все оставшиеся числа: 2 + 4 + 6 + 8 + 10 = 30.
    let even_sum: i32 = (1..=10).filter(|num| num % 2 == 0).sum();
    println!("sum of even numbers from 1 to 10 = {}", even_sum); // 30

    // fold() сворачивает весь диапазон в одно итоговое значение.
    // Первый аргумент - начальное значение аккумулятора, здесь это 1.
    // На каждом шаге acc хранит промежуточный результат:
    // 1 * 1 -> 1, 1 * 2 -> 2, 2 * 3 -> 6, 6 * 4 -> 24, 24 * 5 -> 120.
    // В итоге получается факториал числа 5.
    let factorial: i32 = (1..=5).fold(1, |acc, num| acc * num);
    println!("5! = {}", factorial); // 120
    println!();

    // Диапазоны можно использовать в match-паттернах.
    println!("7) Range in match");
    let value = 42;
    match value {
        0..=9 => println!("{} - однозначное число", value),
        10..=99 => println!("{} - двузначное число", value),
        _ => println!("{} - много знаков", value),
    }
    println!();

    // Диапазоны работают не только с числами, но и с char.
    println!("8) RangeInclusive для char");
    for ch in 'a'..='z' {
        print!("{} ", ch);
    }
    println!("\n");

    // Пустой диапазон: start == end, конец не включается.
    println!("9) Пустой диапазон: 1..1");
    for num in 1..1 {
        print!("{} ", num); // этот код ничего не напечатает
    }
    println!("ничего не вывелось");
}
