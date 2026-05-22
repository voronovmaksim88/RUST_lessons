# RUST_lessons

Учебный курс по языку Rust, выстроенный по принципу **«от простого к сложному, от часто используемого к редкому»**. Каждый урок — это самостоятельный Cargo-проект, который можно запустить командой `cargo run`.
Текущая структура: **16 блоков (включая 2.5 и 7.5) и 130 уроков**.

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
| `016_raw_and_byte_strings` | Строковые литералы: escape-последовательности, raw strings `r"..."`, байтовые строки `b"..."` |
| `017_labeled_loops_and_break_values` | Метки циклов `'outer`, `break value`, возврат значения из `loop` |
| `018_string_basics` | Базовая работа со строками: `String`, `&str`, конкатенация, UTF-8 и индексация по байтам |
| `019_io_basics` | Базовый ввод/вывод: `std::io`, `stdin().read_line()`, обработка ошибок |

### 2. `2_composite_data_types/` — Составные типы данных

| Урок | Тема |
|---|---|
| `001_array` | Массивы фиксированной длины `[T; N]` |
| `002_tuple` | Кортежи и их деструктуризация |
| `003_tuple_struct` | Структуры-кортежи (tuple structs) |
| `004_structs` | Структуры: поля, методы, `impl`, обобщения, паттерны Builder и Newtype |
| `005_enum_type` | Перечисления `enum` |
| `006_range` | Диапазоны `..` и `..=` |
| `007_vec` | Динамический массив `Vec<T>` |

### 2.5 `2_5_patterns/` — Паттерны и деструктуризация

| Урок | Тема |
|---|---|
| `001_pattern_syntax` | Основы паттернов: wildcard, `_`, `..`, `|`, range-паттерны |
| `002_destructuring_in_let_and_params` | Деструктуризация в `let` и в параметрах функций |
| `003_match_guards_or_at_binding` | Match guards (`if` в ветках) и `@`-binding |
| `004_ref_refmut_ranges` | `ref` / `ref mut`, диапазоны в паттернах, управление заимствованием |
| `005_if_let_while_let_let_else` | Связка `if let`, `while let`, `let ... else` на практике |

### 3. `3_ownership_and_borrowing/` — Владение и заимствование

| Урок | Тема |
|---|---|
| `001_ownership` | Владение, `move`, `drop` — фундамент модели памяти Rust |
| `002_references_and_borrowing` | Ссылки `&T`, `&mut T`, правила заимствования |
| `003_slices` | Срезы `&str` и `&[T]` |
| `004_lifetimes_basics` | Базовые аннотации времён жизни `'a` |
| `005_string_vs_str` | Различия между `String` и `&str` |
| `006_copy_vs_clone` | Разница `Copy` и `Clone`, влияние на `move` и владение |

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
| `007_iter_into_iter_iter_mut` | Разница `iter()`, `iter_mut()`, `into_iter()` |
| `008_iterator_implementation` | Реализация собственного итератора (`impl Iterator`) |
| `009_iterator_lazy_and_adapters` | Ленивость итераторов, адаптеры и композиция цепочек |

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
| `009_impl_trait` | `impl Trait` в аргументах и возвращаемых значениях |
| `010_default_methods_and_supertraits` | Методы по умолчанию в трейтах и supertraits (`trait B: A`) |
| `011_blanket_impls_and_coherence` | Blanket impls, orphan rule и coherence |
| `012_conversion_traits` | `From`/`Into`/`TryFrom`/`TryInto` и conversion-идиомы |
| `013_asref_asmut_borrow_toowned` | `AsRef`, `AsMut`, `Borrow`, `ToOwned` для эргономичных API |
| `014_eq_ord_hash_semantics` | Семантика `PartialEq`/`Eq`/`PartialOrd`/`Ord`/`Hash`, включая edge-cases |

### 7. `7_modules_and_packages/` — Модули и пакеты

| Урок | Тема |
|---|---|
| `001_modules` | `mod`, `pub`, `use`, `super`, `crate` |
| `002_files_and_folders` | Разделение проекта на файлы и подпапки |
| `003_cargo_basics` | `Cargo.toml`, зависимости, features |
| `004_workspaces` | Многокрейтовые workspaces |
| `005_tests_and_docs` | `#[test]`, `#[cfg(test)]`, doc-tests |
| `006_visibility_levels` | Уровни видимости: `pub`, `pub(crate)`, `pub(super)`, `pub(in path)` |
| `007_attributes` | Системные атрибуты: `#[derive]`, `#[cfg]`, `#[allow]`, `#[must_use]`, `#[non_exhaustive]` |
| `008_integration_tests` | Integration tests (`tests/`), организация тестовых сценариев |

### 7.5 `7_5_std_library/` — Практика стандартной библиотеки

| Урок | Тема |
|---|---|
| `001_path_and_pathbuf` | Пути и файловые пути: `Path`, `PathBuf` |
| `002_fs_basics` | Работа с файловой системой: `std::fs` |
| `003_env_and_process` | Переменные окружения и процессы: `std::env`, `std::process` |
| `004_time_instant_duration` | Время и измерения: `Instant`, `Duration` |
| `005_buffered_io` | Буферизованный ввод/вывод: `BufReader`, `BufWriter` |

### 8. `8_smart_pointers/` — Умные указатели

| Урок | Тема |
|---|---|
| `001_box` | `Box<T>` — размещение значения в куче |
| `002_rc` | `Rc<T>` — разделяемое владение (однопоточное) |
| `003_refcell` | `RefCell<T>` — внутренняя изменяемость |
| `004_arc_mutex` | `Arc<T>` и `Mutex<T>` — основа многопоточного владения |
| `005_cow` | `Cow<'a, T>` — clone-on-write |
| `006_drop_trait` | Трейт `Drop`, идиома RAII |
| `007_deref_and_deref_coercion` | `Deref`, `DerefMut` и deref coercion (`&String` -> `&str`) |
| `008_cell_oncecell_lazylock` | `Cell`, `RefCell`, `OnceCell`, `LazyLock` и interior mutability |
| `009_drop_order_and_guarantees` | Порядок вызова `Drop`, гарантии деструкторов и edge cases |

