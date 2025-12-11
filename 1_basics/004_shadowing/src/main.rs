fn main() {
    let number: u8 = 1;
    println!("number = {}", number);

    let number: i8 = 2; // затенение
    println!("number = {}", number);

    let number: i8 = number + 1; // затенение с использованием прошлого значения
    println!("number = {}", number);
}
