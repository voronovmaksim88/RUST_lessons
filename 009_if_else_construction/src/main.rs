fn main() {
    let a = 10;
    if a > 5 {
        println!("a is greater than 5");
    } else {
        println!("a is less than or equal to 5");
    }

    let number = 5;
    if number > 6 {
        println!("number больше 6");
    } else if number < 6 {
        println!("number меньше 6");
    } else {
        println!("number равна 6");
    }

    // Конструкция let-if позволяет получить в переменную результат конструкции if..else:
    let condition = true;
    let number = if condition { 4 } else { 5 };
    println!("number = {}", number); // number = 4

    // более сложный пример
    let a = 5;
    let b = 2;
    let operation = 2;

    let number = if operation == 1 {
        a + b
    } else if operation == 2 {
        a - b
    } else {
        a * b
    };
    println!("number = {}", number); // number = 3
}
