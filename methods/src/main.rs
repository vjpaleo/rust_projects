
struct Ractangle {
    width: u32,
    height: u32,
}

impl Ractangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    
    let rect1 = Ractangle {
        width: 10,
        height: 20,
    };

    println!("{}", rect1.area());
}