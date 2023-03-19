fn main() {
    let a = 5;
    let b = Box::new(a);
    println!("{}", a);
    println!("{}", a==*b);
}
