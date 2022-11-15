use redlock::RedLock;

fn main(){
    lock();
    std::thread::sleep(std::time::Duration::from_secs(1));
}


fn lock() {
    let rl = RedLock::new(vec!["redis://127.0.0.1:6379/"]);

    let lock;
    loop {
        println!("try lock...");
        match rl.lock("mutex1".as_bytes(), 1000) {
            Some(l) => { lock = l; break }
            None => ()
        }
    }

    // Critical section


    rl.unlock(&lock);
}

fn try_lock() {
    let rl = RedLock::new(vec!["redis://127.0.0.1:6379/"]);

    let lock_guard= rl.acquire("mutex1".as_bytes(), 1000);
    loop {
        println!("try lock...");
        match lock_guard.lock.lock("mutex1".as_bytes(), 1000) {
            Some(l) => { lock = l; break }
            None => ()
        }
    }

    // Critical section


    rl.unlock(&lock);
}

