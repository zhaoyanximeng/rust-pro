// fn main() {
//     let a: User = User{id : 123, ..Default::default()};
//     let b = a.clone();
//     println!("{:?},{:?}", a,  b);
// }

use std::fmt::Formatter;

impl Clone for User{
    fn clone(&self) -> Self {
        User{
            id: self.id,
            name: self.name.clone() + "被克隆",
            age: self.age,
        }
    }
}

#[derive(Debug, Default)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

fn load_user(u:&mut User) {
    u.age = 42;
}

// fn main() {
//     let mut user = User{id: 123, ..Default::default()};
//     load_user(&mut user);
//     println!("a={:?}", user);
// }

impl User {
    fn new()->Self {
        User{..Default::default()}
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "用户id是{}，用户年龄是{}", self.id, self.age)
    }
}

fn main() {
    let u = User::new();
    println!("{}", u);
}