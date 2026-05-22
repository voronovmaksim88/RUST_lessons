// ============================================================================
// УРОК: СТРУКТУРЫ (STRUCTS) В RUST
// ============================================================================
// Структуры - это составные типы данных, которые позволяют группировать
// связанные данные под одним именем. Структуры являются основой для
// создания пользовательских типов данных в Rust.
// ============================================================================

fn main() {
    println!("=== УРОК: СТРУКТУРЫ В RUST ===\n");

    // ------------------------------------------------------------------------
    // 1. ПРОСТЕЙШАЯ СТРУКТУРА (Unit-like struct)
    // ------------------------------------------------------------------------
    println!("1. Простейшая структура без полей:");

    struct EmptyStruct;

    let _empty = EmptyStruct;
    println!("   Создана пустая структура EmptyStruct");
    println!("   Используется редко, но может быть полезна для типов-маркеров\n");

    // ------------------------------------------------------------------------
    // 2. СТРУКТУРА С ИМЕНОВАННЫМИ ПОЛЯМИ
    // ------------------------------------------------------------------------
    println!("2. Структура с именованными полями:");

    struct Person {
        name: String,
        age: u8,
        height: f32,
        is_student: bool,
    }

    // Создание экземпляра структуры
    let tom = Person {
        name: String::from("Tom"),
        age: 36,
        height: 1.78,
        is_student: false,
    };

    println!("   Имя: {}", tom.name);
    println!("   Возраст: {}", tom.age);
    println!("   Рост: {}", tom.height);
    println!("   Студент: {}\n", tom.is_student);

    // ------------------------------------------------------------------------
    // 3. ИЗМЕНЯЕМЫЕ СТРУКТУРЫ
    // ------------------------------------------------------------------------
    println!("3. Изменяемые структуры (mut):");

    let mut alice = Person {
        name: String::from("Alice"),
        age: 25,
        height: 1.65,
        is_student: true,
    };

    println!("   До изменения: {} лет, рост {}", alice.age, alice.height);

    alice.age = 26;
    alice.height = 1.66;
    alice.is_student = false;

    println!(
        "   После изменения: {} лет, рост {}",
        alice.age, alice.height
    );
    println!("   Студент: {}\n", alice.is_student);

    // ------------------------------------------------------------------------
    // 4. СОКРАЩЁННАЯ ИНИЦИАЛИЗАЦИЯ (Field Init Shorthand)
    // ------------------------------------------------------------------------
    println!("4. Сокращённая инициализация:");

    let name = String::from("Bob");
    let age = 30;
    let height = 1.80;

    let bob = Person {
        name, // вместо name: name
        age,  // вместо age: age
        height,
        is_student: false,
    };

    println!(
        "   Создан {} с возрастом {} и ростом {}\n",
        bob.name, bob.age, bob.height
    );

    // ------------------------------------------------------------------------
    // 5. ОБНОВЛЕНИЕ СТРУКТУРЫ (Struct Update Syntax)
    // ------------------------------------------------------------------------
    println!("5. Создание структуры на основе другой:");

    let charlie = Person {
        name: String::from("Charlie"),
        age: 28,
        ..bob // берём остальные поля из bob
    };

    println!(
        "   Charlie: возраст {}, рост {} (рост взят из bob)",
        charlie.age, charlie.height
    );
    // Примечание: после этого bob.name и bob.is_student больше недоступны,
    // так как они не реализуют трейт Copy
    println!();

    // ------------------------------------------------------------------------
    // 6. СТРУКТУРЫ-КОРТЕЖИ (Tuple Structs)
    // ------------------------------------------------------------------------
    println!("6. Структуры-кортежи:");

    struct Color(u8, u8, u8); // RGB
    struct Point3D(f64, f64, f64); // x, y, z

    let red = Color(255, 0, 0);
    let point = Point3D(1.5, 2.3, 3.7);

    println!("   Красный цвет: RGB({}, {}, {})", red.0, red.1, red.2);
    println!(
        "   Точка в пространстве: ({}, {}, {})\n",
        point.0, point.1, point.2
    );

    // ------------------------------------------------------------------------
    // 7. ДЕКОМПОЗИЦИЯ СТРУКТУРЫ
    // ------------------------------------------------------------------------
    println!("7. Декомпозиция (деструктуризация) структуры:");

    struct Rectangle {
        width: f64,
        height: f64,
    }

    let rect = Rectangle {
        width: 10.5,
        height: 5.2,
    };

    let Rectangle { width, height } = rect;
    println!("   Ширина: {}, Высота: {}", width, height);

    // Декомпозиция с переименованием
    let Rectangle {
        width: w,
        height: h,
    } = Rectangle {
        width: 20.0,
        height: 15.0,
    };
    println!("   Новый прямоугольник: ширина {}, высота {}", w, h);

    // Частичная декомпозиция
    let rect2 = Rectangle {
        width: 30.0,
        height: 25.0,
    };
    let Rectangle { width, .. } = rect2;
    println!("   Извлекли только ширину: {}\n", width);

    // ------------------------------------------------------------------------
    // 8. СТРУКТУРЫ КАК ПАРАМЕТРЫ ФУНКЦИЙ
    // ------------------------------------------------------------------------
    println!("8. Структуры как параметры функций:");

    struct Book {
        title: String,
        author: String,
        pages: u32,
    }

    // Передача по ссылке (рекомендуется)
    fn print_book(book: &Book) {
        println!(
            "   '{}' by {} ({} pages)",
            book.title, book.author, book.pages
        );
    }

    let my_book = Book {
        title: String::from("Rust Programming"),
        author: String::from("Steve Klabnik"),
        pages: 500,
    };

    print_book(&my_book);
    print_book(&my_book); // Можем использовать повторно
    println!();

    // ------------------------------------------------------------------------
    // 9. ВОЗВРАТ СТРУКТУРЫ ИЗ ФУНКЦИИ
    // ------------------------------------------------------------------------
    println!("9. Возврат структуры из функции:");

    fn create_book(title: String, author: String, pages: u32) -> Book {
        Book {
            title,
            author,
            pages,
        }
    }

    let new_book = create_book(
        String::from("The Rust Book"),
        String::from("Carol Nichols"),
        600,
    );

    print_book(&new_book);
    println!();

    // ------------------------------------------------------------------------
    // 10. ВЛОЖЕННЫЕ СТРУКТУРЫ
    // ------------------------------------------------------------------------
    println!("10. Вложенные структуры:");

    struct Address {
        street: String,
        city: String,
        country: String,
    }

    struct Employee {
        name: String,
        position: String,
        address: Address,
        salary: u32,
    }

    let employee = Employee {
        name: String::from("John Doe"),
        position: String::from("Software Engineer"),
        address: Address {
            street: String::from("123 Main St"),
            city: String::from("New York"),
            country: String::from("USA"),
        },
        salary: 75000,
    };

    println!("   Сотрудник: {}", employee.name);
    println!("   Должность: {}", employee.position);
    println!(
        "   Адрес: {}, {}, {}",
        employee.address.street, employee.address.city, employee.address.country
    );
    println!("   Зарплата: ${}\n", employee.salary);

    // ------------------------------------------------------------------------
    // 11. МЕТОДЫ СТРУКТУР
    // ------------------------------------------------------------------------
    println!("11. Методы структур:");

    struct Circle {
        radius: f64,
    }

    // impl в Rust — это блок реализации: в нём ты описываешь методы для типа (структуры, enum и т.д.).
    impl Circle {
        // Метод (принимает self)
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }

        fn circumference(&self) -> f64 {
            2.0 * std::f64::consts::PI * self.radius
        }

        // Метод, изменяющий self
        fn scale(&mut self, factor: f64) {
            self.radius *= factor;
        }

        // Ассоциированная функция (не принимает self)
        fn new(radius: f64) -> Circle {
            Circle { radius }
        }
    }

    let mut circle = Circle::new(5.0);
    println!("   Круг с радиусом: {}", circle.radius);
    println!("   Площадь: {:.2}", circle.area());
    println!("   Длина окружности: {:.2}", circle.circumference());

    circle.scale(2.0);
    println!(
        "   После масштабирования (x2): радиус = {}, площадь = {:.2}\n",
        circle.radius,
        circle.area()
    );

    // ------------------------------------------------------------------------
    // 12. СТРУКТУРЫ С НЕСКОЛЬКИМИ impl БЛОКАМИ
    // ------------------------------------------------------------------------
    println!("12. Множественные impl блоки:");

    struct Counter {
        count: i32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }

        fn increment(&mut self) {
            self.count += 1;
        }
    }

    impl Counter {
        fn decrement(&mut self) {
            self.count -= 1;
        }

        fn reset(&mut self) {
            self.count = 0;
        }

        fn get(&self) -> i32 {
            self.count
        }
    }

    let mut counter = Counter::new();
    counter.increment();
    counter.increment();
    counter.increment();
    println!("   Счётчик после 3 инкрементов: {}", counter.get());
    counter.decrement();
    println!("   Счётчик после декремента: {}", counter.get());
    counter.reset();
    println!("   Счётчик после сброса: {}\n", counter.get());

    // ------------------------------------------------------------------------
    // 13. СТРУКТУРЫ С GENERIC ТИПАМИ
    // ------------------------------------------------------------------------
    println!("13. Обобщённые (Generic) структуры:");

    struct Pair<T> {
        first: T,
        second: T,
    }

    impl<T> Pair<T> {
        fn new(first: T, second: T) -> Pair<T> {
            Pair { first, second }
        }
    }

    let int_pair = Pair::new(10, 20);
    let float_pair = Pair::new(3.14, 2.71);
    let string_pair = Pair::new(String::from("Hello"), String::from("World"));

    println!("   Пара целых: {} и {}", int_pair.first, int_pair.second);
    println!(
        "   Пара дробных: {} и {}",
        float_pair.first, float_pair.second
    );
    println!(
        "   Пара строк: {} и {}\n",
        string_pair.first, string_pair.second
    );

    // ------------------------------------------------------------------------
    // 14. СТРУКТУРЫ С РАЗНЫМИ ТИПАМИ ПАРАМЕТРОВ
    // ------------------------------------------------------------------------
    println!("14. Структуры с несколькими типами параметров:");

    struct MixedPair<T, U> {
        first: T,
        second: U,
    }

    impl<T, U> MixedPair<T, U> {
        fn new(first: T, second: U) -> MixedPair<T, U> {
            MixedPair { first, second }
        }
    }

    let mixed = MixedPair::new(42, String::from("ответ"));
    println!("   Смешанная пара: {} и {}\n", mixed.first, mixed.second);

    // ------------------------------------------------------------------------
    // 15. АВТОМАТИЧЕСКИЕ ТРЕЙТЫ (Debug, Clone, Copy)
    // ------------------------------------------------------------------------
    println!("15. Автоматические трейты:");

    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 10, y: 20 };
    let p2 = p1; // Copy работает

    println!("   Point 1: {:?}", p1); // Debug работает
    println!("   Point 2: {:?}", p2);
    println!("   Point 1: x = {}, y = {}", p1.x, p1.y);
    println!("   Point 2: x = {}, y = {}", p2.x, p2.y);

    #[derive(Debug, Clone)]
    struct Vehicle {
        brand: String,
        model: String,
        year: u16,
    }

    let car1 = Vehicle {
        brand: String::from("Toyota"),
        model: String::from("Camry"),
        year: 2023,
    };

    let car2 = car1.clone(); // Clone работает
    println!("   Автомобиль 1: {:?}", car1);
    println!("   Автомобиль 2 (клон): {:?}", car2);
    println!(
        "   Автомобиль 1: brand = {}, model = {}, year = {}",
        car1.brand, car1.model, car1.year
    );
    println!(
        "   Автомобиль 2: brand = {}, model = {}, year = {}\n",
        car2.brand, car2.model, car2.year
    );

    // ------------------------------------------------------------------------
    // 16. СТРУКТУРЫ СО ССЫЛКАМИ (требуют lifetime аннотаций)
    // ------------------------------------------------------------------------
    println!("16. Структуры со ссылками:");

    struct TextRef<'a> {
        content: &'a str,
        author: &'a str,
    }

    let content = "Rust - это потрясающий язык!";
    let author = "Программист";

    let text = TextRef { content, author };

    println!("   '{}' - {}\n", text.content, text.author);

    // ------------------------------------------------------------------------
    // 17. ПАТТЕРН БИЛДЕР (Builder Pattern)
    // ------------------------------------------------------------------------
    println!("17. Паттерн Builder:");

    #[derive(Debug)]
    struct Computer {
        cpu: String,
        ram: u32,
        storage: u32,
        gpu: Option<String>,
    }

    struct ComputerBuilder {
        cpu: String,
        ram: u32,
        storage: u32,
        gpu: Option<String>,
    }

    impl ComputerBuilder {
        fn new(cpu: String) -> ComputerBuilder {
            ComputerBuilder {
                cpu,
                ram: 8,
                storage: 256,
                gpu: None,
            }
        }

        fn ram(mut self, ram: u32) -> ComputerBuilder {
            self.ram = ram;
            self
        }

        fn storage(mut self, storage: u32) -> ComputerBuilder {
            self.storage = storage;
            self
        }

        fn gpu(mut self, gpu: String) -> ComputerBuilder {
            self.gpu = Some(gpu);
            self
        }

        fn build(self) -> Computer {
            Computer {
                cpu: self.cpu,
                ram: self.ram,
                storage: self.storage,
                gpu: self.gpu,
            }
        }
    }

    let pc = ComputerBuilder::new(String::from("Intel i7"))
        .ram(16)
        .storage(512)
        .gpu(String::from("NVIDIA RTX 3080"))
        .build();

    println!("   Собран компьютер: {:?}", pc);
    println!(
        "   Параметры: CPU = {}, RAM = {} GB, Storage = {} GB, GPU = {}\n",
        pc.cpu,
        pc.ram,
        pc.storage,
        pc.gpu.as_deref().unwrap_or("None")
    );

    // ------------------------------------------------------------------------
    // 18. NEWTYPE ПАТТЕРН
    // ------------------------------------------------------------------------
    println!("18. Newtype паттерн:");

    struct Kilometers(f64);
    struct Miles(f64);

    impl Kilometers {
        fn to_miles(&self) -> Miles {
            Miles(self.0 * 0.621371)
        }
    }

    impl Miles {
        fn to_kilometers(&self) -> Kilometers {
            Kilometers(self.0 / 0.621371)
        }
    }

    let distance_km = Kilometers(100.0);
    let distance_miles = distance_km.to_miles();
    let distance_km_back = distance_miles.to_kilometers();

    println!("   {} км = {:.2} миль", distance_km.0, distance_miles.0);
    println!(
        "   {:.2} миль = {:.2} км\n",
        distance_miles.0, distance_km_back.0
    );

    // ------------------------------------------------------------------------
    // 19. ПРАКТИЧЕСКИЙ ПРИМЕР: СИСТЕМА УПРАВЛЕНИЯ ЗАДАЧАМИ
    // ------------------------------------------------------------------------
    println!("19. Практический пример - Система задач:");

    #[derive(Debug, Clone)]
    enum Priority {
        Low,
        Medium,
        High,
    }

    #[derive(Debug, Clone)]
    struct Task {
        id: u32,
        title: String,
        description: String,
        completed: bool,
        priority: Priority,
    }

    impl Task {
        fn new(id: u32, title: String, description: String, priority: Priority) -> Task {
            Task {
                id,
                title,
                description,
                completed: false,
                priority,
            }
        }

        fn complete(&mut self) {
            self.completed = true;
        }

        fn display(&self) {
            let status = if self.completed { "✓" } else { "✗" };
            println!(
                "   [{}] Task #{}: {} (Priority: {:?})",
                status, self.id, self.title, self.priority
            );
            println!("       Description: {}", self.description);
        }
    }

    let mut task1 = Task::new(
        1,
        String::from("Изучить структуры"),
        String::from("Освоить все возможности структур в Rust"),
        Priority::High,
    );

    let task2 = Task::new(
        2,
        String::from("Написать проект"),
        String::from("Применить знания на практике"),
        Priority::Medium,
    );

    let task3 = Task::new(
        3,
        String::from("Сделать обзор"),
        String::from("Кратко описать ключевые идеи"),
        Priority::Low,
    );

    task1.display();
    task2.display();
    task3.display();

    task1.complete();
    println!("\n   После выполнения первой задачи:");
    task1.display();
    task2.display();
    println!();

    // ------------------------------------------------------------------------
    // 20. СРАВНЕНИЕ СТРУКТУР
    // ------------------------------------------------------------------------
    println!("20. Сравнение структур:");

    #[derive(Debug, PartialEq)]
    struct Coordinate {
        x: i32,
        y: i32,
    }

    let coord1 = Coordinate { x: 5, y: 10 };
    let coord2 = Coordinate { x: 5, y: 10 };
    let coord3 = Coordinate { x: 3, y: 7 };

    println!("   coord1 == coord2: {}", coord1 == coord2);
    println!("   coord1 == coord3: {}", coord1 == coord3);
    println!();

    // ------------------------------------------------------------------------
    // ЗАКЛЮЧЕНИЕ
    // ------------------------------------------------------------------------
    println!("=== ИТОГИ УРОКА ===");
    println!("✓ Простые структуры и структуры с полями");
    println!("✓ Изменяемость и сокращённая инициализация");
    println!("✓ Структуры-кортежи");
    println!("✓ Методы и ассоциированные функции");
    println!("✓ Обобщённые структуры (Generics)");
    println!("✓ Автоматические трейты (Debug, Clone, Copy, PartialEq)");
    println!("✓ Паттерны проектирования (Builder, Newtype)");
    println!("✓ Практические примеры использования");
    println!("\nСтруктуры - это мощный инструмент для создания");
    println!("пользовательских типов данных в Rust!");
}

