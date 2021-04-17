fn main() {
    let long_lived_binding = 1;

    {
        // 此绑定只存在于该scope中
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        // 此绑定遮蔽了外面一层scope中的绑定
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }

    // 报错：`short_lived_binding` 在此作用域上不存在
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);

    // 此绑定同样遮蔽了前面的绑定
    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);
}
