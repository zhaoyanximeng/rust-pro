use std::time::Duration;
use tokio::runtime::*;
use tokio::time::*;
use chrono::Utc;



async fn job(i:i32)->i32 {
    sleep(Duration::from_secs(2)).await;
    println!("{}", i);
    i
}

fn main() {
    let rt = Builder::new_multi_thread().worker_threads(2).enable_time().build().unwrap();
    let start_time = Utc::now().time();
    rt.block_on(async{
        let r1 = job(123).await;
        let r2 = job(234).await;
        let r3 = job(345).await;
        println!("执行结果是:{},{},{}", r1, r2, r3);
    });
    let end_time = Utc::now().time();
    println!("耗时:{}", (end_time-start_time).num_milliseconds());
}
