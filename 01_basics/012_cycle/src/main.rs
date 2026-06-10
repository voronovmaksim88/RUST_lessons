// ============================================================================
// Урок . 
// ============================================================================
// В Rust существует 3 основных типа циклов:
// 1. loop    - бесконечный цикл (выход через break)
// 2. while   - цикл с условием
// 3. for     - итераторный цикл
// ============================================================================

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║              Урок 012. ЦИКЛЫ В RUST                          ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // ========================================================================
    // 1. ЦИКЛ LOOP - БЕСКОНЕЧНЫЙ ЦИКЛ
    // ========================================================================
    // loop - это бесконечный цикл, который выполняется, пока не встретит break
    // Используется когда количество итераций заранее неизвестно
    
    println!("╔═══ 1.1 Простой loop с break ═══╗");
    let mut n = 1;
    loop {
        println!("  n = {}", n);
        n += 1;
        if n == 6 {
            break; // Выход из цикла
        }
    }
    println!("  Цикл завершён\n");

    // ========================================================================
    // 2. LOOP С ВОЗВРАТОМ ЗНАЧЕНИЯ
    // ========================================================================
    // loop может возвращать значение через break
    // Это уникальная особенность Rust!
    
    println!("╔═══ 1.2 Loop с возвратом значения ═══╗");
    let mut m = 1;
    let result = loop {
        println!("  m = {}", m);
        m += 1;
        if m == 6 {
            break m * 2; // Возвращаем значение m * 2
        }
    };
    println!("  Результат цикла: {}\n", result);

    // ========================================================================
    // 3. LOOP С CONTINUE
    // ========================================================================
    // continue пропускает текущую итерацию и переходит к следующей
    
    println!("╔═══ 1.3 Loop с continue (пропуск чётных) ═══╗");
    let mut counter = 0;
    loop {
        counter += 1;
        
        if counter > 10 {
            break; // Выходим при counter > 10
        }
        
        if counter % 2 == 0 {
            continue; // Пропускаем чётные числа
        }
        
        println!("  Нечётное число: {}", counter);
    }
    println!();

    // ========================================================================
    // 4. ЦИКЛ WHILE - ЦИКЛ С УСЛОВИЕМ
    // ========================================================================
    // while выполняется пока условие истинно
    // Проверка условия происходит ПЕРЕД каждой итерацией
    
    println!("╔═══ 2.1 Простой while ═══╗");
    let mut count = 1;
    while count < 6 {
        println!("  count = {}", count);
        count += 1;
    }
    println!();

    println!("╔═══ 2.2 While с несколькими условиями ═══╗");
    let mut x = 0;
    let mut y = 10;
    while x < 5 && y > 5 {
        println!("  x = {}, y = {}", x, y);
        x += 1;
        y -= 1;
    }
    println!();

    // ========================================================================
    // 5. WHILE LET - ЦИКЛ С СОПОСТАВЛЕНИЕМ ОБРАЗЦОВ
    // ========================================================================
    // while let - специальная конструкция для работы с Option и Result
    // Цикл продолжается пока pattern matching успешен
    
    println!("╔═══ 2.3 While let с Option ═══╗");
    let mut stack = vec![1, 2, 3, 4, 5];
    println!("  Исходный стек: {:?}", stack);
    println!("  Извлекаем элементы:");
    
    // pop() возвращает Option<T>: Some(value) или None
    while let Some(top) = stack.pop() {
        println!("    Извлечён элемент: {}", top);
    }
    println!("  Стек пуст: {:?}\n", stack);

    // ========================================================================
    // 6. ЦИКЛ FOR - ИТЕРАТОРНЫЙ ЦИКЛ
    // ========================================================================
    // for - самый безопасный и часто используемый цикл в Rust
    // Используется для итерации по коллекциям и диапазонам
    
    println!("╔═══ 3.1 For с диапазоном (range) ═══╗");
    println!("  Диапазон 1..6 (исключает 6):");
    for num in 1..6 {
        println!("    num = {}", num);
    }
    
    println!("  Диапазон 1..=5 (включает 5):");
    for num in 1..=5 {
        println!("    num = {}", num);
    }
    println!();

    println!("╔═══ 3.2 For в обратном порядке ═══╗");
    for num in (1..=5).rev() {
        println!("  Обратный отсчёт: {}", num);
    }
    println!("  Пуск! 🚀\n");

    println!("╔═══ 3.3 For с шагом (step) ═══╗");
    println!("  Чётные числа от 0 до 10:");
    for num in (0..=10).step_by(2) {
        println!("    {}", num);
    }
    println!();

    // ========================================================================
    // 7. FOR С МАССИВАМИ И ВЕКТОРАМИ
    // ========================================================================
    
    println!("╔═══ 3.4 For с массивом ═══╗");
    let array = [10, 20, 30, 40, 50];
    println!("  Массив: {:?}", array);
    for element in array {
        println!("    Элемент: {}", element);
    }
    println!();

    println!("╔═══ 3.5 For с вектором ═══╗");
    let vec = vec!["Rust", "Python", "JavaScript", "Go"];
    println!("  Вектор: {:?}", vec);
    for language in &vec {
        println!("    Язык: {}", language);
    }
    println!();

    // ========================================================================
    // 8. FOR С ENUMERATE - ИНДЕКС И ЗНАЧЕНИЕ
    // ========================================================================
    // enumerate() возвращает кортеж (индекс, значение)
    
    println!("╔═══ 3.6 For с enumerate (индекс + значение) ═══╗");
    let fruits = vec!["Яблоко", "Банан", "Апельсин", "Манго"];
    for (index, fruit) in fruits.iter().enumerate() {
        println!("  Фрукт #{}: {}", index + 1, fruit);
    }
    println!();

    // ========================================================================
    // 9. ITER(), ITER_MUT(), INTO_ITER()
    // ========================================================================
    // В Rust есть 3 способа итерации:
    // - iter()      - заимствование (иммутабельное)
    // - iter_mut()  - изменяемое заимствование
    // - into_iter() - передача владения (consume)
    
    println!("╔═══ 3.7 Iter() - неизменяемое заимствование ═══╗");
    let numbers = vec![1, 2, 3, 4, 5];
    println!("  До цикла: {:?}", numbers);
    for num in numbers.iter() {
        println!("    &num = {}", num);
    }
    println!("  После цикла numbers всё ещё доступен: {:?}\n", numbers);

    println!("╔═══ 3.8 Iter_mut() - изменяемое заимствование ═══╗");
    let mut numbers_mut = vec![1, 2, 3, 4, 5];
    println!("  До изменения: {:?}", numbers_mut);
    for num in numbers_mut.iter_mut() {
        *num *= 2; // Умножаем каждый элемент на 2
    }
    println!("  После изменения: {:?}\n", numbers_mut);

    println!("╔═══ 3.9 Into_iter() - передача владения ═══╗");
    let numbers_owned = vec![1, 2, 3, 4, 5];
    println!("  До цикла: {:?}", numbers_owned);
    for num in numbers_owned.into_iter() {
        println!("    Владение передано: {}", num);
    }
    // println!("{:?}", numbers_owned); // ОШИБКА! numbers_owned больше не существует
    println!("  После into_iter() вектор больше недоступен\n");

    // ========================================================================
    // 10. ВЛОЖЕННЫЕ ЦИКЛЫ
    // ========================================================================
    
    println!("╔═══ 4.1 Вложенные циклы - таблица умножения ═══╗");
    println!("  Таблица умножения 5x5:");
    print!("     ");
    for i in 1..=5 {
        print!("{:4}", i);
    }
    println!();
    print!("    ");
    for _ in 1..=5 {
        print!("────");
    }
    println!();
    
    for i in 1..=5 {
        print!("{:2} │", i);
        for j in 1..=5 {
            print!("{:4}", i * j);
        }
        println!();
    }
    println!();

    // ========================================================================
    // 11. МЕТКИ ЦИКЛОВ (LOOP LABELS)
    // ========================================================================
    // Метки позволяют управлять вложенными циклами
    // Синтаксис: 'label_name: loop { ... }
    
    println!("╔═══ 4.2 Метки циклов - break с меткой ═══╗");
    let mut count = 0;
    'outer: loop {
        println!("  Внешний цикл: count = {}", count);
        let mut remaining = 10;
        
        loop {
            println!("    Внутренний цикл: remaining = {}", remaining);
            
            if remaining == 9 {
                break; // Выход из внутреннего цикла
            }
            
            if count == 2 {
                println!("  Выход из внешнего цикла!");
                break 'outer; // Выход из внешнего цикла
            }
            
            remaining -= 1;
        }
        count += 1;
    }
    println!("  Final count = {}\n", count);

    println!("╔═══ 4.3 Метки циклов - continue с меткой ═══╗");
    let mut outer_count = 0;
    'outer_loop: loop {
        outer_count += 1;
        
        if outer_count > 3 {
            break;
        }
        
        println!("  Внешняя итерация #{}", outer_count);
        
        for inner in 1..=5 {
            if inner == 3 {
                println!("    Пропускаем внешнюю итерацию!");
                continue 'outer_loop; // Переходим к следующей итерации внешнего цикла
            }
            println!("    Внутреннее значение: {}", inner);
        }
    }
    println!();

    // ========================================================================
    // 12. ЦИКЛ С ВОЗВРАТОМ ЗНАЧЕНИЯ ИЗ МЕТОК
    // ========================================================================
    
    println!("╔═══ 4.4 Возврат значения из помеченного цикла ═══╗");
    let result = 'search: loop {
        for i in 1..=100 {
            if i * i == 64 {
                println!("  Найден квадратный корень из 64!");
                break 'search i; // Возвращаем значение из внешнего цикла
            }
        }
        break 0; // Если не найдено
    };
    println!("  Результат: {}\n", result);

    // ========================================================================
    // 13. ИТЕРАТОРЫ И ЦЕПОЧКИ МЕТОДОВ
    // ========================================================================
    
    println!("╔═══ 5.1 Итераторы с filter и map ═══╗");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("  Исходные числа: {:?}", numbers);
    
    println!("  Чётные числа × 2:");
    for num in numbers.iter().filter(|x| *x % 2 == 0).map(|x| x * 2) {
        print!("    {} ", num);
    }
    println!("\n");

    println!("╔═══ 5.2 Цикл for с фильтрацией ═══╗");
    let temperatures = vec![-5, 0, 3, 8, 15, 22, 18, 12, 5, -2];
    println!("  Положительные температуры:");
    for &temp in temperatures.iter().filter(|&&t| t > 0) {
        println!("    +{}°C", temp);
    }
    println!();

    println!("╔═══ 5.3 Consumer методы итераторов ═══╗");
    let numbers = vec![1, 2, 3, 4, 5];
    println!("  Исходные числа: {:?}", numbers);

    // collect - сбор в новую коллекцию
    println!("  collect - удвоение чисел в новый вектор:");
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("    Результат: {:?}", doubled);

    // sum - сумма всех элементов
    println!("  sum - сумма всех чисел:");
    let sum: i32 = numbers.iter().sum();
    println!("    Сумма: {}", sum);

    // product - произведение всех элементов
    println!("  product - произведение всех чисел:");
    let product: i32 = numbers.iter().product();
    println!("    Произведение: {}", product);

    // find - поиск первого элемента, удовлетворяющего условию
    println!("  find - поиск первого чётного числа:");
    let found = numbers.iter().find(|&&x| x % 2 == 0);
    println!("    Найдено: {:?}", found);

    // position - индекс первого найденного элемента
    println!("  position - индекс первого числа > 3:");
    let index = numbers.iter().position(|&x| x > 3);
    println!("    Индекс: {:?}", index);

    // any - проверка, есть ли элемент, удовлетворяющий условию
    println!("  any - есть ли чётные числа:");
    let has_even = numbers.iter().any(|&x| x % 2 == 0);
    println!("    Результат: {}", has_even);

    // all - проверка, все ли элементы удовлетворяют условию
    println!("  all - все ли числа положительные:");
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("    Результат: {}", all_positive);

    // fold - свёртка с накопителем
    println!("  fold - суммирование через накопитель:");
    let folded_sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    println!("    Сумма: {}", folded_sum);

    // reduce - свёртка без начального значения
    println!("  reduce - нахождение максимума:");
    let max_val = numbers.iter().reduce(|a, b| a.max(b));
    println!("    Максимум: {:?}", max_val);
    println!();

    println!("╔═══ 5.4 Продвинутые адаптеры итераторов ═══╗");
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec!['A', 'B', 'C', 'D', 'E'];
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // zip - объединение двух итераторов в пары
    println!("  zip - объединение чисел и букв:");
    for (num, ch) in vec1.iter().zip(vec2.iter()) {
        println!("    {} -> {}", num, ch);
    }
    println!();

    // take - взятие первых N элементов
    println!("  take(5) - первые 5 чисел:");
    for num in numbers.iter().take(5) {
        println!("    {}", num);
    }
    println!();

    // skip - пропуск первых N элементов
    println!("  skip(5) - пропустить первые 5 чисел:");
    for num in numbers.iter().skip(5) {
        println!("    {}", num);
    }
    println!();

    // windows - скользящие окна заданного размера
    println!("  windows(2) - пары соседних чисел:");
    let nums = vec![1, 2, 3, 4, 5];
    for window in nums.windows(2) {
        println!("    {:?}", window);
    }
    println!();

    // chunks - разбиение на фрагменты
    println!("  chunks(3) - разбиение на группы по 3:");
    for chunk in nums.chunks(3) {
        println!("    {:?}", chunk);
    }
    println!();

    // cycle - бесконечное повторение (с take для ограничения)
    println!("  cycle - повторение ABC (с ограничением 10):");
    let letters = vec!['A', 'B', 'C'];
    for ch in letters.iter().cycle().take(10) {
        print!("{} ", ch);
    }
    println!("\n");

    // chain - объединение двух итераторов
    println!("  chain - объединение двух векторов:");
    let first = vec![1, 2, 3];
    let second = vec![4, 5, 6];
    for num in first.iter().chain(second.iter()) {
        print!("{} ", num);
    }
    println!("\n");

    // flat_map - разворачивание вложенных структур
    println!("  flat_map - разворачивание вложенных векторов:");
    let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    for num in nested.iter().flat_map(|v| v.iter()) {
        print!("{} ", num);
    }
    println!("\n");

    // rev - обратный порядок
    println!("  rev - итерация в обратном порядке:");
    for num in vec1.iter().rev() {
        print!("{} ", num);
    }
    println!("\n");

    // enumerate - индекс + значение (уже был, но напоминание)
    println!("  enumerate - индекс и значение:");
    for (i, num) in vec1.iter().enumerate() {
        println!("    [{}] = {}", i, num);
    }
    println!();

    // ========================================================================
    // 14. РАБОТА С КОЛЛЕКЦИЯМИ
    // ========================================================================
    
    println!("╔═══ 5.5 Итерация по HashMap ═══╗");
    use std::collections::HashMap;
    
    let mut scores = HashMap::new();
    scores.insert("Алиса", 95);
    scores.insert("Боб", 87);
    scores.insert("Чарли", 92);
    
    println!("  Результаты экзамена:");
    for (name, score) in &scores {
        println!("    {} набрал(а) {} баллов", name, score);
    }
    println!();

    println!("╔═══ 5.6 Итерация по строкам (chars) ═══╗");
    let text = "Rust🦀";
    println!("  Строка: '{}'", text);
    println!("  Символы:");
    for (i, ch) in text.chars().enumerate() {
        println!("    [{}] = '{}'", i, ch);
    }
    println!();

    // ========================================================================
    // 15. ПРАКТИЧЕСКИЕ ПРИМЕРЫ
    // ========================================================================
    
    println!("╔═══ 6.1 Нахождение простых чисел ═══╗");
    let limit = 30;
    println!("  Простые числа до {}:", limit);
    print!("  ");
    
    'outer_prime: for num in 2..=limit {
        for divisor in 2..num {
            if num % divisor == 0 {
                continue 'outer_prime; // Не простое число
            }
        }
        print!("{} ", num);
    }
    println!("\n");

    println!("╔═══ 6.2 Факториал с циклом ═══╗");
    let n = 5;
    let mut factorial = 1u64;
    for i in 1..=n {
        factorial *= i;
    }
    println!("  {}! = {}\n", n, factorial);

    println!("╔═══ 6.3 Числа Фибоначчи ═══╗");
    let fib_count = 10;
    let (mut a, mut b) = (0u64, 1u64);
    println!("  Первые {} чисел Фибоначчи:", fib_count);
    print!("  ");
    
    for _ in 0..fib_count {
        print!("{} ", a);
        let temp = a + b;
        a = b;
        b = temp;
    }
    println!("\n");

    println!("╔═══ 6.4 Поиск в коллекции ═══╗");
    let names = vec!["Анна", "Борис", "Вера", "Григорий", "Дарья"];
    let search_name = "Вера";
    let mut found = false;
    
    for (index, name) in names.iter().enumerate() {
        if *name == search_name {
            println!("  Найдено: '{}' на позиции {}", name, index);
            found = true;
            break;
        }
    }
    
    if !found {
        println!("  '{}' не найдено", search_name);
    }
    println!();

    println!("╔═══ 6.5 Обработка с match внутри цикла ═══╗");
    let results = vec![Some(10), None, Some(20), Some(30), None, Some(40)];
    let mut sum = 0;
    
    for result in results {
        match result {
            Some(value) => {
                println!("  Найдено значение: {}", value);
                sum += value;
            }
            None => {
                println!("  Пропущено: None");
            }
        }
    }
    println!("  Сумма всех значений: {}\n", sum);

    // ========================================================================
    // 16. БЕСКОНЕЧНЫЕ ЦИКЛЫ И УСЛОВИЯ ВЫХОДА
    // ========================================================================
    
    println!("╔═══ 7.1 Симуляция игрового цикла ═══╗");
    let mut health = 100;
    let mut turn = 0;
    
    loop {
        turn += 1;
        health -= 15;
        
        println!("  Ход {}: Здоровье = {}", turn, health);
        
        if health <= 0 {
            println!("  Игра окончена! Продержались {} ходов", turn);
            break;
        }
        
        if turn >= 8 {
            println!("  Достигнут лимит ходов!");
            break;
        }
    }
    println!();

    // ========================================================================
    // РЕЗЮМЕ
    // ========================================================================
    
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║                         РЕЗЮМЕ                                 ║");
    println!("╠════════════════════════════════════════════════════════════════╣");
    println!("║ • loop       - бесконечный цикл, выход через break             ║");
    println!("║ • while      - цикл с условием                                 ║");
    println!("║ • while let  - цикл с pattern matching                         ║");
    println!("║ • for        - итерация по диапазонам и коллекциям             ║");
    println!("║ • break      - выход из цикла                                  ║");
    println!("║ • continue   - переход к следующей итерации                    ║");
    println!("║ • 'label:    - метки для управления вложенными циклами         ║");
    println!("║ • iter()     - неизменяемое заимствование                      ║");
    println!("║ • iter_mut() - изменяемое заимствование                        ║");
    println!("║ • into_iter()- передача владения                               ║");
    println!("╠════════════════════════════════════════════════════════════════╣");
    println!("║ Consumer методы:                                                ║");
    println!("║ • collect    - сбор в коллекцию                                ║");
    println!("║ • sum/product - агрегация значений                             ║");
    println!("║ • find       - поиск первого элемента                          ║");
    println!("║ • position   - индекс найденного элемента                      ║");
    println!("║ • any/all    - проверка условий                                ║");
    println!("║ • fold/reduce- свёртка с накопителем                           ║");
    println!("╠════════════════════════════════════════════════════════════════╣");
    println!("║ Адаптеры итераторов:                                            ║");
    println!("║ • zip        - объединение итераторов в пары                   ║");
    println!("║ • take/skip  - ограничение количества элементов                ║");
    println!("║ • windows    - скользящие окна соседних элементов              ║");
    println!("║ • chunks     - разбиение на фрагменты                          ║");
    println!("║ • cycle      - бесконечное повторение                          ║");
    println!("║ • chain      - объединение итераторов                          ║");
    println!("║ • flat_map   - разворачивание вложенных структур               ║");
    println!("║ • rev        - обратный порядок итерации                       ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    
    println!("\n✅ Урок по циклам в Rust завершён!");
}
