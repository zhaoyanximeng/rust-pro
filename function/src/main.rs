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

fn main() {
    let str = String::from("abc");
    let f = || {
        println!("{}", str);
    };
    // exec(f);
    // exec(f);

    // let u = User{id:1, info: f};
    // (u.info)();

    let u = User{id:1, info: &f};
    (u.info)();
}
