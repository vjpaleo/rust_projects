fn main() {
    
    let x = 1;

    {
        let x = 2;
        println!("The value of x is {}", x);
    }

    if x == 1 {
        let a = 10;
        println!("The value of a is {}", a);
    }
}
    
