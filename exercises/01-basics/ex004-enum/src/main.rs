use std::{arch::naked_asm, thread::LocalKey};

#[derive(Debug)]
enum IPAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum location {
    Point(i32),
    Range(i32, i32),
}

// impl<T> Option<T> {
    // fn unwrap_or(self, other: T) -> T {
        // match self {
            // Some(t) => t,
            // None => other,
        // }
    // }
// }

fn decr_twice_v1(n: u32) -> Option<u32> {
    if n == 0 {
        None
    } else if n == 1 {
        None
    } else {
        Some(n - 2)
    }
}


fn decr_twice_v2(n: u32) -> Option<u32> {
    match n {
        0 => None,
        1 => None,
        _ => Some(n - 2),
    }
}

fn main() {
    let ip = IPAddr::V4(127, 0, 0, 1);
    println!("{:#?}", ip);

    let opt: Option<String> = Some(String::from("Hello"));
    match &opt {
        Some(s) => println!("The value is: {}", s),
        None => println!("No value"),
    }
    if let Some(s) = &opt {
        println!("in if let The value is: {}", s);
    }
    println!("{:#?}", opt);

    let l: location = location::Range(0, 5);
    let n = match l {
        location::Point(1) => 1,
        location::Range(_, n) => n,
        location::Range(0, _) => 0,
        _ => -2,
    };
    println!("{n}");
    println!("{:?}", decr_twice_v1(0));
    println!("{:?}", decr_twice_v2(0));
}
