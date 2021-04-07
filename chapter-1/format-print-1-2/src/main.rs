fn main() {
    println!("{} days", 31);

    // 可以在format字符串中用位置指代具体的变量
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 可以在format字符串中，通过名称指代具体的变量
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // 通过 :b 的形式指定用什么格式输出数字的二进制格式
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );

    // 通过 :> 符号表示向右对齐，总的字符串对齐的宽度是width指定的6
    println!("{number:>width$}", number = 1, width = 6);
    println!("{number:>width$}", number = 10, width = 6);

    // 通过 :>0 符号表示向右对齐并且左边的pad用符号0填充
    println!("{number:>0width$}", number = 1, width = 6);
    // 下面的这种格式报错，不支持任意的符号补全左边的pad
    // println!("{number:>2width$}", number = 1, width = 6)

    // 下面的语句报错，因为{1}位置的变量没有被指定
    // println!("My name is {0}, {1} {0}", "Bond");

    // 输出浮点数中小数点后指定位数
    let pi = 3.141592;
    println!("pi is {0:.3}", pi);
}
