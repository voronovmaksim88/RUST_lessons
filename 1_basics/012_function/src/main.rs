fn main() {
    hello();
    welcome();
    square(5);
    show_user("John", 25);

    let num = sum(10, 20);
    println!("Сумма чисел 10 и 20 равна {}", num);

    let product = multiply(10, 20);
    println!("Произведение чисел 10 и 20 равно {}", product);
}

fn hello() {
    println!("Hello!");
}

fn welcome() {
    println!("Welcome to Rust World!");
}

fn square(n: i32) {
    let result = n * n;
    println!("Квадрат числа {} равен {}", n, result);
}

fn show_user(name: &str, age: i32) {
    println!("Информация о пользователе");
    println!("Имя: {}", name);
    println!("Возраст: {}", age);
}

fn sum(a: i32, b: i32) -> i32 // установка типа возвращаемого значения
{
    a + b // возвращение значения, после него не ставится точка с запятой
}

fn multiply(a: i32, b: i32) -> i32 // установка типа возвращаемого значения
{
    return a * b; // возвращение значения
}
