# RUST_lessons

Учебный курс по языку Rust, выстроенный по принципу **«от простого к сложному, от часто используемого к редкому»**. Каждый урок — это самостоятельный Cargo-проект, который можно запустить командой `cargo run`.
Текущая структура: **16 блоков и 128 уроков** со сквозной нумерацией (`001` … `128`).

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
| `020_array` | Массивы фиксированной длины `[T; N]` |
| `021_tuple` | Кортежи и их деструктуризация |
| `022_tuple_struct` | Структуры-кортежи (tuple structs) |
| `023_structs` | Структуры: поля, методы, `impl`, обобщения, паттерны Builder и Newtype |
| `024_enum_type` | Перечисления `enum` |
| `025_range` | Диапазоны `..` и `..=` |
| `026_vec` | Динамический массив `Vec<T>` |

### 3. `3_patterns/` — Паттерны и деструктуризация

| Урок | Тема |
|---|---|
| `027_pattern_syntax` | Основы паттернов: wildcard, `_`, `..`, `|`, range-паттерны |
| `028_destructuring_in_let_and_params` | Деструктуризация в `let` и в параметрах функций |
| `029_match_guards_or_at_binding` | Match guards (`if` в ветках) и `@`-binding |
| `030_ref_refmut_ranges` | `ref` / `ref mut`, диапазоны в паттернах, управление заимствованием |
| `031_if_let_while_let_let_else` | Связка `if let`, `while let`, `let ... else` на практике |

### 4. `4_ownership_and_borrowing/` — Владение и заимствование

| Урок | Тема |
|---|---|
| `032_ownership` | Владение, `move`, `drop` — фундамент модели памяти Rust |
| `033_references_and_borrowing` | Ссылки `&T`, `&mut T`, правила заимствования |
| `034_slices` | Срезы `&str` и `&[T]` |
| `035_lifetimes_basics` | Базовые аннотации времён жизни `'a` |
| `036_string_vs_str` | Различия между `String` и `&str` |
| `037_copy_vs_clone` | Разница `Copy` и `Clone`, влияние на `move` и владение |

### 5. `5_error_handling/` — Обработка ошибок

| Урок | Тема |
|---|---|
| `038_panic` | `panic!`, `unwrap`, `expect` — когда программа должна упасть |
| `039_option` | `Option<T>`: `Some` / `None` и методы работы с ним |
| `040_if_let_and_let_else` | `if let`, `while let`, `let ... else` |
| `041_result` | `Result<T, E>`: `Ok` / `Err` |
| `042_question_mark_operator` | Оператор `?` для проброса ошибок |
| `043_custom_errors` | Свои типы ошибок, `From`/`Into` |
| `044_anyhow_thiserror` | Крейты `anyhow` и `thiserror` — реальная работа с ошибками |

### 6. `6_collections/` — Стандартные коллекции

| Урок | Тема |
|---|---|
| `045_string_methods` | Методы `String`, работа с UTF-8 |
| `046_hashmap` | `HashMap<K, V>` — хеш-таблица |
| `047_hashset` | `HashSet<T>` — множество |
| `048_btreemap_btreeset` | Упорядоченные `BTreeMap` и `BTreeSet` |
| `049_vecdeque` | `VecDeque<T>` — двусторонняя очередь |
| `050_iterators` | Итераторы: `map`, `filter`, `collect`, `fold` и др. |
| `051_iter_into_iter_iter_mut` | Разница `iter()`, `iter_mut()`, `into_iter()` |
| `052_iterator_implementation` | Реализация собственного итератора (`impl Iterator`) |
| `053_iterator_lazy_and_adapters` | Ленивость итераторов, адаптеры и композиция цепочек |

### 7. `7_traits_and_generics/` — Трейты и обобщения

| Урок | Тема |
|---|---|
| `054_traits_basics` | Определение и реализация собственных трейтов |
| `055_derive_macros` | `#[derive(Debug, Clone, Copy, PartialEq, Default)]` |
| `056_generics` | Обобщённые функции и типы |
| `057_trait_bounds` | Ограничения трейтов `T: Trait`, блок `where` |
| `058_trait_objects` | Трейт-объекты `dyn Trait` — динамическая диспетчеризация |
| `059_associated_types` | Ассоциированные типы (`type Item;` в трейтах) |
| `060_operator_overloading` | Перегрузка операторов через `std::ops` (`Add`, `Sub`, `Index`) |
| `061_common_traits` | Часто используемые трейты: `Display`, `From`/`Into`, `Iterator`, `IntoIterator` |
| `062_impl_trait` | `impl Trait` в аргументах и возвращаемых значениях |
| `063_default_methods_and_supertraits` | Методы по умолчанию в трейтах и supertraits (`trait B: A`) |
| `064_blanket_impls_and_coherence` | Blanket impls, orphan rule и coherence |
| `065_conversion_traits` | `From`/`Into`/`TryFrom`/`TryInto` и conversion-идиомы |
| `066_asref_asmut_borrow_toowned` | `AsRef`, `AsMut`, `Borrow`, `ToOwned` для эргономичных API |
| `067_eq_ord_hash_semantics` | Семантика `PartialEq`/`Eq`/`PartialOrd`/`Ord`/`Hash`, включая edge-cases |

