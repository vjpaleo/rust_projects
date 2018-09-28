fn main() {
    let s = dangle();
    println!("the value of s = {}", s);
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}
