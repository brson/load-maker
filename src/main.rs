use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    let n = num_cpus::get();
    let mut handles = vec![];
    let atomic = Arc::new(AtomicBool::new(false));
    for _ in 0..n {
        let atomic = atomic.clone();
        handles.push(thread::spawn(move || {
            loop {
                let foo = atomic.load(Ordering::Relaxed);
                atomic.store(!foo, Ordering::Relaxed);
            }
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }
}
