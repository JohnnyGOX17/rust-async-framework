extern crate core_affinity;
use std::thread;

fn main() {
    // Retrieve the IDs of all active CPU cores
    let core_ids = core_affinity::get_core_ids().unwrap();

    let handles = core_ids
        .into_iter()
        .map(|id| {
            thread::spawn(move || {
                // NOTE: doesn't work on apple silicon https://github.com/Elzair/core_affinity_rs/issues/22
                if core_affinity::set_for_current(id) {
                    println!("Hello from thread {id:?}");
                } else {
                    println!("ruh roh");
                }
            })
        })
        .collect::<Vec<_>>();

    for handle in handles.into_iter() {
        handle.join().unwrap();
    }

    // Can show thread names in htop by F2 → Display options → Show custom thread names
}
