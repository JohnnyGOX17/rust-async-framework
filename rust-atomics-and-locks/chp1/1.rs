use std::thread;

fn main() {
    // Spawn threads with an explicit function
    {
        let t1 = thread::spawn(f);
        let t2 = thread::spawn(f);

        println!("Hello from the main thread.");

        // Wait till threads are finished before continuing (returns Result)
        t1.join().unwrap();
        t2.join().unwrap();
    }

    // Commonly, we can pass functionality to a thread as a closure rather than a function
    {
        let numbers = vec![1, 2, 3];

        thread::spawn(move || {
            for n in &numbers {
                println!("Closure thread: {n}");
            }
        })
        .join()
        .unwrap();
    }
}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
