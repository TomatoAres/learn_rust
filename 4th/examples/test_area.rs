use homework2::area::{Area, Circle, Square, Triangle};

fn main() {
    let c = Circle::new(10f64);
    let c_area = calc_area(&c);
    println!("c area is: {}", c_area);

    let s = Square::new(10f64);
    let s_area = calc_area(&s);
    println!("s area is : {}", s_area);

    let t = Triangle::new([3f64, 4f64, 5f64], 3f64, 4f64);
    let t_area = calc_area(&t);
    println!("t area is :{}", t_area);
}

fn calc_area(a: &impl Area) -> f64 {
    a.area()
}
