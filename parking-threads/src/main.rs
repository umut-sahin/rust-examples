use std::{
    thread,
    time::Duration,
};

fn main() {
    println!("Main thread is starting...");

    let handle = thread::spawn(move || {
        println!("New thread is starting...");

        println!("New thread is parking...");
        thread::park();
        println!("New thread is unparked...");

        println!("New thread is ending...");
    });

    println!("Main thread is going to sleep for 1 seconds...");
    thread::sleep(Duration::from_secs(1));
    println!("Main thread is awaken...");

    println!("Main thread is unparking the new thread...");
    handle.thread().unpark();

    println!("Main thread is waiting for the new thread...");
    handle.join().unwrap();

    println!("Main thread is ending...");
}