### 9. `9_concurrency/` — Многопоточность

| Урок | Тема |
|---|---|
| `001_threads` | `std::thread`, `spawn`, `join` |
| `002_channels` | Каналы `mpsc` — передача сообщений |
| `003_mutex_arc` | Разделяемое состояние через `Arc<Mutex<T>>` |
| `004_send_sync` | Маркер-трейты `Send` и `Sync` |
| `005_rayon` | Параллельные итераторы через крейт `rayon` |
| `006_atomics_and_ordering` | Атомики (`AtomicUsize`) и модели порядка памяти (`Ordering`) |
| `007_condvar_barrier_thread_local` | `Condvar`, `Barrier`, `thread_local!` |

### 10. `10_async/` — Асинхронное программирование

| Урок | Тема |
|---|---|
| `001_async_await` | `async fn`, `.await`, `Future` |
| `002_tokio_basics` | Runtime `tokio`, `tokio::spawn` |
| `003_async_io` | Асинхронные файлы и сеть |
| `004_select_join` | `tokio::select!`, `join!` |
| `005_streams` | `Stream` и `tokio-stream` |
| `006_pin_unpin_and_async_traits` | `Pin` / `Unpin`, async в трейтах (native и `async-trait`) |
| `007_spawn_send_static_bounds` | Почему `tokio::spawn` требует `Send + 'static` |
| `008_spawn_blocking_and_block_in_place` | Мост sync/async: `spawn_blocking`, `block_in_place` |
| `009_cancellation_and_future_drop` | Отмена задач, `JoinHandle::abort`, drop у `Future` |

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
| `004_unsafecell_union_repr` | `UnsafeCell`, `union`, `#[repr(C)]`, `#[repr(transparent)]`, обзор `#![no_std]` |

### 13. `13_advanced_patterns/` — Продвинутые паттерны

| Урок | Тема |
|---|---|
| `001_lifetimes_advanced` | `'static`, HRTB, вариативность |
| `002_phantomdata` | `PhantomData<T>` — типобезопасные API |
| `003_state_machines` | Машины состояний через систему типов (typestate) |
| `004_builder_pattern` | Типобезопасный паттерн Builder |
| `005_newtype_pattern` | Паттерн Newtype (углублённо) |
| `006_type_aliases_and_never_type` | Type aliases (`type Foo = ...`) и never type `!` |

### 14. `14_ecosystem/` — Популярные крейты экосистемы

| Урок | Тема |
|---|---|
| `001_serde` | Сериализация и десериализация |
| `002_clap` | Парсинг аргументов командной строки |
| `003_reqwest` | HTTP-клиент |
| `004_axum` | Веб-сервер на `axum` |
| `005_sqlx` | Работа с БД через `sqlx` |
| `006_logging_tracing` | Логирование: `log`, `env_logger`, `tracing` |
| `007_benchmarks_criterion` | Бенчмарки: `cargo bench`, `criterion` |
| `008_property_testing` | Property-based testing: `proptest` / `quickcheck` |
| `009_cargo_tooling` | `cargo check`, `fmt`, `clippy`, `doc`, profiles, `build.rs` |
| `010_tracing_subscriber_spans` | Практика `tracing-subscriber`, spans и контекстная диагностика |

## Дополнительные папки

| Папка | Описание |
|---|---|
| `book/` | Учебные проекты из «The Rust Programming Language» (например, `guess_the_number`) |
| `leetcode/` | Решения задач с LeetCode на Rust |

## Обязательные темы сейчас

- Блоки **1 -> 2 -> 2.5 -> 3 -> 4 -> 5 -> 6 -> 7 -> 7.5** как основа, без них дальше сложно.
- Темы-критичные для повседневного Rust: `Copy/Clone`, итераторы (`iter/iter_mut/into_iter`), conversion traits, `AsRef/Borrow/ToOwned`, `Deref`/coercion, расширенная видимость `pub(...)`.
- Новые уроки в этих блоках пока каркасные: структура и точки входа готовы, детальный материал будет добавляться поэтапно.

## Расширение после базы

- Углубления по параллелизму: `9_concurrency/006-007`.
- Углубления по async: `10_async/006-009` (Pin/Unpin, spawn limits, cancellation).
- Углубления по low-level: `12_unsafe_and_ffi/004`.
- Расширение экосистемы: `14_ecosystem/007-010` (benchmarks, property testing, cargo tooling, tracing spans).

## Покрытие ключевых тем языка

- [x] Базовый синтаксис, управление потоком, функции.
- [x] Составные типы и коллекции.
- [x] Владение и заимствование, включая `Copy/Clone`.
- [x] Pattern matching как отдельный этап (`2_5_patterns`).
- [x] Трейты и обобщения, включая conversion traits, `AsRef/Borrow/ToOwned`, `Eq/Ord/Hash`.
- [x] Модули, видимость и системные атрибуты.
- [x] Практика стандартной библиотеки (`std::fs`, `std::path`, `std::env`, `std::process`, `std::time`, buffered I/O).
- [x] Расширения async/concurrency/unsafe добавлены в структуру курса.
- [x] Расширены темы тестирования и инструментов `cargo`.

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

Рекомендуемая последовательность старта: **1 -> 2 -> 2.5 -> 3 -> 4 -> 5 -> 6 -> 7 -> 7.5**, затем переход к блокам 8-14.
