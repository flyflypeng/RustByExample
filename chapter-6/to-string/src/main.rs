use std::string::ToString;

struct Circle {
    radius: u32,
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 5 };
    println!("Circle {}", circle.to_string());

    // FromStr类型转换
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed: i32 = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}
