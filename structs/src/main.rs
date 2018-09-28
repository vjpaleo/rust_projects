
struct Ractangle {
    width: u32,
    height: u32,
}

fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User{
        username: String::from("vijay"),
        email: String::from("vijay@singh.com"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    let rect1 = Ractangle {
        width: 10,
        height: 20,
    };

    println!("{}", area(&rect1));
}

fn area(ractangle: &Ractangle) -> u32 {
    ractangle.width * ractangle.height
}
