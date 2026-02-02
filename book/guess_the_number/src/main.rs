use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    #[cfg(windows)]
    let _ = colored::control::set_virtual_terminal(true);
    println!("Я загадал число от 1 до 100. Угадай его за 7 попыток чтобы выиграть!");
    let secret_number = rand::rng().random_range(1..=100);
    let mut attempts = 0;

    loop {
        println!("Попытка #{}", attempts + 1);
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Ошибка чтения ввода");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        attempts += 1;

        if attempts >= 7 && guess != secret_number {
            println!("{}", "ты проиграл".red());
            println!("Загаданное число было: {}", secret_number);
            break;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком мало!"),
            Ordering::Greater => println!("Слишком много!"),
            Ordering::Equal => {
                println!(
                    "{}",
                    format!(
                        "Поздравляю! Ты угадал число {} за {} попыток!",
                        secret_number, attempts
                    )
                    .green()
                );
                break;
            }
        }
    }

    println!("Нажми Enter, чтобы завершить программу...");
    let mut exit_input = String::new();
    let _ = io::stdin().read_line(&mut exit_input);
}
