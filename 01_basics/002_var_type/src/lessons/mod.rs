//! Модули уроков по типам данных в Rust.
//!
//! Каждый подмодуль реализует одну тему и содержит функцию `show()`,
//! которая выводит пояснения и примеры кода.
//!
//! ## Список модулей
//!
//! | № | Модуль | Тема |
//! |---|--------|------|
//! | 1 | `signed_integers` | Целые числа со знаком (`i8`–`i128`) |
//! | 2 | `unsigned_integers` | Целые числа без знака (`u8`–`u128`) |
//! | 3 | `arch_types` | Архитектурно-зависимые типы (`isize`, `usize`) |
//! | 4 | `number_literals` | Способы записи чисел |
//! | 5 | `floats` | Числа с плавающей точкой (`f32`, `f64`) |
//! | 6 | `boolean` | Логический тип (`bool`) |
//! | 7 | `char_type` | Символьный тип (`char`) |
//! | 8 | `strings` | Строковые срезы (`&str`) |
//! | 9 | `tuples_arrays` | Кортежи и массивы |
//! | 10 | `mutability` | Изменяемость и затенение |
//! | 11 | `constants` | Константы и статические переменные |
//! | 12 | `type_operations` | Преобразование и вывод типов |

pub mod arch_types;
pub mod boolean;
pub mod char_type;
pub mod constants;
pub mod floats;
pub mod mutability;
pub mod number_literals;
pub mod signed_integers;
pub mod strings;
pub mod tuples_arrays;
pub mod type_operations;
pub mod unsigned_integers;
