// ============================================================
// Урок 026. VEC

fn main() {
    println!("┌──────────────────────────────────────────────────────────────────────┐");
    println!("│              Урок 026. ВЕКТОРЫ (VEC) В RUST             │");
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    // ========================================================================
    // 1. СОЗДАНИЕ ВЕКТОРОВ
    // ========================================================================
    println!("┌── 1.1 Создание с помощью Vec::new() ─────────────────────────────────┐");
    let mut empty_vec: Vec<i32> = Vec::new();
    println!("  пустой вектор: {:?}", empty_vec);
    empty_vec.push(1);
    empty_vec.push(2);
    println!("  после добавления: {:?}", empty_vec);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 1.2 Создание с помощью макроса vec! ───────────────────────────────┐");
    let numbers = vec![1, 2, 3, 5, 8, 13, 21];
    println!("  numbers = {:?}", numbers);

    let zeros = vec![0; 5]; // 5 элементов со значением 0
    println!("  zeros = {:?}", zeros);

    let strings = vec![String::from("Tom"), String::from("Bob")];
    println!("  strings = {:?}", strings);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 1.3 Создание с указанием capacity ─────────────────────────────────┐");
    let mut with_capacity = Vec::with_capacity(10);
    println!(
        "  длина: {}, вместимость: {}",
        with_capacity.len(),
        with_capacity.capacity()
    );

    with_capacity.push(100);
    println!(
        "  после push: длина: {}, вместимость: {}",
        with_capacity.len(),
        with_capacity.capacity()
    );
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    // ========================================================================
    // 2. ДОБАВЛЕНИЕ И УДАЛЕНИЕ ЭЛЕМЕНТОВ
    // ========================================================================
    println!("┌── 2.1 Добавление элементов: push() ──────────────────────────────────┐");
    let mut fruits = vec!["яблоко", "банан"];
    println!("  начальное состояние: {:?}", fruits);

    fruits.push("апельсин");
    println!("  после push('апельсин'): {:?}", fruits);

    fruits.push("груша");
    println!("  после push('груша'): {:?}", fruits);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 2.2 Удаление элементов: pop() ─────────────────────────────────────┐");
    let mut stack = vec![1, 2, 3, 4, 5];
    println!("  начало: {:?}", stack);

    let last = stack.pop();
    println!("  pop() вернул: {:?}, вектор: {:?}", last, stack);

    let another = stack.pop();
    println!("  pop() вернул: {:?}, вектор: {:?}", another, stack);

    let mut empty_stack: Vec<i32> = vec![];
    let nothing = empty_stack.pop();
    println!("  pop() из пустого вектора: {:?}", nothing);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 2.3 Удаление по индексу: remove() ─────────────────────────────────┐");
    let mut colors = vec!["красный", "зелёный", "синий", "жёлтый"];
    println!("  до remove: {:?}", colors);

    let removed = colors.remove(1); // удаляет "зелёный"
    println!("  удалён: '{}', вектор: {:?}", removed, colors);
    println!("  ⚠️  remove() сдвигает все элементы после удалённого (O(n))");
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 2.4 Вставка элемента: insert() ────────────────────────────────────┐");
    let mut list = vec![1, 2, 4, 5];
    println!("  до insert: {:?}", list);

    list.insert(2, 3); // вставляет 3 на позицию 2
    println!("  после insert(2, 3): {:?}", list);
    println!("  ⚠️  insert() также сдвигает элементы (O(n))");
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 2.5 Очистка вектора: clear() ──────────────────────────────────────┐");
    let mut data = vec![1, 2, 3, 4, 5];
    println!("  до clear: {:?}, len: {}", data, data.len());

    data.clear();
    println!("  после clear: {:?}, len: {}", data, data.len());
    println!("  capacity остаётся: {}", data.capacity());
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 2.6 Добавление нескольких элементов: append() и extend() ──────────┐");
    let mut vec1 = vec![1, 2, 3];
    let mut vec2 = vec![4, 5, 6];
    println!("  vec1: {:?}, vec2: {:?}", vec1, vec2);

    vec1.append(&mut vec2);
    println!("  после append: vec1: {:?}, vec2: {:?}", vec1, vec2);
    println!("  ⚠️  append() опустошает второй вектор");

    let mut vec3 = vec![7, 8];
    let vec4 = vec![9, 10];
    vec3.extend(&vec4);
    println!("  после extend: vec3: {:?}, vec4: {:?}", vec3, vec4);
    println!("  ℹ️  extend() копирует элементы, не опустошая источник");
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 2.7 Быстрое удаление: swap_remove() ───────────────────────────────┐");
    let mut fast_remove = vec!["a", "b", "c", "d", "e"];
    println!("  до swap_remove: {:?}", fast_remove);

    let removed_item = fast_remove.swap_remove(1); // удаляет "b"
    println!("  удалён: '{}', вектор: {:?}", removed_item, fast_remove);
    println!("  ℹ️  swap_remove() меняет местами с последним и удаляет (O(1))");
    println!("  ⚠️  не сохраняет порядок элементов!");
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    // ========================================================================
    // 3. ДОСТУП К ЭЛЕМЕНТАМ
    // ========================================================================
    println!("┌── 3.1 Доступ по индексу [] ──────────────────────────────────────────┐");
    let users = vec!["Tom", "Bob", "Sam"];
    println!("  users[0] = {}", users[0]);
    println!("  users[2] = {}", users[2]);
    // users[10]; // ПАНИКА! Выход за границы
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 3.2 Безопасный доступ: get() ──────────────────────────────────────┐");
    let safe_vec = vec![10, 20, 30, 40];

    match safe_vec.get(2) {
        Some(value) => println!("  safe_vec[2] = {}", value),
        None => println!("  Индекс вне границ"),
    }

    match safe_vec.get(10) {
        Some(value) => println!("  safe_vec[10] = {}", value),
        None => println!("  safe_vec[10]: Индекс вне границ ❌"),
    }

    if let Some(first) = safe_vec.get(0) {
        println!("  первый элемент: {}", first);
    }
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 3.3 Изменение элементов ───────────────────────────────────────────┐");
    let mut mutable = vec![1, 2, 3, 4, 5];
    println!("  до изменения: {:?}", mutable);

    mutable[0] = 100;
    mutable[4] = 500;
    println!("  после изменения: {:?}", mutable);

    if let Some(elem) = mutable.get_mut(2) {
        *elem = 300;
    }
    println!("  после get_mut: {:?}", mutable);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 3.4 Первый и последний элементы ───────────────────────────────────┐");
    let sequence = vec![100, 200, 300, 400];
    println!("  first(): {:?}", sequence.first());
    println!("  last(): {:?}", sequence.last());

    let empty: Vec<i32> = vec![];
    println!("  first() в пустом: {:?}", empty.first());
    println!("  last() в пустом: {:?}", empty.last());
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    // ========================================================================
    // 4. ИТЕРИРОВАНИЕ
    // ========================================================================
    println!("┌── 4.1 Простой перебор (по ссылке) ───────────────────────────────────┐");
    let names = vec!["Алиса", "Боб", "Чарли"];
    print!("  ");
    for name in &names {
        print!("{} ", name);
    }
    println!();
    println!("  вектор после перебора: {:?}", names);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 4.2 Перебор с потреблением (без &) ────────────────────────────────┐");
    let consumable = vec![String::from("A"), String::from("B")];
    println!("  до перебора: {:?}", consumable);

    for item in consumable {
        println!("    обработка: {}", item);
    }
    // println!("{:?}", consumable); // ОШИБКА! Вектор потреблён
    println!("  ⚠️  После перебора без & вектор недоступен");
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 4.3 Изменяющий перебор: iter_mut() ────────────────────────────────┐");
    let mut values = vec![1, 2, 3, 4, 5];
    println!("  до изменения: {:?}", values);

    for val in &mut values {
        *val *= 2;
    }
    println!("  после удвоения: {:?}", values);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 4.4 Перебор с индексами: enumerate() ──────────────────────────────┐");
    let items = vec!["первый", "второй", "третий"];
    for (index, item) in items.iter().enumerate() {
        println!("  [{}] = {}", index, item);
    }
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 4.5 Обратный перебор: rev() ───────────────────────────────────────┐");
    let countdown = vec![5, 4, 3, 2, 1];
    print!("  обратный порядок: ");
    for n in countdown.iter().rev() {
        print!("{} ", n);
    }
    println!();
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    // ========================================================================
    // 5. СРЕЗЫ (SLICES)
    // ========================================================================
    println!("┌── 5.1 Создание срезов ───────────────────────────────────────────────┐");
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let slice = &vec[2..5];
    println!("  вектор: {:?}", vec);
    println!("  срез [2..5]: {:?}", slice);

    let full_slice = &vec[..];
    println!("  полный срез [..]: {:?}", full_slice);

    let from_start = &vec[..4];
    println!("  срез [..4]: {:?}", from_start);

    let to_end = &vec[3..];
    println!("  срез [3..]: {:?}", to_end);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 5.2 Изменяемые срезы ──────────────────────────────────────────────┐");
    let mut vec_mut = vec![10, 20, 30, 40, 50];
    println!("  до изменения: {:?}", vec_mut);

    let slice_mut = &mut vec_mut[1..4];
    slice_mut[0] = 200;
    slice_mut[2] = 400;
    println!("  после изменения среза: {:?}", vec_mut);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    // ========================================================================
    // 6. CAPACITY И ПРОИЗВОДИТЕЛЬНОСТЬ
    // ========================================================================
    println!("┌── 6.1 Length vs Capacity ────────────────────────────────────────────┐");
    let mut perf_vec: Vec<i32> = Vec::new();
    println!("  создан пустой Vec:");
    println!(
        "    len: {}, capacity: {}",
        perf_vec.len(),
        perf_vec.capacity()
    );

    perf_vec.push(1);
    println!("  после 1 push:");
    println!(
        "    len: {}, capacity: {}",
        perf_vec.len(),
        perf_vec.capacity()
    );

    perf_vec.push(2);
    println!("  после 2 push:");
    println!(
        "    len: {}, capacity: {}",
        perf_vec.len(),
        perf_vec.capacity()
    );

    perf_vec.push(3);
    println!("  после 3 push:");
    println!(
        "    len: {}, capacity: {}",
        perf_vec.len(),
        perf_vec.capacity()
    );

    println!();
    println!("  💡 Vec автоматически увеличивает capacity при необходимости");
    println!("  💡 Обычно capacity удваивается при реаллокации");
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 6.2 Управление capacity ───────────────────────────────────────────┐");
    let mut optimized: Vec<i32> = Vec::with_capacity(100);
    println!(
        "  with_capacity(100): len={}, cap={}",
        optimized.len(),
        optimized.capacity()
    );

    optimized.reserve(50); // резервирует ещё места
    println!(
        "  после reserve(50): len={}, cap={}",
        optimized.len(),
        optimized.capacity()
    );

    let mut shrinkable = vec![1, 2, 3, 4, 5];
    shrinkable.reserve(100);
    println!(
        "  перед shrink_to_fit: len={}, cap={}",
        shrinkable.len(),
        shrinkable.capacity()
    );

    shrinkable.shrink_to_fit();
    println!(
        "  после shrink_to_fit: len={}, cap={}",
        shrinkable.len(),
        shrinkable.capacity()
    );
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    // ========================================================================
    // 7. МЕТОДЫ ПОИСКА И ПРОВЕРКИ
    // ========================================================================
    println!("┌── 7.1 Проверка содержимого: contains() ──────────────────────────────┐");
    let search_vec = vec!["яблоко", "банан", "апельсин"];
    println!("  есть 'банан': {}", search_vec.contains(&"банан"));
    println!("  есть 'груша': {}", search_vec.contains(&"груша"));
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 7.2 Поиск элементов ───────────────────────────────────────────────┐");
    let numbers_search = vec![10, 20, 30, 40, 50];

    let position = numbers_search.iter().position(|&x| x == 30);
    println!("  позиция элемента 30: {:?}", position);

    let not_found = numbers_search.iter().position(|&x| x == 100);
    println!("  позиция элемента 100: {:?}", not_found);

    let first_big = numbers_search.iter().find(|&&x| x > 25);
    println!("  первый элемент > 25: {:?}", first_big);

    let count = numbers_search.iter().filter(|&&x| x >= 30).count();
    println!("  количество >= 30: {}", count);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 7.3 Проверки: all(), any(), is_empty() ────────────────────────────┐");
    let check_vec = vec![2, 4, 6, 8, 10];

    println!("  все чётные: {}", check_vec.iter().all(|&x| x % 2 == 0));
    println!("  есть > 5: {}", check_vec.iter().any(|&x| x > 5));
    println!("  пустой: {}", check_vec.is_empty());

    let empty_check: Vec<i32> = vec![];
    println!("  пустой вектор is_empty: {}", empty_check.is_empty());
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    // ========================================================================
    // 8. СОРТИРОВКА И МОДИФИКАЦИЯ
    // ========================================================================
    println!("┌── 8.1 Сортировка: sort() и sort_by() ────────────────────────────────┐");
    let mut unsorted = vec![5, 2, 8, 1, 9, 3];
    println!("  до сортировки: {:?}", unsorted);

    unsorted.sort();
    println!("  после sort(): {:?}", unsorted);

    let mut names_sort = vec!["Чарли", "Алиса", "Боб"];
    names_sort.sort();
    println!("  имена отсортированы: {:?}", names_sort);

    let mut descending = vec![1, 5, 3, 9, 2];
    descending.sort_by(|a, b| b.cmp(a)); // обратный порядок
    println!("  по убыванию: {:?}", descending);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 8.2 Разворот: reverse() ───────────────────────────────────────────┐");
    let mut reversible = vec![1, 2, 3, 4, 5];
    println!("  до reverse: {:?}", reversible);
    reversible.reverse();
    println!("  после reverse: {:?}", reversible);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 8.3 Дедупликация: dedup() ─────────────────────────────────────────┐");
    let mut duplicates = vec![1, 2, 2, 3, 3, 3, 4, 5, 5];
    println!("  до dedup: {:?}", duplicates);
    duplicates.dedup();
    println!("  после dedup: {:?}", duplicates);
    println!("  ⚠️  dedup() удаляет только последовательные дубликаты");
    println!("  💡 Для всех дубликатов используйте sort() + dedup()");

    let mut all_dups = vec![3, 1, 2, 1, 3, 2, 1];
    all_dups.sort();
    all_dups.dedup();
    println!("  после sort+dedup: {:?}", all_dups);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 8.4 Обмен элементов: swap() ───────────────────────────────────────┐");
    let mut swappable = vec![10, 20, 30, 40];
    println!("  до swap: {:?}", swappable);
    swappable.swap(0, 3);
    println!("  после swap(0, 3): {:?}", swappable);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    // ========================================================================
    // 9. РАЗДЕЛЕНИЕ И ОБЪЕДИНЕНИЕ
    // ========================================================================
    println!("┌── 9.1 Разделение: split_off() ───────────────────────────────────────┐");
    let mut original = vec![1, 2, 3, 4, 5, 6];
    println!("  исходный: {:?}", original);

    let second_half = original.split_off(3);
    println!("  после split_off(3):");
    println!("    первая часть: {:?}", original);
    println!("    вторая часть: {:?}", second_half);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 9.2 Слияние векторов ──────────────────────────────────────────────┐");
    let mut part1 = vec![1, 2, 3];
    let mut part2 = vec![4, 5, 6];

    part1.append(&mut part2);
    println!("  после append: {:?}", part1);
    println!("  part2 теперь пуст: {:?}", part2);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    // ========================================================================
    // 10. ПРЕОБРАЗОВАНИЯ
    // ========================================================================
    println!("┌── 10.1 Vec → Array ──────────────────────────────────────────────────┐");
    let vec_to_arr = vec![1, 2, 3, 4, 5];

    // Через try_into (безопасно)
    let arr_result: Result<[i32; 5], _> = vec_to_arr.clone().try_into();
    match arr_result {
        Ok(arr) => println!("  преобразование успешно: {:?}", arr),
        Err(_) => println!("  ошибка преобразования"),
    }

    // Через as_slice
    let slice = vec_to_arr.as_slice();
    println!("  как срез: {:?}", slice);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 10.2 Array → Vec ──────────────────────────────────────────────────┐");
    let array = [1, 2, 3, 4, 5];

    let vec_from_arr = array.to_vec();
    println!("  массив: {:?}", array);
    println!("  вектор: {:?}", vec_from_arr);

    let vec_from_slice = (&array[1..4]).to_vec();
    println!("  из среза [1..4]: {:?}", vec_from_slice);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 10.3 Преобразование типов: map() и collect() ──────────────────────┐");
    let numbers_map = vec![1, 2, 3, 4, 5];

    let doubled: Vec<i32> = numbers_map.iter().map(|x| x * 2).collect();
    println!("  удвоенные: {:?}", doubled);

    let strings_to_ints = vec!["1", "2", "3"];
    let parsed: Vec<i32> = strings_to_ints
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    println!("  строки → числа: {:?}", parsed);

    let to_strings: Vec<String> = numbers_map.iter().map(|n| format!("число_{}", n)).collect();
    println!("  числа → строки: {:?}", to_strings);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    // ========================================================================
    // 11. РАБОТА С ФУНКЦИЯМИ
    // ========================================================================
    println!("┌── 11.1 Передача вектора по ссылке ───────────────────────────────────┐");

    fn print_vec(vec: &Vec<i32>) {
        print!("  вектор: ");
        for item in vec {
            print!("{} ", item);
        }
        println!();
    }

    fn sum_vec(vec: &[i32]) -> i32 {
        vec.iter().sum()
    }

    let func_vec = vec![1, 2, 3, 4, 5];
    print_vec(&func_vec);
    println!("  сумма: {}", sum_vec(&func_vec));
    println!("  вектор доступен: {:?}", func_vec);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 11.2 Возврат вектора из функции ───────────────────────────────────┐");

    fn create_range(n: usize) -> Vec<i32> {
        (0..n as i32).collect()
    }

    fn filter_even(vec: Vec<i32>) -> Vec<i32> {
        vec.into_iter().filter(|x| x % 2 == 0).collect()
    }

    let range = create_range(10);
    println!("  созданный диапазон: {:?}", range);

    let evens = filter_even(range);
    println!("  только чётные: {:?}", evens);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 11.3 Изменение вектора в функции ──────────────────────────────────┐");

    fn double_values(vec: &mut Vec<i32>) {
        for item in vec.iter_mut() {
            *item *= 2;
        }
    }

    fn add_value(vec: &mut Vec<i32>, value: i32) {
        vec.push(value);
    }

    let mut modifiable = vec![1, 2, 3];
    println!("  до изменения: {:?}", modifiable);

    double_values(&mut modifiable);
    println!("  после удвоения: {:?}", modifiable);

    add_value(&mut modifiable, 100);
    println!("  после добавления: {:?}", modifiable);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    // ========================================================================
    // 12. ПРОДВИНУТЫЕ ТЕХНИКИ
    // ========================================================================
    println!("┌── 12.1 Заполнение с помощью repeat() и collect() ────────────────────┐");
    let repeated: Vec<i32> = std::iter::repeat(42).take(5).collect();
    println!("  repeat(42).take(5): {:?}", repeated);

    let range_vec: Vec<i32> = (1..=10).collect();
    println!("  диапазон (1..=10): {:?}", range_vec);

    let step_vec: Vec<i32> = (0..20).step_by(3).collect();
    println!("  с шагом step_by(3): {:?}", step_vec);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 12.2 Фильтрация: filter() и retain() ──────────────────────────────┐");
    let original_filter = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // filter() - создаёт новый вектор
    let evens_filter: Vec<i32> = original_filter
        .iter()
        .filter(|&&x| x % 2 == 0)
        .copied()
        .collect();
    println!("  исходный: {:?}", original_filter);
    println!("  только чётные (filter): {:?}", evens_filter);

    // retain() - изменяет существующий вектор
    let mut retain_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    retain_vec.retain(|&x| x % 2 == 0);
    println!("  после retain: {:?}", retain_vec);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 12.3 Работа с результатами: partition() ───────────────────────────┐");
    let numbers_part = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let (evens_part, odds): (Vec<i32>, Vec<i32>) =
        numbers_part.into_iter().partition(|&x| x % 2 == 0);

    println!("  чётные: {:?}", evens_part);
    println!("  нечётные: {:?}", odds);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 12.4 Объединение: flatten() ───────────────────────────────────────┐");
    let nested = vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]];
    let flattened: Vec<i32> = nested.into_iter().flatten().collect();
    println!("  объединённый: {:?}", flattened);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 12.5 Агрегация: sum(), product(), min(), max() ────────────────────┐");
    let stats_vec = vec![2, 4, 6, 8, 10];

    let sum: i32 = stats_vec.iter().sum();
    let product: i32 = stats_vec.iter().product();
    let min = stats_vec.iter().min();
    let max = stats_vec.iter().max();

    println!("  вектор: {:?}", stats_vec);
    println!("  сумма: {}", sum);
    println!("  произведение: {}", product);
    println!("  минимум: {:?}", min);
    println!("  максимум: {:?}", max);
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    println!("┌── 12.6 Группировка: chunks() и windows() ────────────────────────────┐");
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];

    println!("  chunks(3) - разбиение на части:");
    for chunk in data.chunks(3) {
        println!("    {:?}", chunk);
    }

    println!("  windows(3) - скользящее окно:");
    for window in data.windows(3) {
        println!("    {:?}", window);
    }
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    // ========================================================================
    // 13. VEC VS ARRAY
    // ========================================================================
    println!("┌── 13. Vec vs Array: основные различия ───────────────────────────────┐");
    println!("  ╔══════════════════╦═══════════════════════════════════════════════╗");
    println!("  ║  Vec<T>          ║  Array [T; N]                                 ║");
    println!("  ╠══════════════════╬═══════════════════════════════════════════════╣");
    println!("  ║ • динамический   ║ • фиксированный размер                        ║");
    println!("  ║   размер         ║                                               ║");
    println!("  ║ • данные на куче ║ • данные на стеке                             ║");
    println!("  ║ • можно изменять ║ • размер в типе [T; N]                        ║");
    println!("  ║   размер         ║                                               ║");
    println!("  ║ • push/pop/      ║ • только изменение элементов                  ║");
    println!("  ║   insert/remove  ║                                               ║");
    println!("  ║ • overhead для   ║ • минимальный overhead                        ║");
    println!("  ║   управления     ║                                               ║");
    println!("  ║   памятью        ║                                               ║");
    println!("  ║ • гибкость       ║ • скорость и предсказуемость                  ║");
    println!("  ╚══════════════════╩═══════════════════════════════════════════════╝");
    println!();
    println!("  💡 Используйте Vec, когда:");
    println!("     - размер данных неизвестен заранее");
    println!("     - нужно добавлять/удалять элементы");
    println!("     - работаете с динамическими данными");
    println!("     - размер может меняться в runtime");
    println!();
    println!("  💡 Используйте Array, когда:");
    println!("     - размер фиксирован и известен");
    println!("     - данные помещаются на стек");
    println!("     - критична производительность");
    println!("     - не нужно изменять размер");
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    // ========================================================================
    // 14. МНОГОМЕРНЫЕ ВЕКТОРА
    // ========================================================================
    println!("┌── 14. Многомерные вектора ───────────────────────────────────────────┐");
    let matrix: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    println!("  матрица 3x3:");
    for row in &matrix {
        println!("    {:?}", row);
    }

    println!("  доступ к элементу [1][2]: {}", matrix[1][2]);

    // Динамическое создание матрицы
    let mut dynamic_matrix: Vec<Vec<i32>> = Vec::new();
    for i in 0..3 {
        let mut row = Vec::new();
        for j in 0..4 {
            row.push((i * 4 + j) as i32);
        }
        dynamic_matrix.push(row);
    }

    println!("  динамическая матрица 3x4:");
    for row in &dynamic_matrix {
        println!("    {:?}", row);
    }
    println!("└──────────────────────────────────────────────────────────────────────┘\n");

    // ========================================================================
    // РЕЗЮМЕ
    // ========================================================================
    println!("┌──────────────────────────────────────────────────────────────────────┐");
    println!("│                                РЕЗЮМЕ                                │");
    println!("├──────────────────────────────────────────────────────────────────────┤");
    println!("│ СОЗДАНИЕ:                                                            │");
    println!("│ • Vec::new()       - пустой вектор                                   │");
    println!("│ • vec![...]        - макрос для инициализации                        │");
    println!("│ • vec![val; n]     - n копий значения                                │");
    println!("│ • Vec::with_capacity(n) - с предварительным резервированием          │");
    println!("│                                                                      │");
    println!("│ ДОБАВЛЕНИЕ/УДАЛЕНИЕ:                                                 │");
    println!("│ • .push(val)       - добавить в конец                                │");
    println!("│ • .pop()           - удалить с конца (возвращает Option)             │");
    println!("│ • .insert(i, val)  - вставить на позицию i                           │");
    println!("│ • .remove(i)       - удалить элемент i                               │");
    println!("│ • .clear()         - очистить вектор                                 │");
    println!("│ • .append(&mut v)  - добавить другой вектор                          │");
    println!("│ • .extend(iter)    - добавить элементы из итератора                  │");
    println!("│                                                                      │");
    println!("│ ДОСТУП:                                                              │");
    println!("│ • vec[i]           - прямой доступ (паника при выходе за границы)    │");
    println!("│ • .get(i)          - безопасный доступ (возвращает Option)           │");
    println!("│ • .get_mut(i)      - изменяемый доступ                               │");
    println!("│ • .first() / .last() - первый/последний элемент                      │");
    println!("│                                                                      │");
    println!("│ ИТЕРИРОВАНИЕ:                                                        │");
    println!("│ • for item in &vec     - по ссылкам                                  │");
    println!("│ • for item in vec      - с потреблением                              │");
    println!("│ • for item in &mut vec - изменяемое                                  │");
    println!("│ • .iter() / .iter_mut() - создание итераторов                        │");
    println!("│ • .enumerate()     - с индексами                                     │");
    println!("│ • .rev()           - в обратном порядке                              │");
    println!("│                                                                      │");
    println!("│ СРЕЗЫ:                                                               │");
    println!("│ • &vec[a..b]       - срез от a до b (не включая b)                   │");
    println!("│ • &vec[..n]        - первые n элементов                              │");
    println!("│ • &vec[n..]        - от n до конца                                   │");
    println!("│ • &vec[..]         - весь вектор как срез                            │");
    println!("│                                                                      │");
    println!("│ CAPACITY:                                                            │");
    println!("│ • .len()           - количество элементов                            │");
    println!("│ • .capacity()      - выделенная вместимость                          │");
    println!("│ • .reserve(n)      - зарезервировать место                           │");
    println!("│ • .shrink_to_fit() - уменьшить capacity до len                       │");
    println!("│                                                                      │");
    println!("│ ПОИСК И ПРОВЕРКА:                                                    │");
    println!("│ • .contains(&val)  - проверка наличия элемента                       │");
    println!("│ • .is_empty()      - проверка на пустоту                             │");
    println!("│ • .position(pred)  - индекс первого совпадения                       │");
    println!("│ • .find(pred)      - первый элемент по условию                       │");
    println!("│ • .all(pred)       - все элементы удовлетворяют условию              │");
    println!("│ • .any(pred)       - хотя бы один элемент удовлетворяет              │");
    println!("│                                                                      │");
    println!("│ МОДИФИКАЦИЯ:                                                         │");
    println!("│ • .sort()          - сортировка                                      │");
    println!("│ • .sort_by(cmp)    - сортировка с компаратором                       │");
    println!("│ • .reverse()       - разворот вектора                                │");
    println!("│ • .dedup()         - удаление последовательных дубликатов            │");
    println!("│ • .swap(i, j)      - обмен элементов                                 │");
    println!("│ • .retain(pred)    - оставить только элементы по условию             │");
    println!("│                                                                      │");
    println!("│ ПРЕОБРАЗОВАНИЯ:                                                      │");
    println!("│ • .map(f)          - преобразование элементов                        │");
    println!("│ • .filter(pred)    - фильтрация элементов                            │");
    println!("│ • .collect()       - сбор итератора в Vec                            │");
    println!("│ • .flatten()       - объединение вложенных векторов                  │");
    println!("│ • .partition(pred) - разделение на два вектора                       │");
    println!("│                                                                      │");
    println!("│ АГРЕГАЦИЯ:                                                           │");
    println!("│ • .sum()           - сумма элементов                                 │");
    println!("│ • .product()       - произведение элементов                          │");
    println!("│ • .min() / .max()  - минимум/максимум                                │");
    println!("│                                                                      │");
    println!("│ РАЗДЕЛЕНИЕ:                                                          │");
    println!("│ • .split_off(i)    - разделить на две части                          │");
    println!("│ • .chunks(n)       - итератор по кускам размера n                    │");
    println!("│ • .windows(n)      - скользящее окно размера n                       │");
    println!("│                                                                      │");
    println!("│ ОСОБЕННОСТИ:                                                         │");
    println!("│ • Vec владеет своими данными                                         │");
    println!("│ • Автоматическое управление памятью                                  │");
    println!("│ • Реаллокация при превышении capacity                                │");
    println!("│ • Непрерывное размещение в памяти (cache-friendly)                   │");
    println!("│ • O(1) для push/pop, O(n) для insert/remove                          │");
    println!("└──────────────────────────────────────────────────────────────────────┘");

    println!("\n✅ Урок по векторам (Vec) в Rust завершён!");
    println!("💡 Vec - это основная динамическая коллекция в Rust!");
}
