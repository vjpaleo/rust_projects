fn main() {
    let mut array: [i32; 3] = [0; 3];
    array[1] = 1;
    for x in &array {
        print!("{} ", x);
    }
}