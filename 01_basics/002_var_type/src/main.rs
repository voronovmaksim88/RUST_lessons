//! Интерактивный урок по типам данных в Rust.
//!
//! ## Что вы узнаете
//!
//! - Какие есть числовые типы: целые со знаком (`i8`–`i128`), без знака (`u8`–`u128`),
//!   архитектурно-зависимые (`isize`, `usize`), числа с плавающей точкой (`f32`, `f64`).
//! - Как записывать числа в разных системах счисления (десятичной, двоичной,
//!   восьмеричной, шестнадцатеричной) и что такое байтовые литералы.
//! - Логический тип `bool` и символьный тип `char` (включая Unicode и эмодзи).
//! - Строковые срезы `&str` и их методы.
//! - Кортежи и массивы — составные типы данных.
//! - Изменяемость (`mut`) vs затенение (`shadowing`).
//! - Константы (`const`) и статические переменные (`static`).
//! - Преобразование типов (`as`) и вывод типов (type inference).
//!
//! ## Структура
//!
//! Главный файл содержит интерактивное меню для выбора темы. Каждая тема
//! вынесена в отдельный модуль в папке `lessons/`.
//!
//! ## Как запустить
//!
//! ```bash
//! cargo run
//! ```

//! Выберите номер темы в меню (1–12) или `0` для просмотра всех уроков подряд.
//! Для выхода нажмите `q`.

mod lessons;

use std::io::{self, Write};

// ============================================================
// Урок 002. ТИПЫ ДАННЫХ В RUST
// ============================================================

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║                                                                ║");
    println!("║            Урок 002. ТИПЫ ДАННЫХ В RUST                      ║");
    println!("║                                                                ║");
    println!("╚════════════════════════════════════════════════════════════════╝");

    loop {
        println!("┌────────────────────────────────────────────────────────────────┐");
        println!("│                    ВЫБЕРИТЕ ТЕМУ УРОКА:                        │");
        println!("└────────────────────────────────────────────────────────────────┘");

        println!("\n  ЧИСЛОВЫЕ ТИПЫ:");
        println!("    1.  Целые числа со знаком (i8, i16, i32, i64, i128)");
        println!("    2.  Целые числа без знака (u8, u16, u32, u64, u128)");
        println!("    3.  Архитектурно-зависимые типы (isize, usize)");
        println!("    4.  Способы записи чисел (системы счисления)");
        println!("    5.  Числа с плавающей точкой (f32, f64)");

        println!("\n  ДРУГИЕ ПРИМИТИВНЫЕ ТИПЫ:");
        println!("    6.  Логический тип (bool)");
        println!("    7.  Символьный тип (char)");
        println!("    8.  Строковые срезы (&str)");

        println!("\n  СОСТАВНЫЕ ТИПЫ:");
        println!("    9.  Кортежи и массивы (tuples & arrays)");

        println!("\n  РАБОТА С ПЕРЕМЕННЫМИ:");
        println!("    10. Изменяемость и затенение (mut & shadowing)");
        println!("    11. Константы и статические переменные (const & static)");
        println!("    12. Преобразование и вывод типов (casting & inference)");

        println!("\n  СПЕЦИАЛЬНЫЕ КОМАНДЫ:");
        println!("    0.  Показать все уроки подряд");
        println!("    q.  Выход из программы");

        print!("\n>>> Ваш выбор: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Не удалось прочитать ввод");

        let choice = choice.trim();

        match choice {
            "1" => {
                lessons::signed_integers::show();
                wait_for_enter();
            }
            "2" => {
                lessons::unsigned_integers::show();
                wait_for_enter();
            }
            "3" => {
                lessons::arch_types::show();
                wait_for_enter();
            }
            "4" => {
                lessons::number_literals::show();
                wait_for_enter();
            }
            "5" => {
                lessons::floats::show();
                wait_for_enter();
            }
            "6" => {
                lessons::boolean::show();
                wait_for_enter();
            }
            "7" => {
                lessons::char_type::show();
                wait_for_enter();
            }
            "8" => {
                lessons::strings::show();
                wait_for_enter();
            }
            "9" => {
                lessons::tuples_arrays::show();
                wait_for_enter();
            }
            "10" => {
                lessons::mutability::show();
                wait_for_enter();
            }
            "11" => {
                lessons::constants::show();
                wait_for_enter();
            }
            "12" => {
                lessons::type_operations::show();
                wait_for_enter();
            }
            "0" => {
                show_all_lessons();
                wait_for_enter();
            }
            "q" | "Q" | "exit" | "quit" => {
                println!("\n");
                println!("╔════════════════════════════════════════════════════════════════╗");
                println!("║                                                                ║");
                println!("║              🎓 Спасибо за изучение Rust! 🎓                    ║");
                println!("║                    До новых встреч! 👋                         ║");
                println!("║                                                                ║");
                println!("╚════════════════════════════════════════════════════════════════╝");
                println!();
                break;
            }
            "" => {
                // Пустой ввод - игнорируем
                continue;
            }
            _ => {
                println!("\n❌ Неверный выбор! Пожалуйста, выберите номер от 1 до 12, 0 для всех уроков, или 'q' для выхода.");
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
        }
    }
}

fn show_all_lessons() {
    println!("\n");
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║           ПОКАЗАТЬ ВСЕ УРОКИ ПОДРЯД                            ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();
    println!("Сейчас будут показаны все уроки по порядку.");
    println!("Между уроками нажимайте Enter для продолжения...");
    println!();

    wait_for_enter();

    lessons::signed_integers::show();
    wait_for_enter();

    lessons::unsigned_integers::show();
    wait_for_enter();

    lessons::arch_types::show();
    wait_for_enter();

    lessons::number_literals::show();
    wait_for_enter();

    lessons::floats::show();
    wait_for_enter();

    lessons::boolean::show();
    wait_for_enter();

    lessons::char_type::show();
    wait_for_enter();

    lessons::strings::show();
    wait_for_enter();

    lessons::tuples_arrays::show();
    wait_for_enter();

    lessons::mutability::show();
    wait_for_enter();

    lessons::constants::show();
    wait_for_enter();

    lessons::type_operations::show();

    println!("\n");
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║                                                                ║");
    println!("║        🎉 ВСЕ УРОКИ ЗАВЕРШЕНЫ! ОТЛИЧНАЯ РАБОТА! 🎉              ║");
    println!("║                                                                ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
}

fn wait_for_enter() {
    println!("\n");
    println!("─────────────────────────────────────────────────────────────────");
    print!("  Нажмите Enter для продолжения...");
    io::stdout().flush().unwrap();
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();
}
