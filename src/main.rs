mod lib;

fn main() {
    let age:i8 = 19;
    println!("my age is {}", age);
    println!("u8 max num is {}, u8 min num is {}",i8::MAX, i8::MIN)
}


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