// ============================================================================
// ДОПОЛНИТЕЛЬНЫЕ ПРИМЕЧАНИЯ:
// ============================================================================
//
// 1. NAMING CONVENTIONS:
//    - Имена структур: PascalCase (Person, Employee, BookStore)
//    - Имена полей: snake_case (first_name, age, is_active)
//
// 2. OWNERSHIP И BORROWING:
//    - При передаче структуры в функцию передаётся владение
//    - Используйте ссылки (&) для заимствования без передачи владения
//    - Используйте &mut для изменяемого заимствования
//
// 3. COPY vs CLONE:
//    - Copy: автоматическое копирование для простых типов
//    - Clone: явное копирование через .clone()
//    - Структуры со String не могут быть Copy
//
// 4. ПРОИЗВОДНЫЕ ТРЕЙТЫ (Derive):
//    - Debug: форматирование с {:?}
//    - Clone: метод .clone()
//    - Copy: автоматическое копирование
//    - PartialEq: операторы == и !=
//    - Eq: полное равенство
//    - PartialOrd, Ord: операторы сравнения
//    - Hash: хеширование для HashMap/HashSet
//
// 5. BEST PRACTICES:
//    - Делайте поля приватными, предоставляйте публичные методы
//    - Используйте ассоциированные функции для конструкторов
//    - Предпочитайте маленькие, сфокусированные структуры
//    - Используйте паттерн Builder для структур с множеством полей
//    - Документируйте публичные структуры и их поля
//
// ============================================================================
