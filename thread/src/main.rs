use std::thread;
use std::time::Duration;

fn printi() {
    for i in 0..5 {
        thread::sleep(Duration::from_millis(500));
        println!("{}", i);
    }
}

fn main() {
    let t = thread::spawn(printi);
    t.join().unwrap();

    println!("{}", "zyxm")
}