### 8. `8_modules_and_packages/` — Модули и пакеты

| Урок | Тема |
|---|---|
| `068_modules` | `mod`, `pub`, `use`, `super`, `crate` |
| `069_files_and_folders` | Разделение проекта на файлы и подпапки |
| `070_cargo_basics` | `Cargo.toml`, зависимости, features |
| `071_workspaces` | Многокрейтовые workspaces |
| `072_tests_and_docs` | `#[test]`, `#[cfg(test)]`, doc-tests |
| `073_visibility_levels` | Уровни видимости: `pub`, `pub(crate)`, `pub(super)`, `pub(in path)` |
| `074_attributes` | Системные атрибуты: `#[derive]`, `#[cfg]`, `#[allow]`, `#[must_use]`, `#[non_exhaustive]` |
| `075_integration_tests` | Integration tests (`tests/`), организация тестовых сценариев |

### 9. `9_std_library/` — Практика стандартной библиотеки

| Урок | Тема |
|---|---|
| `076_path_and_pathbuf` | Пути и файловые пути: `Path`, `PathBuf` |
| `077_fs_basics` | Работа с файловой системой: `std::fs` |
| `078_env_and_process` | Переменные окружения и процессы: `std::env`, `std::process` |
| `079_time_instant_duration` | Время и измерения: `Instant`, `Duration` |
| `080_buffered_io` | Буферизованный ввод/вывод: `BufReader`, `BufWriter` |

### 10. `10_smart_pointers/` — Умные указатели

| Урок | Тема |
|---|---|
| `081_box` | `Box<T>` — размещение значения в куче |
| `082_rc` | `Rc<T>` — разделяемое владение (однопоточное) |
| `083_refcell` | `RefCell<T>` — внутренняя изменяемость |
| `084_arc_mutex` | `Arc<T>` и `Mutex<T>` — основа многопоточного владения |
| `085_cow` | `Cow<'a, T>` — clone-on-write |
| `086_drop_trait` | Трейт `Drop`, идиома RAII |
| `087_deref_and_deref_coercion` | `Deref`, `DerefMut` и deref coercion (`&String` -> `&str`) |
| `088_cell_oncecell_lazylock` | `Cell`, `RefCell`, `OnceCell`, `LazyLock` и interior mutability |
| `089_drop_order_and_guarantees` | Порядок вызова `Drop`, гарантии деструкторов и edge cases |

### 11. `11_concurrency/` — Многопоточность

| Урок | Тема |
|---|---|
| `090_threads` | `std::thread`, `spawn`, `join` |
| `091_channels` | Каналы `mpsc` — передача сообщений |
| `092_mutex_arc` | Разделяемое состояние через `Arc<Mutex<T>>` |
| `093_send_sync` | Маркер-трейты `Send` и `Sync` |
| `094_rayon` | Параллельные итераторы через крейт `rayon` |
| `095_atomics_and_ordering` | Атомики (`AtomicUsize`) и модели порядка памяти (`Ordering`) |
| `096_condvar_barrier_thread_local` | `Condvar`, `Barrier`, `thread_local!` |

### 12. `12_async/` — Асинхронное программирование

| Урок | Тема |
|---|---|
| `097_async_await` | `async fn`, `.await`, `Future` |
| `098_tokio_basics` | Runtime `tokio`, `tokio::spawn` |
| `099_async_io` | Асинхронные файлы и сеть |
| `100_select_join` | `tokio::select!`, `join!` |
| `101_streams` | `Stream` и `tokio-stream` |
| `102_pin_unpin_and_async_traits` | `Pin` / `Unpin`, async в трейтах (native и `async-trait`) |
| `103_spawn_send_static_bounds` | Почему `tokio::spawn` требует `Send + 'static` |
| `104_spawn_blocking_and_block_in_place` | Мост sync/async: `spawn_blocking`, `block_in_place` |
| `105_cancellation_and_future_drop` | Отмена задач, `JoinHandle::abort`, drop у `Future` |

### 13. `13_macros/` — Макросы

| Урок | Тема |
|---|---|
| `106_declarative_macros` | Декларативные макросы `macro_rules!` |
| `107_common_macros` | Как устроены `println!`, `vec!`, `format!`, `dbg!` |
| `108_procedural_macros` | Процедурные и derive-макросы (обзорно) |

