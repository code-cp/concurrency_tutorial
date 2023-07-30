use std::sync::{Arc, Condvar, Mutex};
use std::thread; 
use std::time::Duration; 

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new())); 
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        // ref https://stackoverflow.com/questions/62651479/understanding-to-access-a-rust-arc
        let (lock, cvar) = &*pair2; 
        let mut started = lock.lock().unwrap(); 
        *started = true; 
        eprintln!("more work?");
        thread::sleep(Duration::from_secs(1));
        // notify main thread 
        cvar.notify_one(); 
    });

    let (lock, cvar) = &*pair; 
    let mut started = lock.lock().unwrap(); 
    while !*started {
        started = cvar.wait(started).unwrap(); 
    }
    eprintln!("work work"); 
}
