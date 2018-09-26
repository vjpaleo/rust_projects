fn main() {
    let a = 10;

    if a < 10 {
        println!("Condition is true");
    } else if a == 10 {
        println!("Condition is unknown");
    } else {
        println!("Condition is false");
    }

    let mut b = 0;

    while b < 10 {
        println!("The value of b = {}", b);
        b = b + 1
    }

    let mut c = 10;
    loop {
        println!("The value of c = {}", c);
        if c == 0 {
            break;
        }
        c = c - 1;
    }

    let arr = [1, 2, 3, 5, 6, 7, 8, 9, 10];

    for ele in arr.iter() {
        println!("The value of ele = {}", ele);
    }
}
