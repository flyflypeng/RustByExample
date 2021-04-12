#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    use Status::{Poor, Rich};
    // 自动地 `use` `Work` 内部的各个名称
    use Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money"),
        Poor => println!("The poor have no money...."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldier fight!"),
    }
}
