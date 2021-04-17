fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding = 2;

    println!("After mutation: {}", mutable_binding);

    // 不能对不可变的变量重新绑定新的值
    // _immutable_binding = 2;
}
