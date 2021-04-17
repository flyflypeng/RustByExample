fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // 编译器会对未使用到的变量产生警告
    // 可以给变量名前面加上下划线`_`来消除警告
    let _unused_variable = 3u32;

    // 改正：在变量名前加上下划线以消除警告
    let nosiy_unused_variable = 2u32;
}
