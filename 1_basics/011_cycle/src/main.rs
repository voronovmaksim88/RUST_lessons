fn main() {
    println!("Loop example!");
    // Цикл, который ничего не возвращает
    let mut n = 1;
    loop {
        println!("n = {}", n);
        n = n + 1;
        if n == 6 {
            break;
        }
    }
    println!("");

    println!("Loop and break example!");
    // Цикл, который возвращает значение
    let mut m = 1;
    let result = loop {
        println!("m = {}", m);
        m = m + 1;
        if m == 6 {
            break m * 2;
        }
    };
    println!("Результат цикла: {}", result);
    println!("");

    println!("While example!");
    // Цикл while
    let mut n = 1;
    while n < 6 {
        println!("n = {}", n);
        n = n + 1;
    }
    println!("");

    println!("For example!");
    // Цикл for
    for num in 1..6 {
        println!("num = {}", num);
    }
    println!("");

    println!("Nested loop example!");
    // Вложенный цикл
    let mut i = 1;
    let mut j = 1;
    while i < 6 {
        while j < 6 {
            print!("{}\t", i * j);
            j = j + 1;
        }
        println!();
        i = i + 1;
        j = 1;
    }
    println!("");

    println!("Loop with label example!");
    // Цикл с меткой
    let mut count = 0;
    'outer: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'outer;
            } // завершается внешний цикл
            remaining -= 1;
        }
        count += 1;
    }
    println!("Final count = {count}");
    println!("");

    println!("Конец программы");
}
