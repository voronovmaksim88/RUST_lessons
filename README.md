# RUST_lessons

Учебный курс по языку Rust, выстроенный по принципу **«от простого к сложному, от часто используемого к редкому»**. Каждый урок — это самостоятельный Cargo-проект, который можно запустить командой `cargo run`.

## Структура курса

### 1. `1_basics/` — Базовый синтаксис

| Урок | Тема |
|---|---|
| `001_hello_world` | Первая программа, компиляция через `rustc`, запуск `.exe` |
| `002_var_type` | Переменные, типы данных, `let` и `mut` |
| `003_print_var` | Вывод переменных через `println!`, плейсхолдеры `{}` и `{:?}` |
| `004_shadowing` | Затенение переменных (shadowing) |
| `005_format_number` | Форматирование чисел: ширина, точность, основание системы счисления |
| `006_convert_data_type` | Преобразование типов: `as`, `.parse()`, `.to_string()` |
| `007_arithmetic_operations` | Арифметические операции и их особенности |
| `008_bitwise_operations` | Побитовые операции: `&`, `\|`, `^`, `<<`, `>>` |
| `009_conditional_expressions` | Условные выражения и булева логика |
| `010_if_else_construction` | Конструкция `if`/`else`, `if` как выражение |
| `011_match` | Сопоставление с образцом — `match` |
| `012_cycle` | Циклы `loop`, `while`, `for` |
| `013_function` | Функции, параметры, возврат значений |
| `014_const` | Константы `const` и статические переменные `static` |
| `015_anonymous_function` | Замыкания (closures), захват переменных |

### 2. `2_composite_data_types/` — Составные типы данных

| Урок | Тема |
|---|---|
| `array` | Массивы фиксированной длины `[T; N]` |
| `tuple` | Кортежи и их деструктуризация |
| `tuple_struct` | Структуры-кортежи (tuple structs) |
| `structs` | Структуры: поля, методы, `impl`, обобщения, паттерны Builder и Newtype |
| `enum_type` | Перечисления `enum` |
| `range` | Диапазоны `..` и `..=` |
| `vec` | Динамический массив `Vec<T>` |

### 3. `3_ownership_and_borrowing/` — Владение и заимствование

| Урок | Тема |
|---|---|
| `001_ownership` | Владение, `move`, `drop` — фундамент модели памяти Rust |
| `002_references_and_borrowing` | Ссылки `&T`, `&mut T`, правила заимствования |
| `003_slices` | Срезы `&str` и `&[T]` |
| `004_lifetimes_basics` | Базовые аннотации времён жизни `'a` |
| `005_string_vs_str` | Различия между `String` и `&str` |

### 4. `4_error_handling/` — Обработка ошибок

| Урок | Тема |
|---|---|
| `001_panic` | `panic!`, `unwrap`, `expect` — когда программа должна упасть |
| `002_option` | `Option<T>`: `Some` / `None` и методы работы с ним |
| `003_if_let_and_let_else` | `if let`, `while let`, `let ... else` |
| `004_result` | `Result<T, E>`: `Ok` / `Err` |
| `005_question_mark_operator` | Оператор `?` для проброса ошибок |
| `006_custom_errors` | Свои типы ошибок, `From`/`Into` |
| `007_anyhow_thiserror` | Крейты `anyhow` и `thiserror` — реальная работа с ошибками |

### 5. `5_collections/` — Стандартные коллекции

| Урок | Тема |
|---|---|
| `001_string_methods` | Методы `String`, работа с UTF-8 |
| `002_hashmap` | `HashMap<K, V>` — хеш-таблица |
| `003_hashset` | `HashSet<T>` — множество |
| `004_btreemap_btreeset` | Упорядоченные `BTreeMap` и `BTreeSet` |
| `005_vecdeque` | `VecDeque<T>` — двусторонняя очередь |
| `006_iterators` | Итераторы: `map`, `filter`, `collect`, `fold` и др. |

### 6. `6_traits_and_generics/` — Трейты и обобщения

| Урок | Тема |
|---|---|
| `001_traits_basics` | Определение и реализация собственных трейтов |
| `002_derive_macros` | `#[derive(Debug, Clone, Copy, PartialEq, Default)]` |
| `003_generics` | Обобщённые функции и типы |
| `004_trait_bounds` | Ограничения трейтов `T: Trait`, блок `where` |
| `005_trait_objects` | Трейт-объекты `dyn Trait` — динамическая диспетчеризация |
| `006_associated_types` | Ассоциированные типы (`type Item;` в трейтах) |
| `007_operator_overloading` | Перегрузка операторов через `std::ops` (`Add`, `Sub`, `Index`) |
| `008_common_traits` | Часто используемые трейты: `Display`, `From`/`Into`, `Iterator`, `IntoIterator` |

