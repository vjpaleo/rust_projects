fn main() {
    let mut s = String::from("Hello");

    s.push_str(", world!");
    
    let s2 = s;
    drop(s);
    println!("{}", s2);

}
