#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }

    fn gen_sqaure(size: u32) -> Rectangle {
        return Rectangle {
            width: size,
            height: size,
        };
    }
    fn can_cover(&self, other: &Rectangle)->bool
    {
        return self.width >= other.width && self.height >= other.height;
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let a = rect.area();
    let rect1 = Rectangle::gen_sqaure(40);
    println!("{}, {}", a, rect1.area());
    println!("{:?} \n {:#?}", rect, rect1);
    let rect2 = Rectangle::gen_sqaure(50);
    println!("{}", rect2.can_cover(&rect));
}
