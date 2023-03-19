/*
fn test(i:i32) ->Result<String, i32> {
    if i < 10 {
        Ok(String::from("success"))
    } else {
        Err(0)
    }
}

fn main() {
    let ret = test(3);
    // if let Ok(s) = ret {
    //     println!("成功:{}", s);
    // } else if let Err(i) = ret {
    //     println!("失败:{}", i);
    // }

    // match ret {
    //     Ok(s) => {
    //         println!("成功:{}", s);
    //     },
    //     Err(i) => {
    //         println!("失败:{}", i);
    //     }
    // }

    // 出错给默认值
    // println!("{}", ret.unwrap_or(("my default value").to_string()));

    // 出错进行业务处理
    println!("{}", ret.unwrap_or_else(|error|{
        error.to_string()
    }))
}
*/

/*// 使用?进行错误传播
fn main() {
    let ret = test();
    println!("{}", ret.unwrap_or_else(|error| {
        error
    }))
}

fn step1()->Result<String, String> {
    Err("err1".to_string())
}

fn step2()->Result<String, String> {
    Err("err2".to_string())
}

fn test()->Result<String, String> {
    // let ret1 = step1();
    // if let Err(_) = ret1 {
    //     Err("fail1".to_string())
    // } else {
    //     let ret2 = step2();
    //     if let Err(_) = ret2 {
    //         Err("fail2".to_string())
    //     } else {
    //         Ok("done".to_string())
    //     }
    // }

    step1()?;
    step2()?;
    Ok("done".to_string())
}*/
use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let path = "./src/text.txt";
    let mut f = File::open(path).expect("open fail");
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("read fail");

    println!("{}", buf);
}