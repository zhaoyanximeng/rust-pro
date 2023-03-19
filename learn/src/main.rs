use std::ops;
use std::ptr::addr_of_mut;

// mod lib;
mod models;
mod api;

/*fn main() {
    let age:i8 = 19;
    println!("my age is {}", age);
    println!("u8 max num is {}, u8 min num is {}",i8::MAX, i8::MIN)
}*/


/*// 包引用练习
mod lib; // 会默认加载同级目录下lib.rs文件里的内容和lib/mod.rs

fn main() {
    lib::helper::show_me();
    lib::config::config::show_version();
}
*/
/*// 函数传参练习
use crate::method::user::get_user;

mod method;

fn main(){
    get_user(1);
    get_user(2);
    get_user(3);
}*/


/*// 表达式练习
mod method;

fn main() {
    // let name:&'static str = "zyxm";
    let uid = 1;
    println!("{}", method::user::get_username(uid));

    let uid = 2;
    println!("{}", method::user::get_username(uid));

    let uid = 3;
    println!("{}", method::user::get_username(uid));
}*/


/*// match和遍历练习
mod method;

fn main() {
    let html = "你好，zyxm";
    method::filter::filter(html);
}*/

/*// String操作
use std::string;

fn main() {
    let first_name = String::from("zy");  // &str
    let last_name = String::from("xm");

    // let name = first_name + &last_name + "a"; // &str;  &String=>&str
    let name = format!("{}{}", first_name, last_name);

    println!("{}", name);
}*/


// // 打印指针
// fn main() {
//     let name = String::from("abc"); // name是"abc"的所有者，它只能有一个所有者
//     // let you = String::from("abc");
//     // let you = &name
//     let you = name.clone();
//     println!("长度是{}，容量是{}，指针是{:p}", name.len(), name.capacity(), name.as_ptr());
//     println!("长度是{}，容量是{}，指针是{:p}", you.len(), you.capacity(), you.as_ptr());
//     let name2 = name;
//     println!("长度是{}，容量是{}，指针是{:p}", name2.len(), name2.capacity(), name2.as_ptr());
// }


/*// 参数所有权
fn show_ptr(s: &String) {    // s得到了所有者
    println!("{:p}", s.as_ptr());
}

fn change(s: &mut String) {
    s.push_str("_19");
}

fn main() {
    let me = String::from("zyxm");
    show_ptr(&me);

    println!("{:p}", me.as_ptr());

    let mut name = String::from("zyxm");
    change(&mut name);
    println!("{}", name);
}*/

/*// 结构体
#[derive(Debug)]
struct User{
    name: String,
    age: u8,
}

impl User {
    fn version(&self) {
        println!("1.0")
    }
    fn to_string(&self)->String{
        return String::from(format!("my name is: {}, my age is: {}", &self.name, &self.age));
    }
}

fn main() {
    let me = User{name:String::from("zyxm"), age: 19};
    println!("{:#?}", me);
}*/

/*// 数组&元组
fn main() {
    let mut tags:[u8;3]=[0;3];
    println!("{:?}", tags);
    for item in tags.iter() {
        println!("{:#?}", item);
    }
    for (i,item) in tags.iter().enumerate() {
        println!("index:{}, value:{}", i, item);
    }
    for i in 0..tags.len() {
        tags[i] = i as u8;
    }
    println!("{:#?}", tags);

    let my:(&str, u8) = ("zyxm", 17);
    println!("{:#?}", my.1)
}*/

/*// 范型
use models::user::UserInfo;

fn set_user(mut u:&mut UserInfo) {
    u.user_id = 100;
    u.user_name = String::from("zyxm");
    u.user_age = 17;
    u.user_tags = ["go", "rust", "c++", "c", "java"]
}

fn main() {
    // let mut user = models::user::new_user_info();
    // set_user(&mut user);
    // println!("{:#?}", user)

    let mut user_score1 = models::user::new_user_score_b();
    user_score1.user_id = "sfs";
    user_score1.score = 10.0;
    println!("{:?}", user_score1.get_user_type());
}*/

/*// trait
use models::book_model::*;
use api::product::*;
use models::Phone;
use models::Book;
use crate::api::Stock;

// fn sum<T: Products, U: Products>(p1: T, p2: U) {
//     println!("商品总价是{}", p1.get_price() + p2.get_price())
// }

fn sum(p1: Book, p2: Book) {
    println!("商品总价是{}", p1 + p2);
}

fn show_product<T: Products>(p: T) {
    println!("商品价格是{}", p.get_price())
}

fn show_detail<T: Products + Stock>(p: T) where T: Products + Stock {
    println!("商品库存是：{}", p.get_stock())
}

fn main() {
    // let book:Book = Products::new(100, 20.0);
    // // show_product(book);
    // let phone:Phone = Products::new(102, 30.0);
    // sum(book, phone)

    // let book:Book = Products::new(100, 20.0);
    // show_detail(book);

    let book1:Book = Products::new(100, 20.0);
    let book2:Book = Products::new(100, 30.0);
    sum(book1 , book2)
}*/

/*// vector
fn main() {
    let mut tags = vec!["php", "java"];
    tags.push("go");
    println!("{:?}", tags);

    let mut tags1:Vec<i32> = Vec::new();
    tags1.push(1);
    tags1.push(2);
    for i in 0..tags1.len() {
        println!("{:?}", tags1[i]);
    }
    println!("{:?}", tags1);

    for i in &mut tags1 {
        *i = *i + 10;
    }
    println!("{:?}", tags1);
}*/


/*// enum
#[derive(Debug)]
enum Sex {
    Male(String, u8),
    Female(String)
}

#[derive(Debug)]
struct User {
    id: i32,
    sex: Option<String>,
}

fn check(u:User) {
    // match u.sex {
    //     Sex::Male => {
    //         println!("{}", "男性");
    //     },
    //     Sex::Female => {
    //         println!("{}", "女性");
    //     },
    // };

    // if let Some(s) = u.sex{
    //     println!("{}", s);
    // }

    println!("{}", u.sex.unwrap());
}

fn main() {
    let u = User{id: 1, sex: Some(String::from("M")) };
    check(u);
}*/

// 宏
#[macro_use]
mod macros;

fn main() {
    echo!();
    echo!("abc");
    let a = 3;
    echo!(a==3);
    echo!("ab", "bc", "cd");
}