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

struct User<'a>
{
    id: i32,
    info:&'a dyn Fn(),
}

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

fn main() {
    let str = String::from("abc");
    let f = || {
        let mut  a = str;
        a.push_str("123");
        println!("{}", a)
    };
    myfn(f);
}