fn main() {
    let square = |n: i32| {
        let result = n * n;
        println!("Квадрат числа {} равен {}", n, result);
    };
    square(4);
    square(5);
    square(6);
    square(7);
}
