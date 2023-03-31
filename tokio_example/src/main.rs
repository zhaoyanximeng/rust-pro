use chrono::Utc;
use std::time::Duration;
use tokio::runtime::*;
use tokio::{task, time};
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

// #[tokio::main(flavor="current_thread")]
// async fn main() {
//
//     let h1 = task::spawn(async {
//         sleep(Duration::from_secs(5)).await;
//         // println!("{}", "h1");
//         println!("h1已经完成了");
//         "h1"
//     });
//
//     let h2 = task::spawn(async {
//         sleep(Duration::from_secs(2)).await;
//         // println!("{}", "h2");
//         println!("h2已经完成了");
//         "h2"
//     });
//
//     let h3 = task::spawn_blocking( ||{
//         std::thread::sleep(Duration::from_secs(8));
//         println!("h3已经完成了");
//         "h3"
//     });
//
//     // h1.await.unwrap();
//     // h2.await.unwrap();
//     // let (_,serde_json) = tokio::join!(h1, h2);
//     let ret1 = h1.await.unwrap();
//     println!("{}", ret1);
//     let ret2 = h2.await.unwrap();
//     println!("{}", ret2);
//     let ret3 = h3.await.unwrap();
//     println!("{}", ret3);
//
//     println!("{}", "abc");
// }

// #[tokio::main]
// async fn main() {
//     let t1 = timeout(Duration::from_secs(2), async {
//        sleep(Duration::from_secs(5)).await;
//         println!("任务完成");
//         panic!("内部报错了")
//     });
//
//    let h1 = task::spawn(t1);
//    h1.await.expect("内部任务出错").expect("超时出错");
// }

use tokio::sync::oneshot;

// oneshot 一对一通道
#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel::<i32>();

    let t1 = task::spawn(async {
        time::sleep(Duration::from_secs(2)).await;
        tx.send(123).unwrap();
    });
    let ret = rx.await.unwrap();
    println!("{}", ret);
}