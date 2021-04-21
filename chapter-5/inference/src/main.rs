fn main() {
    // 根据变量定义时的类型说明，编译器知道该类型是u8
    let elem = 5u8;

    // 创建一个空向量，但是现在编译器还不知道向量里存放的是什么类型的数据
    let mut vec = Vec::new();

    vec.push(elem);
    // 试试把上面的`vec.push(elem)`注释掉，看看会发生什么

    println!("{:?}", vec);
}