### 14. `14_unsafe_and_ffi/` — Низкий уровень

| Урок | Тема |
|---|---|
| `109_unsafe_basics` | Блок `unsafe { }`, сырые указатели |
| `110_ffi_c` | FFI: `extern "C"`, вызов C-библиотек |
| `111_inline_assembly` | Inline assembly через `asm!` (обзорно) |
| `112_unsafecell_union_repr` | `UnsafeCell`, `union`, `#[repr(C)]`, `#[repr(transparent)]`, обзор `#![no_std]` |

### 15. `15_advanced_patterns/` — Продвинутые паттерны

| Урок | Тема |
|---|---|
| `113_lifetimes_advanced` | `'static`, HRTB, вариативность |
| `114_phantomdata` | `PhantomData<T>` — типобезопасные API |
| `115_state_machines` | Машины состояний через систему типов (typestate) |
| `116_builder_pattern` | Типобезопасный паттерн Builder |
| `117_newtype_pattern` | Паттерн Newtype (углублённо) |
| `118_type_aliases_and_never_type` | Type aliases (`type Foo = ...`) и never type `!` |

### 16. `16_ecosystem/` — Популярные крейты экосистемы

| Урок | Тема |
|---|---|
| `119_serde` | Сериализация и десериализация |
| `120_clap` | Парсинг аргументов командной строки |
| `121_reqwest` | HTTP-клиент |
| `122_axum` | Веб-сервер на `axum` |
| `123_sqlx` | Работа с БД через `sqlx` |
| `124_logging_tracing` | Логирование: `log`, `env_logger`, `tracing` |
| `125_benchmarks_criterion` | Бенчмарки: `cargo bench`, `criterion` |
| `126_property_testing` | Property-based testing: `proptest` / `quickcheck` |
| `127_cargo_tooling` | `cargo check`, `fmt`, `clippy`, `doc`, profiles, `build.rs` |
| `128_tracing_subscriber_spans` | Практика `tracing-subscriber`, spans и контекстная диагностика |

## Дополнительные папки

| Папка | Описание |
|---|---|
| `book/` | Учебные проекты из «The Rust Programming Language» (например, `guess_the_number`) |
| `leetcode/` | Решения задач с LeetCode на Rust |

## Обязательные темы сейчас

- Блоки **1 → 2 → 3 → 4 → 5 → 6 → 7 → 8 → 9** как основа, без них дальше сложно.
- Темы-критичные для повседневного Rust: `Copy/Clone`, итераторы (`iter/iter_mut/into_iter`), conversion traits, `AsRef/Borrow/ToOwned`, `Deref`/coercion, расширенная видимость `pub(...)`.
- Новые уроки в этих блоках пока каркасные: структура и точки входа готовы, детальный материал будет добавляться поэтапно.

## Расширение после базы

- Углубления по параллелизму: `11_concurrency/095`–`096`.
- Углубления по async: `12_async/102`–`105` (Pin/Unpin, spawn limits, cancellation).
- Углубления по low-level: `14_unsafe_and_ffi/112`.
- Расширение экосистемы: `16_ecosystem/125`–`128` (benchmarks, property testing, cargo tooling, tracing spans).

## Покрытие ключевых тем языка

- [x] Базовый синтаксис, управление потоком, функции.
- [x] Составные типы и коллекции.
- [x] Владение и заимствование, включая `Copy/Clone`.
- [x] Pattern matching как отдельный этап (`3_patterns`).
- [x] Трейты и обобщения, включая conversion traits, `AsRef/Borrow/ToOwned`, `Eq/Ord/Hash`.
- [x] Модули, видимость и системные атрибуты.
- [x] Практика стандартной библиотеки (`std::fs`, `std::path`, `std::env`, `std::process`, `std::time`, buffered I/O).
- [x] Расширения async/concurrency/unsafe добавлены в структуру курса.
- [x] Расширены темы тестирования и инструментов `cargo`.

## Как запустить любой урок

Каждый урок — самостоятельный Cargo-проект. Номер урока в пути совпадает со сквозным номером в таблице:

```bash
cd 5_error_handling/039_option
cargo run
```

## Порядок изучения

Блоки рекомендуется проходить по порядку — следующий опирается на предыдущий. Номера уроков (`001`, `002`, …) сквозные по всему курсу: чем больше число, тем позже тема в программе.

Особо важные блоки, без которых дальше делать нечего:
- **4** (владение и заимствование) — сердце Rust;
- **5** (`Option`, `Result`, `?`) — встречаются в каждой функции;
- **7** (трейты и обобщения) — без них вся стандартная библиотека выглядит магией.

Рекомендуемая последовательность старта: **1 → 2 → 3 → 4 → 5 → 6 → 7 → 8 → 9**, затем переход к блокам 10–16.
