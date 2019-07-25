use std::thread;

fn main() {
    let data_to_pass = 0;

    let handle = thread::spawn(move || {
        println!("Hello from the new thread!");
        data_to_pass + 42
    });

    println!("Hello from the main thread!");

    let threads_return = handle.join().unwrap();

    assert_eq!(threads_return, data_to_pass + 42);
}
