fn main() {
    println!("┌──────────────────────────────────────────────────────────────────────┐");
    println!("│                 ПОЛНОЕ РУКОВОДСТВО ПО МАССИВАМ В RUST                │");
    println!("└──────────────────────────────────────────────────────────────────────┘\n");
    println!("Кратко о массивах:");
    println!("• Array [T; N] — фиксированный по размеру контейнер: длина N входит в тип,");
    println!("  поэтому [i32; 3] и [i32; 4] — разные типы и не взаимозаменяемы");
    println!("• Данные хранятся на стеке, быстрый доступ по индексу");
    println!("• Нельзя менять длину, но можно изменять элементы (если массив mut)");
    println!("• Хороши, когда размер известен на этапе компиляции\n");
    basics::run();
    access_len::run();
    iteration::run();
    defaults_methods_copy::run();
    multidim_slices::run();
    iterators::run();
    safety::run();
    extra_methods::run();
    functions::run();
    array_vs_vec::run();
    advanced_init::run();
    compare_search::run();
    summary::run();
    println!("\n✅ Урок по массивам в Rust завершён!");
}

mod basics;
mod access_len;
mod iteration;
mod defaults_methods_copy;
mod multidim_slices;
mod iterators;
mod safety;
mod extra_methods;
mod functions;
mod array_vs_vec;
mod advanced_init;
mod compare_search;
mod summary;
