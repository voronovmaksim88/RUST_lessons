fn main() {
    let x = 5;
    let y = 7;
    let result: bool;
    result = x == y; // result = false - x не равно y
    println!("x={}, y={}, выражение x==y имеет значение {}", x, y, result);

    let result: bool;
    result = x != y; // result = true
    println!("x={}, y={}, выражение x!=y имеет значение {}", x, y, result);

    let result: bool;
    result = x > y; // result = false - x не больше y
    println!("x={}, y={}, выражение x>y имеет значение {}", x, y, result);

    let result: bool;
    result = x < y; // result = true - x меньше y
    println!("x={}, y={}, выражение x<y имеет значение {}", x, y, result);

    let result: bool;
    result = x >= y; // result = false - x не больше или равно y
    println!("x={}, y={}, выражение x>=y имеет значение {}", x, y, result);

    let result: bool;
    result = x <= y; // result = true - x меньше или равно y
    println!("x={}, y={}, выражение x<=y имеет значение {}", x, y, result);

    let z: bool = false;
    println!("z = {}", z);
    println!("!z = {}", !z);

    let a: bool = false;
    let b: bool = true;
    println!("a = {}", a);
    println!("b = {}", b);
    println!("a && b = {}", a && b);
    println!("a || b = {}", a || b);
}
