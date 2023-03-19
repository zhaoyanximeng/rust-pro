macro_rules! echo {
    () => {
        println!("zyxm");
    };
    ($exp:expr) => {
        println!("{}", stringify!($exp)); // stringify!取出表达式
    };
    // 可变参数，+表示至少有一个，*表示可以没有
    ($($exp:expr), +) => {
        $(
            println!("{}", stringify!($exp));
        )+
    };
}

macro_rules! func {
    ($fn_name:ident) => {
        fn $fn_name() {
            println!("my function,name is:{}", stringify!($fn_name));
        }
    };
}