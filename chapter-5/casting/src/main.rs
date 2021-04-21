// 不显示类型转换产生的溢出警告
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // 将下面注释掉；Rust不提供隐式类型转换
    // let integer: u8 = decimal;

    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // 当把任何有符号类型的值转换为无符号类型T时，会不断地加上或者减去(std::T::MAX + 1)
    // 知道值位于新类型T的范围内

    // 1000 - 256 - 256 - 256 = 232
    println!("1000 as a u16 is: {}", 1000 as u16);
    // -1 + 256
    println!("1000 as a u8 is: {}", 1000 as u8);

    // 取模运算
    println!("1000 mod 256: {}", 1000 % 256);

    // 无符号数到有符号类型T转换时，如果按位转换后的结果的最高位为1，则对应该值为负
    println!("128 as i16 is: {}", 128 as i16);
    println!("128 as i8 is: {}", 128 as i8);

    println!("1000 as a i8 is: {}", 1000 as i8);
    println!("232 as a i8 is: {}", 232 as i8);
}
