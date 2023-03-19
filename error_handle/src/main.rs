
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
