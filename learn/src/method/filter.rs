pub fn filter(html:&str) {
    println!("{}", html.len());
    for char in html.chars() {
        println!("{}", char)
    }

    match html.len() {
        0..=12=>println!("{}", "太短了"),
        13..=15 => println!("{}", "ok"),
        _ => println!("{}", "太长了"),
    }

    for i in 1..=3{
        println!("{}",i)
    }
    for i in 1..3{
        println!("{}",i)
    }
}