fn exec<F> (f: F)
    where
    F:Fn()
{
    f();
}

// struct User<F>
// where
// F:Fn()
// {
//     id: i32,
//     info: F,
// }

// struct User<'a>
// {
//     id: i32,
//     info:&'a dyn Fn(),
// }

// fn main() {
//     let str = String::from("abc");
//     let f = || {
//         println!("{}", str);
//     };
//     // exec(f);
//     // exec(f);
//
//     // let u = User{id:1, info: f};
//     // (u.info)();
//
//     let u = User{id:1, info: &f};
//     (u.info)();
// }

// fn myfn<F: FnMut()>(mut f:F) {
//     f();
// }
//
// fn main() {
//     let mut str = String::from("abc");
//     let mut f = || {
//         let a = &mut str;
//         a.push_str("123");
//     };
//     f();
//     myfn(f);
//     println!("{}", str);
// }

fn myfn<F: FnOnce()>(f:F) {
    f();
}

// fn main() {
//     let str = String::from("abc");
//     let f = || {
//         let mut  a = str;
//         a.push_str("123");
//         println!("{}", a)
//     };
//     myfn(f);
// }

// fn main() {
//     let p = "abc".to_string();
//     let i = 12;
//     let a = change(&p, &i);
//     println!("{}", a)
// }
//
// fn change<'a>(str1:&'a String, i: &i32) ->&'a String {
//    str1
// }

#[derive(Debug)]
struct User<'a> {
    name: &'a String
}

impl <'a> User<'a> {
    fn get_name(&self)->&String {
        return &self.name;
    }
}

fn main() {
    // let name = String::from("zyxm");
    // let u:User;
    // {
    //     u = User { name: &name };
    // }
    // println!("{:?}", u);

    let str1 = "zyxm";
    let u = User{name: &String::from("coleary")};
    let str3 = max_str(str1,&u);
    println!("{}", str3);
}

fn max_str<'a, 'b>(a:&'a str, b:&'b User) ->&'a str where 'b:'a {
    if a.len() >= b.get_name().len() {
        return a;
    }
    b.get_name().as_str()
}