fn main() {
    let number_u8: u8 = 10;
    //let number_u16: u16 = number_u8; // ошибка
    let number_u16: u16 = number_u8 as u16;
    println!("number_u16 = {}", number_u16);

    let number_u16: u16 = 257;
    let number_u8: u8 = number_u16 as u8;
    println!("number_u8 = {} произошло переполнение", number_u8);

}
