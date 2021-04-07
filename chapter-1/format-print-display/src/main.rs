use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

// 定义一个struct里面包含一个vec列表的成员
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        // 起始括号
        write!(f, "[")?;

        // 遍历枚举vec中的所有数据
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ",")?;
            }

            write!(f, "{}", v)?;
        }

        // 终止括号
        write!(f, "]")
    }
}

fn main() {
    let minmax = MinMax(0, 14);
    println!("Compared structures");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Compared points");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let comp_value = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Compared Complex");
    println!("Display: {}", comp_value);
    println!("Debug: {:?}", comp_value);

    // Vec list test
    let list = List(vec![5, 6, 7, 8]);
    println!("{}", list);
}
