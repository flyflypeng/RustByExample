#![allow(dead_code)]

// 创建一个 `enum`（枚举）来对 web 事件分类,
// 枚举类型中各个元素的类型可以各不相同，相互独立
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char), //枚举元素类型可以是元组结构体类型
    PasteString(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("key pressed {}", c),
        WebEvent::PasteString(s) => println!("pasted \"{}\"", s),
        // 将x,y从event对象中解构出来，赋值给x,y
        WebEvent::Click { x, y } => println!("clicked at x={}, y={}", x, y),
    }
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// 创建一个类型别名
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` 从一个字符串切片中创建一个具有所有权的 `String`
    let pasted = WebEvent::PasteString("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let opt = Operations::Add;
    let result = opt.run(1, 9);
    println!("result with opt: {}", result);
}
