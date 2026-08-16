#[derive(Debug)]
enum IPAddr{
    V4(u8, u8, u8, u8),
    V6(String)
}

fn main() {
    let ip = IPAddr::V4(127, 0,0,1);
    println!("{:#?}", ip);
}
