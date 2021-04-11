#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Nil;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

// 使用嵌套解构的方法结构了Point和Rectangle两个结构体
fn rect_area(r: &Rectangle) -> f32 {
    let &Rectangle {
        p1: Point { x: x1, y: y1 },
        p2: Point { x: x2, y: y2 },
    } = r;

    return (x2 - x1).abs() * (y2 - y1).abs();
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point = Point { x: 0.3, y: 0.5 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 使用结构体更新语法创建新的Point对象，这样可以使用到之前的point的字段
    let new_point = Point { x: 0.1, ..point };
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // 使用let绑定来解开point中的字段
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        p1: Point { x: my_x, y: my_y },
        p2: point,
    };

    // 实例化一个单元结构体
    // 以下划线开头的变量名称为常规名称，但是有一点特殊的是，就是如果它们未被使用的话，
    // 编译器不会报警告
    let _nil = Nil;

    let pair = Pair(1, 0.1);

    println!("pair contains: {:?} and {:?}", pair.0, pair.1);

    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    let rect = Rectangle {
        p1: Point { x: 0.3, y: 0.5 },
        p2: Point { x: 1.0, y: 1.0 },
    };

    let a = rect_area(&rect);
    println!("area of {rect:?} is {a}", rect = rect, a = a);
}
