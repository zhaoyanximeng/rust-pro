use std::fmt::format;
use std::rc::Rc;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;

/*fn printi() {
    for i in 0..5 {
        thread::sleep(Duration::from_millis(500));
        println!("{}", i);
    }
}

fn main() {
    let (tx , rx):(mpsc::SyncSender<i32>, mpsc::Receiver<i32>)= mpsc::sync_channel(2);

    let t = thread::spawn(move || {
        for i in 0..5 {
            thread::sleep(Duration::from_millis(500));
            tx.send(i).unwrap();
        }
    });
    thread::sleep(Duration::from_secs(3));
    for ret in rx {
        println!("{}", ret)
    }
}*/

/*// 使用channel和Arc在不同线程内传递数据
#[derive(Debug)]
struct User {
    id: i32,
    name:String
}

fn new_user()->User{
    return User{id:1, name:String::from("zyxm")}
}

fn main() {
    let (tx, rx) : (mpsc::Sender<Arc<User>>, mpsc::Receiver<Arc<User>>) = mpsc::channel();
    let user = Arc::new(new_user()) ;
    let u = user.clone();
    thread::spawn(move || {
        tx.send(u).unwrap();
    });
    println!("{:?}", user);

    println!("{:?}", rx.recv().unwrap());
}*/

/*fn curl(i: i32, tx:mpsc::Sender<String>) {
    thread::sleep(Duration::from_secs(1));
    tx.send(format!("第{}个网页抓取完成", i)).unwrap();
}

fn main() {
    // let t1 = thread::spawn(|| curl(1));
    // let t2 = thread::spawn(|| curl(2));
    // t1.join().unwrap();
    // t2.join().unwrap();

    /*let mut thread_pool = Vec::new();
    for i in 0..5 {
        thread_pool.push(thread::spawn(move || curl(i)));
    }
    for t in thread_pool {
        t.join().unwrap();
    }*/

    let (tx, rx) = mpsc::channel();
    for i in 0..5 {
        let clone_tx = tx.clone();
        thread::spawn(move || {
            curl(i, clone_tx);
        });
    }
    thread::spawn(move || tx.send(String::from("开始抓取")).unwrap());
    for ret in &rx {
        println!("{}", ret);
    }
}*/

/*static mut N: i32 = 0;

fn main() {
    let mut n = 0;
    let mut pool = Vec::new();
    for _ in 0..15 {
        pool.push(thread::spawn(move || {
            unsafe {
                N = N + 1;
                println!("{}", N);
            }
        }))
    }
    for t in pool {
        t.join().unwrap();
    }
    unsafe { println!("{}", N); }
}*/

fn main() {
    let share_num = Arc::new(Mutex::new(0));
    let mut pool = Vec::new();
    for _ in 0..5 {
        let share_num_thread = share_num.clone();
        pool.push(thread::spawn(move || {
            let mut  num = share_num_thread.lock().unwrap();
            *num+=1;
        }))
    }
    for t in pool {
        t.join().unwrap();
    }
    println!("{:?}", share_num.lock().unwrap());
}