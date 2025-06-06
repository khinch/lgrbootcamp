use rand::Rng;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn spawn_one_thread() {
    let handle = thread::spawn(|| {
        println!("Spawned thread");
    });

    println!("Main thread");
    handle.join().unwrap();
}

pub fn spawn_multiple_threads(thread_count: u32, max_delay: u64) {
    let mut threads = Vec::with_capacity(thread_count as usize);
    for i in 0..thread_count {
        let delay = rand::rng().random_range(0..max_delay);
        threads.push(print_message_in_thread(delay, i));
    }
    println!("Main thread");
    for thread in threads {
        thread.join().unwrap();
    }
}

fn print_message_in_thread(delay: u64, thread: u32) -> JoinHandle<()> {
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(delay));
        println!("Hello World, from thread {} ({}ms)", thread, delay);
    })
}

pub fn message_passing_with_mpsc(thread_count: u32, max_delay: u64) {
    let (tx, rx) = mpsc::channel();

    for thread in 0..thread_count {
        let delay = rand::rng().random_range(0..max_delay);
        let tx_clone = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(delay));
            tx_clone
                .send(format!("Hello World, from thread {} ({}ms)", thread, delay))
                .unwrap();
        });
    }

    drop(tx);

    for message in rx {
        println!("{}", message);
    }
}

#[derive(Debug)]
struct Database {
    connections: Vec<u32>,
}

impl Database {
    fn new() -> Self {
        Database {
            connections: vec![],
        }
    }
    fn connect(&mut self, id: u32) {
        self.connections.push(id);
    }
}

pub fn state_sharing_with_mutex() {
    let db = Arc::new(Mutex::new(Database::new()));
    let mut handles = Vec::new();

    for i in 0..10 {
        let db = Arc::clone(&db);
        let handle = thread::spawn(move || {
            let mut db_lock = db.lock().unwrap();
            db_lock.connect(i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let db_lock = db.lock().unwrap();
    println!("{:?}", db_lock);
}
