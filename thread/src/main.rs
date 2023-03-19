use std::rc::Rc;
use std::sync::{mpsc, Arc};
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
}