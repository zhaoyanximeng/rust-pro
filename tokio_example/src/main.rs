use chrono::Utc;
use std::time::Duration;
use tokio::runtime::*;
use tokio::task;
use tokio::time::*;

async fn job(i: i32) -> i32 {
    sleep(Duration::from_secs(2)).await;
    println!("{}", i);
    i
}

// fn main() {
//     let rt = Builder::new_multi_thread().worker_threads(2).enable_time().build().unwrap();
//     let h1 = rt.spawn(job(123));
//     let h2 = rt.spawn(job(234));
//     let h3 = rt.spawn(job(456));
//     let start_time = Utc::now().time();
//     rt.block_on(async{
//         let r1 = h1.await.unwrap();
//         let r2 = h2.await.unwrap();
//         let r3 = h3.await.unwrap();
//         println!("执行结果是:{},{},{}", r1, r2, r3);
//     });
//     let end_time = Utc::now().time();
//     println!("耗时:{}", (end_time-start_time).num_milliseconds());
// }

// #[tokio::main(flavor = "multi_thread")]
// async fn main() {
//     let handler = tokio::runtime::Handle::current();
//     let h1 = handler.spawn(job(123));
//     let h2 = handler.spawn(job(234));
//     let h3 = handler.spawn(job(456));
//     let start_time = Utc::now().time();
//     let r1 = h1.await.unwrap();
//     let r2 = h2.await.unwrap();
//     let r3 = h3.await.unwrap();
//     println!("执行结果是:{},{},{}", r1, r2, r3);
//     let end_time = Utc::now().time();
//     println!("耗时:{}", (end_time - start_time).num_milliseconds());
// }

#[tokio::main]
async fn main() {

    let h1 = task::spawn(async {
        sleep(Duration::from_secs(5)).await;
        // println!("{}", "h1");
        println!("h1已经完成了");
        "h1"
    });

    let h2 = task::spawn(async {
        sleep(Duration::from_secs(2)).await;
        // println!("{}", "h2");
        println!("h2已经完成了");
        "h2"
    });

    // h1.await.unwrap();
    // h2.await.unwrap();
    // let (_,serde_json) = tokio::join!(h1, h2);
    let ret1 = h1.await.unwrap();
    println!("{}", ret1);
    let ret2 = h2.await.unwrap();
    println!("{}", ret2);

    println!("{}", "abc");
}