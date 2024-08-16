use std::thread;
use std::time::Duration;

fn main() {
    let threads: Vec<_> = (0..10)
        .map(|i| {
            thread::Builder::new()
                .name("My test thread {i}".to_string())
                .spawn(move || {
                    println!("Thread #{i} started!");
                    thread::sleep(Duration::from_secs(2));
                    println!("Thread #{i} finished!");
                })
        })
        .collect();

    for handle in threads {
        handle.unwrap().join().unwrap();
    }
}