### 7. `7_modules_and_packages/` — Модули и пакеты

| Урок | Тема |
|---|---|
| `001_modules` | `mod`, `pub`, `use`, `super`, `crate` |
| `002_files_and_folders` | Разделение проекта на файлы и подпапки |
| `003_cargo_basics` | `Cargo.toml`, зависимости, features |
| `004_workspaces` | Многокрейтовые workspaces |
| `005_tests_and_docs` | `#[test]`, `#[cfg(test)]`, doc-tests |

### 8. `8_smart_pointers/` — Умные указатели

| Урок | Тема |
|---|---|
| `001_box` | `Box<T>` — размещение значения в куче |
| `002_rc` | `Rc<T>` — разделяемое владение (однопоточное) |
| `003_refcell` | `RefCell<T>` — внутренняя изменяемость |
| `004_arc_mutex` | `Arc<T>` и `Mutex<T>` — основа многопоточного владения |
| `005_cow` | `Cow<'a, T>` — clone-on-write |
| `006_drop_trait` | Трейт `Drop`, идиома RAII |

### 9. `9_concurrency/` — Многопоточность

| Урок | Тема |
|---|---|
| `001_threads` | `std::thread`, `spawn`, `join` |
| `002_channels` | Каналы `mpsc` — передача сообщений |
| `003_mutex_arc` | Разделяемое состояние через `Arc<Mutex<T>>` |
| `004_send_sync` | Маркер-трейты `Send` и `Sync` |
| `005_rayon` | Параллельные итераторы через крейт `rayon` |

### 10. `10_async/` — Асинхронное программирование

| Урок | Тема |
|---|---|
| `001_async_await` | `async fn`, `.await`, `Future` |
| `002_tokio_basics` | Runtime `tokio`, `tokio::spawn` |
| `003_async_io` | Асинхронные файлы и сеть |
| `004_select_join` | `tokio::select!`, `join!` |
| `005_streams` | `Stream` и `tokio-stream` |

### 11. `11_macros/` — Макросы

| Урок | Тема |
|---|---|
| `001_declarative_macros` | Декларативные макросы `macro_rules!` |
| `002_common_macros` | Как устроены `println!`, `vec!`, `format!`, `dbg!` |
| `003_procedural_macros` | Процедурные и derive-макросы (обзорно) |

### 12. `12_unsafe_and_ffi/` — Низкий уровень

| Урок | Тема |
|---|---|
| `001_unsafe_basics` | Блок `unsafe { }`, сырые указатели |
| `002_ffi_c` | FFI: `extern "C"`, вызов C-библиотек |
| `003_inline_assembly` | Inline assembly через `asm!` (обзорно) |

### 13. `13_advanced_patterns/` — Продвинутые паттерны

| Урок | Тема |
|---|---|
| `001_lifetimes_advanced` | `'static`, HRTB, вариативность |
| `002_phantomdata` | `PhantomData<T>` — типобезопасные API |
| `003_state_machines` | Машины состояний через систему типов (typestate) |
| `004_builder_pattern` | Типобезопасный паттерн Builder |
| `005_newtype_pattern` | Паттерн Newtype (углублённо) |

### 14. `14_ecosystem/` — Популярные крейты экосистемы

| Урок | Тема |
|---|---|
| `001_serde` | Сериализация и десериализация |
| `002_clap` | Парсинг аргументов командной строки |
| `003_reqwest` | HTTP-клиент |
| `004_axum` | Веб-сервер на `axum` |
| `005_sqlx` | Работа с БД через `sqlx` |
| `006_logging_tracing` | Логирование: `log`, `env_logger`, `tracing` |

## Дополнительные папки

| Папка | Описание |
|---|---|
| `book/` | Учебные проекты из «The Rust Programming Language» (например, `guess_the_number`) |
| `leetcode/` | Решения задач с LeetCode на Rust |

## Как запустить любой урок

Каждый урок — самостоятельный Cargo-проект:

```bash
cd 4_error_handling/002_option
cargo run
```

## Порядок изучения

Блоки рекомендуется проходить по порядку — следующий опирается на предыдущий. Внутри блока уроки тоже идут по нарастанию сложности (номера `001`, `002`, …).

Особо важные блоки, без которых дальше делать нечего:
- **3** (владение и заимствование) — сердце Rust;
- **4** (`Option`, `Result`, `?`) — встречаются в каждой функции;
- **6** (трейты и обобщения) — без них вся стандартная библиотека выглядит магией.
