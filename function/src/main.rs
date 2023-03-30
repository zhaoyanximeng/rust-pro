fn exec<F> (f: F)
    where
    F:Fn()
{
    f();
}

fn main() {
    let str = String::from("abc");
    let f = || {
        println!("{}", str);
    };
    exec(f);
    exec(f);
}
