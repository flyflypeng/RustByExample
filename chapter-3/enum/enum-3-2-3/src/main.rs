use List::*;

enum List {
    // Cons: 元素结构体，包含链表的第一个元素和指向下一个节点的指针
    Cons(u32, Box<List>),
    // Nil：末尾节点，表示链表结束
    Nil,
}

impl List {
    // 创建一个空的List实例
    fn new() -> List {
        // Nil为List类型，完整名称为List::Nil
        Nil
    }

    // 处理一个List，向其头部插入新的元素，并返回该List
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // 返回 List 的长度
    fn len(&self) -> u32 {
        // 必须对 `self` 进行匹配（match），因为这个方法的行为取决于 `self` 的
        // 取值种类。
        // `self` 为 `&List` 类型，`*self` 为 `List` 类型，匹配一个具体的 `T`
        // 类型要好过匹配引用 `&T`
        match *self {
            // 这里不能得到tail的所有权，因为self是借用 的;
            // 因此使用一个tail链表的引用
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    // 返回列表的字符串输出格式
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    // 创建一个空的链表
    let mut list = List::new();

    // 向链表中插入元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 显示链表最后的状态
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
