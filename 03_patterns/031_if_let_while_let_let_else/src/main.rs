// ============================================================================
// Урок 031. IF LET, WHILE LET И LET...ELSE
// ============================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║      Урок 031. IF LET, WHILE LET И LET...ELSE               ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    // ============================================
    // 1. ПРОБЛЕМА: MATCH ДЛЯ ОДНОГО ВАРИАНТА — МНОГОСЛОВНО
    // ============================================
    println!("\n1. Проблема: match для одного варианта громоздок");

    #[derive(Debug)]
    enum Direction { North, South, East, West }

    let dir = Direction::North;

    // Хотим обработать только North — остальные нам не интересны.
    // С match приходится писать пустую ветку _ => {}:
    match dir {
        Direction::North => println!("   [match] Идём на север"),
        _ => {} // раздражающая пустая ветка
    }

    // if let решает эту проблему: пустая ветка не нужна.
    if let Direction::North = dir {
        println!("   [if let] Идём на север — чище!");
    }

    // ============================================
    // 2. IF LET — ПАТТЕРН ДЛЯ ОДНОЙ ВЕТКИ
    // ============================================
    println!("\n2. if let — синтаксис и разные виды паттернов");

    // Синтаксис: if let ПАТТЕРН = ВЫРАЖЕНИЕ { ... }
    // Выполняется только если паттерн СОВПАЛ.
    // Переменные, связанные в паттерне, доступны внутри блока.

    // --- enum с данными ---
    #[derive(Debug)]
    enum Shape {
        Circle(f64),
        Rectangle(f64, f64),
        Triangle,
    }

    let shape = Shape::Circle(5.0);

    if let Shape::Circle(r) = shape {
        println!("   Круг радиуса {r:.1}, площадь = {:.2}", r * r * std::f64::consts::PI);
    }
    // Shape::Rectangle и Shape::Triangle — молча пропущены

    // --- кортеж ---
    let point = (0_i32, 5_i32);
    if let (0, y) = point {
        println!("   Точка на оси Y: y={y}");
    }
    let point2 = (3_i32, 5_i32);
    if let (0, y) = point2 {
        println!("   Это не выведется — x != 0");
    } else {
        println!("   Точка ({}, {}) не на оси Y", point2.0, point2.1);
    }

    // --- struct-паттерн ---
    struct Rgb { r: u8, g: u8, b: u8 }
    let color = Rgb { r: 255, g: 30, b: 10 };
    if let Rgb { r: 255, g, b } = color {
        println!("   Насыщенный красный: g={g}, b={b}");
    }

    // ============================================
    // 3. IF LET С ELSE
    // ============================================
    println!("\n3. if let с else — аналог двухветвевого match");

    let maybe: Option<i32> = Some(42);

    // match-версия:
    match maybe {
        Some(n) => println!("   [match]  Есть значение: {n}"),
        None    => println!("   [match]  Пусто"),
    }

    // if let-версия — то же самое, но лаконичнее:
    if let Some(n) = maybe {
        println!("   [if let] Есть значение: {n}");
    } else {
        println!("   [if let] Пусто");
    }

    // else-блок НЕ видит переменные из паттерна (n там недоступен),
    // но видит все переменные из внешней области:
    let label = "результат";
    if let Some(n) = maybe {
        println!("   {label} = {n}");
    } else {
        println!("   {label} отсутствует"); // label доступен, n — нет
    }

    // ============================================
    // 4. ЦЕПОЧКИ ELSE IF LET
    // ============================================
    println!("\n4. Цепочки else if let");

    // Последовательная проверка нескольких паттернов.
    // Полезно когда вариантов 2–3 и возврат значения не нужен.

    #[derive(Debug)]
    enum ServerError {
        Timeout,
        NotFound(String),
        Unauthorized,
        Internal(u32),
    }

    let errors: Vec<ServerError> = vec![
        ServerError::Timeout,
        ServerError::NotFound(String::from("/api/users")),
        ServerError::Unauthorized,
        ServerError::Internal(503),
    ];

    for err in &errors {
        if let ServerError::Timeout = err {
            println!("   Таймаут — повторить запрос");
        } else if let ServerError::NotFound(path) = err {
            println!("   Не найдено: {path}");
        } else if let ServerError::Unauthorized = err {
            println!("   Нет доступа — нужна авторизация");
        } else if let ServerError::Internal(code) = err {
            println!("   Внутренняя ошибка сервера: {code}");
        }
    }

    // Когда match читабельнее: если вариантов 3+ и нужно значение —
    // лучше match. if let хорош когда интересен один-два варианта.
    println!("\n   Для сравнения — тот же код через match (нагляднее при 4+ ветках):");
    for err in &errors {
        let msg = match err {
            ServerError::Timeout          => String::from("Таймаут"),
            ServerError::NotFound(p)      => format!("Не найдено: {p}"),
            ServerError::Unauthorized     => String::from("Нет доступа"),
            ServerError::Internal(c)      => format!("Ошибка сервера: {c}"),
        };
        println!("   → {msg}");
    }

    // ============================================
    // 5. IF LET С ref, @ И ВЛОЖЕННЫМИ ПАТТЕРНАМИ
    // ============================================
    println!("\n5. if let с ref, @ и вложенными паттернами");

    // --- ref: не перемещаем значение из Option ---
    let data: Option<String> = Some(String::from("hello, Rust!"));

    if let Some(ref s) = data {
        // s: &String — data не перемещена
        println!("   ref в if let: длина = {}, заглавные = {}", s.len(), s.to_uppercase());
    }
    println!("   data после if let: {:?}", data); // всё ещё жива

    // --- @ внутри if let: захватить и проверить диапазон ---
    let maybe_score: Option<u32> = Some(87);
    if let Some(score @ 60..=100) = maybe_score {
        println!("   @-binding: зачёт, score={score}");
    }
    if let Some(score @ 0..=59) = maybe_score {
        println!("   Незачёт: {score}");
    } else if let Some(score) = maybe_score {
        println!("   Зачёт (else if): score={score}");
    }

    // --- вложенный паттерн ---
    let nested: Option<Option<i32>> = Some(Some(7));
    if let Some(Some(inner)) = nested {
        println!("   Вложенный Some: inner={inner}");
    }

    // --- enum с кортежем внутри ---
    #[derive(Debug)]
    enum Packet { Data(u8, Vec<u8>), Ack(u32), Err }

    let pkt = Packet::Data(3, vec![0xDE, 0xAD, 0xBE]);
    if let Packet::Data(len, ref bytes) = pkt {
        println!("   Пакет Data: len={len}, bytes={:?}", bytes);
    }

    // ============================================
    // 6. WHILE LET — ЦИКЛ ПОКА ПАТТЕРН СОВПАДАЕТ
    // ============================================
    println!("\n6. while let — цикл до первого несовпадения");

    // Синтаксис: while let ПАТТЕРН = ВЫРАЖЕНИЕ { ... }
    // Продолжает итерации пока паттерн совпадает.
    // При первом несовпадении — выходит из цикла.

    // --- классика: опустошение стека ---
    let mut stack = vec![10, 20, 30, 40, 50];
    print!("   Стек: ");
    while let Some(top) = stack.pop() {
        print!("{top} ");
    }
    println!("(стек опустошён)");

    // --- обратный отсчёт через Option ---
    let mut countdown: Option<u32> = Some(5);
    print!("   Отсчёт: ");
    while let Some(n) = countdown {
        print!("{n}.. ");
        countdown = if n > 1 { Some(n - 1) } else { None };
    }
    println!("Пуск!");

    // --- итерация по срезу с мутабельным состоянием ---
    let tokens = vec![
        Some("let"),
        Some("x"),
        Some("="),
        None, // конец токенов
        Some("unreachable"),
    ];
    let mut iter = tokens.iter();
    print!("   Токены: ");
    while let Some(Some(token)) = iter.next() {
        // Some(None) или None тоже прервут цикл
        print!("[{token}] ");
    }
    println!();

    // Сравнение: while let vs loop + break:
    // while let Some(x) = it.next() { ... }
    // эквивалентно:
    // loop { match it.next() { Some(x) => { ... }, _ => break } }
    // while let — лаконичнее и яснее намерение.

    // ============================================
    // 7. LET...ELSE — РАННИЙ ВЫХОД (RUST 1.65+)
    // ============================================
    println!("\n7. let...else — ранний выход если паттерн не совпал");

    // Синтаксис:
    //   let ПАТТЕРН = ВЫРАЖЕНИЕ else {
    //       // ОБЯЗАТЕЛЬНО расходящийся блок:
    //       // return / break / continue / panic! / todo!
    //   };
    //
    // Ключевое отличие от if let:
    //   if let Some(x) = val { /* x доступен только здесь */ }
    //   let Some(x) = val else { return; };
    //   /* x доступен здесь — в той же области видимости! */

    fn describe_number(maybe: Option<i32>) -> String {
        // Если None — сразу выходим. Дальнейший код плоский.
        let Some(n) = maybe else {
            return String::from("   значение отсутствует");
        };
        // n доступна здесь — не нужна вложенность if let { ... }
        if n > 0 {
            format!("   положительное: {n}")
        } else if n < 0 {
            format!("   отрицательное: {n}")
        } else {
            String::from("   ноль")
        }
    }

    println!("{}", describe_number(Some(42)));
    println!("{}", describe_number(None));
    println!("{}", describe_number(Some(-7)));

    // --- несколько let...else подряд — "flat" стиль без лесенки ---
    fn parse_connection(host: Option<&str>, port: Option<u16>) -> String {
        let Some(h) = host else {
            return String::from("   ошибка: хост не указан");
        };
        let Some(p) = port else {
            return String::from("   ошибка: порт не указан");
        };
        let 1..=65535 = p else {
            return format!("   ошибка: порт {p} вне диапазона");
        };
        format!("   подключение: {h}:{p}")
    }

    println!("{}", parse_connection(Some("localhost"), Some(8080)));
    println!("{}", parse_connection(None,              Some(8080)));
    println!("{}", parse_connection(Some("example.com"), None));
    println!("{}", parse_connection(Some("host"), Some(0)));

    // --- let...else с enum ---
    #[derive(Debug)]
    enum Response { Json(String), Binary(Vec<u8>), Empty }

    fn handle(resp: Response) -> String {
        let Response::Json(body) = resp else {
            return String::from("   не JSON — пропускаем");
        };
        format!("   JSON тело: {body}")
    }

    println!("{}", handle(Response::Json(String::from("{\"ok\":true}"))));
    println!("{}", handle(Response::Binary(vec![0x00, 0xFF])));
    println!("{}", handle(Response::Empty));

    // ============================================
    // 8. СРАВНЕНИЕ: КОГДА ЧТО ВЫБРАТЬ
    // ============================================
    println!("\n8. Когда что выбрать");
    println!("   ┌─────────────────┬────────────────────────────────────────┐");
    println!("   │ Конструкция     │ Когда использовать                     │");
    println!("   ├─────────────────┼────────────────────────────────────────┤");
    println!("   │ match           │ 2+ вариантов, нужна исчерпывающесть    │");
    println!("   │ if let          │ 1 вариант важен, остальные — игнор     │");
    println!("   │ if let + else   │ 2 варианта, значение не возвращается   │");
    println!("   │ else if let     │ 2–3 варианта, без возврата значения    │");
    println!("   │ while let       │ цикл до первого несовпадения паттерна  │");
    println!("   │ let...else      │ ранний выход, переменная нужна после   │");
    println!("   └─────────────────┴────────────────────────────────────────┘");

    // Антипаттерн: if let там где match уместнее
    // ПЛОХО — три else if let подряд утомительны:
    //   if let A = x { } else if let B = x { } else if let C = x { } else if let D = x { }
    // ХОРОШО — match на 4 варианта:
    //   match x { A => {}, B => {}, C => {}, D => {} }

    // ============================================
    // 9. ПРАКТИКА: ОБРАБОТЧИК КОМАНД
    // ============================================
    println!("\n9. Практика: обработчик команд");

    #[derive(Debug)]
    enum Cmd {
        Echo(String),
        Repeat(u32, String),
        Shout(String),
        Quit,
        Unknown(String),
    }

    fn parse_cmd(s: &str) -> Cmd {
        // split_once разбивает строку на две части по первому пробелу:
        // "echo привет мир" → Some(("echo", "привет мир"))
        // "quit"            → None
        match s.split_once(' ') {
            None => match s {
                "quit" => Cmd::Quit,
                other  => Cmd::Unknown(other.to_string()),
            },
            Some(("echo",  msg)) => Cmd::Echo(msg.to_string()),
            Some(("shout", msg)) => Cmd::Shout(msg.to_string()),
            Some(("repeat", rest)) => {
                // rest = "3 ура" → нужно отделить число от сообщения
                let Some((n_str, msg)) = rest.split_once(' ') else {
                    return Cmd::Unknown(s.to_string());
                };
                let Ok(count) = n_str.parse::<u32>() else {
                    return Cmd::Unknown(s.to_string());
                };
                Cmd::Repeat(count, msg.to_string())
            }
            Some(_) => Cmd::Unknown(s.to_string()),
        }
    }

    let commands = vec![
        "echo привет мир",
        "shout тихий голос",
        "repeat 3 ура",
        "repeat abc сломано",
        "quit",
        "взлом системы",
    ];

    let mut running = true;
    let mut cmd_iter = commands.iter();

    while running {
        // while let: берём команды пока они есть
        let Some(raw) = cmd_iter.next() else { break };

        let cmd = parse_cmd(raw);

        // quit — ранний выход через let...else внутри цикла
        if let Cmd::Quit = cmd {
            println!("   [quit] Завершаем работу");
            running = false;
            continue;
        }

        // Остальные команды — через match
        match cmd {
            Cmd::Echo(msg) => {
                println!("   [echo] {msg}");
            }
            Cmd::Shout(msg) => {
                println!("   [shout] {}", msg.to_uppercase());
            }
            Cmd::Repeat(n, msg) => {
                print!("   [repeat x{n}]");
                for _ in 0..n { print!(" {msg}"); }
                println!();
            }
            Cmd::Unknown(raw) => {
                println!("   [?] Неизвестная команда: \"{raw}\"");
            }
            Cmd::Quit => unreachable!(),
        }
    }

    // ============================================
    // ИТОГИ УРОКА
    // ============================================
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                     ИТОГИ УРОКА                             ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  if let PAT = expr {{}}                                     ║");
    println!("║    — выполняется только если паттерн совпал                 ║");
    println!("║    — переменные паттерна живут только внутри блока          ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  while let PAT = expr {{}}                                  ║");
    println!("║    — цикл завершается при первом несовпадении               ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  let PAT = expr else {{ return/break/panic!() }};           ║");
    println!("║    — ранний выход при несовпадении                          ║");
    println!("║    — переменные паттерна доступны ПОСЛЕ блока (!)           ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  Все три поддерживают ref, @-binding, вложенные паттерны    ║");
    println!("║  Если вариантов 3+ и нужно значение — лучше match           ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    println!("\n✅ Урок по if let, while let и let...else завершён!");
}
