/*use std::thread;
use std::time::Duration;
use futures::executor::block_on;

async fn getScore()->i32 {
    100
}

async fn getuser()->String {
    thread::sleep(Duration::from_secs(2));
    format!("user is {}, score is {}", "zyxm", getScore().await)
}

fn main() {
    let ret = block_on(getuser());
    println!("{}", ret);
    println!("{}", "done");
}*/

/*use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let server = TcpListener::bind("0.0.0.0:8080").unwrap();
    let stream = server.accept().unwrap();
    for s in server.incoming(){
        let stream = s.unwrap();
        handler(stream);
    }
}

fn handler(mut stream:TcpStream) {
    let rsp = String::from("http/1.1 200 Ok\r\n\r\n rust server");
    stream.write(rsp.as_bytes()).unwrap();
}*/

/*use std::time::Duration;
use tokio::time::timeout;
async fn job() -> String {
    String::from("abc")
}


#[tokio::main]
async fn main(){
    let ret = timeout(Duration::from_secs(2), job()).await;
    println!("{}", ret.unwrap());
}*/


use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use tokio::task;

#[tokio::main]
async fn main() {
    let server = TcpListener::bind("0.0.0:8080").await.unwrap();
    loop {
        let (mut stream, _) = server.accept().await.unwrap();

        task::spawn(async move {
            let mut rsp = String::from("http/1.1 200 Ok\r\n\r\n rust server");
            let ret = task::spawn_blocking(|| {
                " bloking"
            }).await;
            if let Ok(s) = ret{
                rsp.push_str(s);
            }

            stream.write_all(rsp.as_bytes()).await.unwrap();
        });
    }
}