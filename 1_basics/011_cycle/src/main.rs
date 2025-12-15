fn main() {
    println!("Hello, world!");

    // Цикл, который ничего не возвращает
    let mut n = 1;
    loop {
        println!("n = {}", n);
        n = n + 1;
        if n == 10 {
            break;
        }
    }

    // Цикл, который возвращает значение
    let mut m = 1;
    let result = loop {
        println!("m = {}", m);
        m = m + 1;
        if m == 10 {
            break m * 2;
        }
    };
    println!("Результат цикла: {}", result);

    // Цикл while
    let mut n = 1;
    while n < 10 {
        println!("n = {}", n);
        n = n + 1;
    }

    // Цикл for
    for num in 1..6 {
        println!("num = {}", num);
    }

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

    println!("Конец программы");
}
