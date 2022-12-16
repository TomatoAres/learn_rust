fn main() {
    let arr = [1, 2, 3, 4, 5, 6, u32::MAX];
    match homework2::sum::sum(&arr) {
        Ok(s) => println!("sum: {}", s),
        Err(msg) => println!("sum err: {}", msg),
    }
}
