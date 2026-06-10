// ============================================================================
// Урок 030. REF, REF MUT И RANGE-ПАТТЕРНЫ
// ============================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║        Урок 030. REF, REF MUT И RANGE-ПАТТЕРНЫ              ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    // ============================================
    // 1. ПРОБЛЕМА: ДЕСТРУКТУРИЗАЦИЯ ПЕРЕМЕЩАЕТ ЗНАЧЕНИЕ
    // ============================================
    println!("\n1. Проблема: деструктуризация перемещает значение");

    // String не реализует трейт Copy, поэтому при захвате в паттерн
    // значение ПЕРЕМЕЩАЕТСЯ — после match оно уже недоступно.
    let name = String::from("Максим");

    match name {
        n => println!("   Привет, {n}! (n захватил String — name перемещён)"),
    }
    // println!("{name}"); // ОШИБКА: value used after move
    // Компилятор не позволит использовать name после match.

    // Для числовых типов (i32, u32 и т.д.) проблемы нет — они Copy:
    let code = 200_u32;
    match code {
        c => println!("   Код: {c} (u32 — Copy, code всё ещё жив: {code})"),
    }

    // Вывод: когда тип не Copy и значение нужно после match — используй ref.

    // ============================================
    // 2. ref В ПАТТЕРНЕ — СОЗДАЁМ ССЫЛКУ ВМЕСТО ПЕРЕМЕЩЕНИЯ
    // ============================================
    println!("\n2. ref в паттерне match");

    let name = String::from("Максим");

    match name {
        // ref n создаёт ссылку &String на name — перемещения не происходит.
        ref n => println!("   Привет, {n}! (n: &String)"),
    }
    println!("   name ещё жив: {name}"); // OK — name не был перемещён

    // ref в match с несколькими ветками:
    let greeting = String::from("Привет");
    match greeting {
        ref s if s.starts_with("Привет") => {
            println!("   Русское приветствие: {s}");
        }
        ref s if s.starts_with("Hello") => {
            println!("   English greeting: {s}");
        }
        ref s => println!("   Неизвестное: {s}"),
    }
    println!("   greeting после match: {greeting}"); // не перемещён

    // Три эквивалентных способа получить ссылку:
    println!("\n   Три способа получить &String:");
    let s = String::from("hello");
    let r1 = &s;           // обычная ссылка через &
    let ref r2 = s;        // ref в let — полностью эквивалентно &s
    println!("   &s          → r1 = {r1}");
    println!("   let ref r2  → r2 = {r2}");
    // Тип r1 и r2 одинаков: &String

    // ============================================
    // 3. ref mut — ИЗМЕНЯЕМАЯ ССЫЛКА В ПАТТЕРНЕ
    // ============================================
    println!("\n3. ref mut — изменяемая ссылка");

    let mut score = 42;

    println!("   score до match: {score}");
    match score {
        // ref mut n создаёт &mut i32 — можно изменить через разыменование *n
        ref mut n => {
            println!("   внутри match: *n = {n}");
            *n += 10; // разыменовываем и меняем
            println!("   после *n += 10: *n = {n}");
        }
    }
    println!("   score после match: {score}"); // 52 — изменение применилось

    // ref mut в кортеже:
    let mut point = (10, 20);
    let (ref mut x, ref mut y) = point;
    *x += 5;
    *y += 5;
    println!("\n   point после сдвига: ({}, {})", point.0, point.1);

    // ============================================
    // 4. ref В ДЕСТРУКТУРИЗАЦИИ СТРУКТУР — ЧАСТИЧНОЕ ЗАИМСТВОВАНИЕ
    // ============================================
    println!("\n4. ref в деструктуризации структур (частичное заимствование)");

    struct User {
        name: String,
        age: u32,
    }

    let user = User {
        name: String::from("Иван"),
        age: 30,
    };

    // ref name — берём ссылку на String, не перемещаем
    // age      — u32 реализует Copy, копируется без ref
    let User { ref name, age } = user;
    println!("   name: {name}, age: {age}");
    println!("   user.name всё ещё жив: {}", user.name); // OK

    // Без ref — перемещение делает user частично недоступным:
    // let User { name, age } = user;
    // println!("{}", user.name); // ОШИБКА: user.name перемещён

    // ref mut для изменения поля через деструктуризацию:
    let mut counter = (String::from("hits"), 0_u32);
    {
        let (ref label, ref mut count) = counter;
        *count += 1;
        println!("\n   {label}: {count}");
    }
    println!("   counter после: ({}, {})", counter.0, counter.1);

    // ============================================
    // 5. КЛЮЧЕВОЕ ОТЛИЧИЕ: ref x vs &x В ПАТТЕРНЕ
    // ============================================
    println!("\n5. ref x vs &x в паттерне — важное различие");

    // Главный вопрос: какой тип у значения, с которым сопоставляется паттерн?
    //
    //  Паттерн  │ Ожидает │ Что делает
    //  ─────────┼─────────┼──────────────────────────────────────
    //  ref x    │  T      │ создаёт ссылку &T, значение остаётся
    //  &x       │  &T     │ снимает ссылку, достаёт T из &T

    let val: i32 = 42;
    let ref_val: &i32 = &val;

    // &x — паттерн для снятия ссылки. Ожидает &T, даёт T:
    let &extracted = ref_val;   // extracted: i32
    println!("   &x паттерн: extracted = {extracted}  (тип i32, снята ссылка)");

    // ref x — паттерн для создания ссылки. Ожидает T, даёт &T:
    let ref borrowed = val;     // borrowed: &i32
    println!("   ref x паттерн: borrowed = {borrowed}  (тип &i32, создана ссылка)");

    // На практике в match с &T:
    let words = vec![
        String::from("один"),
        String::from("два"),
        String::from("три"),
    ];

    println!("\n   Итерация по &Vec<String>:");
    for word in &words {
        // word: &String — уже ссылка.
        // Можно использовать как &x (снять ссылку) или ref (лишнее — уже &T).
        match word {
            w if w.len() == 4 => println!("   '{w}' — 4 буквы"),
            w => println!("   '{w}'"),
        }
    }

    // ============================================
    // 6. MATCH ERGONOMICS (RUST 2018+) — КОГДА ref НЕ НУЖЕН
    // ============================================
    println!("\n6. Match ergonomics — автоматический ref");

    // С Rust 2018 компилятор сам добавляет ref когда ты сопоставляешь
    // &T с паттерном для T. Это называется match ergonomics.
    //
    // До Rust 2018 (старый стиль):
    //   for name in &names { match name { ref n => ... } }
    //
    // Rust 2018+ (современный стиль):
    //   for name in &names { match name { n => ... } }  ← n: &String автоматически

    let names = vec![
        String::from("Анна"),
        String::from("Борис"),
        String::from("Вера"),
    ];

    println!("   Современный стиль (match ergonomics):");
    for name in &names {
        // name: &String. Паттерн n автоматически получает тип &String.
        // ref писать НЕ нужно — компилятор добавит сам.
        match name {
            n if n.starts_with('А') => println!("   {n} — начинается с А"),
            n => println!("   {n}"),
        }
    }
    println!("   names после цикла: {:?}", names); // не перемещены

    // Когда ref всё ещё нужен явно:
    // — сопоставляешь T (не &T), но хочешь ссылку:
    let text = String::from("пример");
    let reference = match text {
        ref s => s as *const String, // нужен именно ref, &text недоступен в выражении
    };
    println!("\n   Явный ref нужен при сопоставлении T → *const: {:p}", reference);

    // ============================================
    // 7. RANGE-ПАТТЕРНЫ В ГЛУБИНУ
    // ============================================
    println!("\n7. Range-паттерны в глубину");

    // 7.1 Синтаксис и поддерживаемые типы
    println!("\n   7.1 Синтаксис:");
    println!("   a..=b — включающий диапазон (конец входит в диапазон)");
    println!("   a..b  — исключающий, тоже работает (стабилен с Rust 1.80)");

    let n: i32 = 7;
    let category = match n {
        i32::MIN..=-1 => "отрицательное",
        0             => "ноль",
        1..=9         => "однозначное положительное",
        10..=99       => "двузначное",
        _             => "трёхзначное и больше",
    };
    println!("   {n} → {category}");

    // 7.2 Какие типы поддерживают range-паттерны
    println!("\n   7.2 Поддерживаемые типы:");

    // Все целые числа (i8..i128, u8..u128, isize, usize)
    let byte: u8 = 200;
    let zone = match byte {
        0..=127   => "ASCII",
        128..=191 => "UTF-8 continuation byte",
        192..=255 => "UTF-8 leading byte",
    };
    println!("   byte={byte} → {zone}");

    // char
    let ch = 'Ж';
    let script = match ch {
        'A'..='Z' | 'a'..='z' => "латиница",
        'А'..='Я' | 'а'..='я' => "кириллица",
        '0'..='9'              => "цифра",
        _                      => "другой символ",
    };
    println!("   '{}' → {script}", ch);

    // f32, f64 — НЕ поддерживают range-паттерны (используй guard):
    let temp: f64 = 36.6;
    let status = match temp {
        t if t < 36.0              => "гипотермия",
        t if t <= 37.5             => "норма",
        t if t <= 38.5             => "субфебрильная",
        _                          => "высокая температура",
    };
    println!("   {temp}°C → {status}  (f64 — только через guard)");

    // 7.3 Комбинации range с | (ИЛИ)
    println!("\n   7.3 Range + | (объединение диапазонов):");
    let port: u16 = 8080;
    let port_type = match port {
        80 | 443               => "стандартный веб",
        8080 | 8443            => "альтернативный веб",
        20 | 21                => "FTP",
        1..=1023               => "системный (< 1024)",
        1024..=49151           => "зарегистрированный",
        49152..=65535          => "динамический/приватный",
        0                      => "зарезервирован (порт 0)",
    };
    println!("   порт {port} → {port_type}");

    // 7.4 Range + guard для исключений внутри диапазона
    println!("\n   7.4 Range + guard (исключение внутри диапазона):");
    for n in [1_u32, 13, 42, 66, 99] {
        let label = match n {
            1..=100 if n % 2 == 0 && n % 3 == 0 => "чётное и кратное 3",
            1..=100 if n % 2 == 0 => "чётное",
            1..=100 if n == 13 => "несчастливое нечётное",
            1..=100 => "обычное нечётное",
            _ => "вне диапазона",
        };
        println!("   {n:>3} → {label}");
    }

    // 7.5 Range + @-binding (из урока 029, в связке с ranges)
    println!("\n   7.5 Range + @-binding:");
    let response_code: u16 = 404;
    match response_code {
        code @ 100..=199 => println!("   [{code}] Информационный"),
        code @ 200..=299 => println!("   [{code}] Успех"),
        code @ 300..=399 => println!("   [{code}] Перенаправление"),
        code @ 400..=499 => println!("   [{code}] Ошибка клиента"),
        code @ 500..=599 => println!("   [{code}] Ошибка сервера"),
        code             => println!("   [{code}] Неизвестный статус"),
    }

    // 7.6 Отличие range-паттерна от Range-типа
    println!("\n   7.6 Паттерн vs тип Range:");

    // Это ПАТТЕРН в match — не создаёт объект, работает только внутри match/let/if let:
    let x = 5;
    let in_range = match x {
        1..=10 => true,
        _      => false,
    };
    println!("   match 1..=10 (паттерн):    {x} входит → {in_range}");

    // Это ЗНАЧЕНИЕ типа std::ops::RangeInclusive<i32> — полноценный объект:
    let range_obj = 1..=10_i32;
    println!("   (1..=10).contains(&{x})   (тип): → {}", range_obj.contains(&x));

    // ============================================
    // 8. ПРАКТИКА: ВАЛИДАТОР ДАННЫХ ПРОФИЛЯ
    // ============================================
    println!("\n8. Практика: валидатор профиля пользователя");

    #[derive(Debug)]
    struct Profile {
        username: String,
        age: u8,
        score: u32,
    }

    fn validate(profile: &Profile) -> String {
        // ref для String-полей — заимствуем, не перемещаем
        let Profile { ref username, age, score } = *profile;

        match (age, score) {
            (0, _) => {
                format!("   {username}: возраст не указан")
            }
            (a, _) if a < 18 => {
                format!("   {username}: слишком молодой ({a} лет)")
            }
            (_, s) if s > 100 => {
                format!("   {username}: score превышает максимум ({s})")
            }
            (18..=120, 0..=59)  => {
                format!("   {username}: зачёт не сдан (score={score})")
            }
            (18..=120, 60..=79) => {
                format!("   {username}: удовлетворительно (score={score})")
            }
            (18..=120, 80..=100) => {
                format!("   {username}: отлично (score={score})")
            }
            _ => format!("   {username}: некорректные данные"),
        }
    }

    let profiles = vec![
        Profile { username: String::from("alice"), age: 25, score: 92 },
        Profile { username: String::from("bob"),   age: 15, score: 70 },
        Profile { username: String::from("carol"), age: 30, score: 55 },
        Profile { username: String::from("dave"),  age: 40, score: 150 },
        Profile { username: String::from("eve"),   age: 0,  score: 80 },
    ];

    for profile in &profiles {
        println!("{}", validate(profile));
    }

    // ============================================
    // ИТОГИ УРОКА
    // ============================================
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                     ИТОГИ УРОКА                             ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  ref x    — создаёт &T, значение не перемещается            ║");
    println!("║  ref mut x — создаёт &mut T, можно изменить через *x        ║");
    println!("║  ref x  ≠  &x:  ref ждёт T и создаёт &T                    ║");
    println!("║              &x ждёт &T и снимает ссылку                    ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  Rust 2018+: match ergonomics — ref часто не нужен явно     ║");
    println!("║  (компилятор добавляет сам при сопоставлении &T с T)        ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  Range-паттерны: a..=b и a..b (с Rust 1.80) оба стабильны   ║");
    println!("║  Работают для целых чисел и char, НЕ для f32/f64            ║");
    println!("║  Комбинируются с |, if guard, @-binding                     ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    println!("\n✅ Урок по ref, ref mut и range-паттернам завершён!");
}
