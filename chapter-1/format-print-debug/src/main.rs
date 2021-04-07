#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));

    // cut here---------------------------------
    let name = "Perter";
    let age = 27;
    let person = Person { name, age };

    // 加上#号后会有更好的输出效果，类似golang里面输出中的加上#号，输出时会增加结构体的成员名称信息
    println!("{:#?}", person);
}
