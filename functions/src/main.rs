fn main() {
    println!("Hello, world!");
    another_function();
    another_function2(56);

    let a = sum(45, 32);
    println!("The value of a = {}", a);
}

fn another_function() {
    println!("Another function");
}

fn another_function2(x:i32) {
    println!("The value of x = {}", x);
}

fn sum(x:i32, y:i32) -> i32 {
    let s = x + y;
    s
}