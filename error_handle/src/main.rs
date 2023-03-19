
fn test(i:i32) ->Result<String, i32> {
    if i < 10 {
        Ok(String::from("success"))
    } else {
        Err(0)
    }
}

fn main() {
   let ret = test(11);
    if let Ok(s) = ret {
        println!("成功:{}", s);
    } else if let Err(i) = ret {
        println!("失败:{}", i);
    }
}
