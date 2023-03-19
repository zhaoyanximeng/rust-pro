/*// Box
fn main() {
    let a = 5;
    let b = Box::new(a);
    println!("{}", a);
    println!("{}", a==*b);
}*/


// Rc



use std::rc::Rc;

fn test(s: Rc<String>) {
    println!("{}",s);
}

fn main() {
    let a = Rc::new(String::from("abc"));
    let b = a.clone();
    let c = a.clone();
    test(b);
    println!("{}", a);
    println!("{}", Rc::strong_count(&a));
}