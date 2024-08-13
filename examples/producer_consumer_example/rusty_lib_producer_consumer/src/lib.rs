use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

// A thread-safe queue with a condition variable to signal when an item is available
pub struct SharedQueue {
    queue: Mutex<VecDeque<i32>>,
    condvar: Condvar,
}

impl SharedQueue {
    pub fn new() -> Self {
        SharedQueue {
            queue: Mutex::new(VecDeque::new()),
            condvar: Condvar::new(),
        }
    }

    pub fn produce(&self, value: i32) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(value);
        println!("Rust produced: TASK-{}", value);
        self.condvar.notify_one(); // data-ready for C++
    }

    pub fn consume(&self) -> i32 {
        // Safely dequeue an item

        let mut queue = self.queue.lock().unwrap();
        while queue.is_empty() {
            queue = self.condvar.wait(queue).unwrap();
        }
        let value = queue.pop_front().unwrap();
        value
    }
}

#[no_mangle]
pub extern "C" fn make_shared_queue() -> *mut SharedQueue {
    // Expose a heap allocated SharedQueue as a raw ptr
    Box::into_raw(Box::new(SharedQueue::new()))
}

#[no_mangle]
pub extern "C" fn produce_task(queue: *mut SharedQueue, value: i32) {
    // Unsafely take the queue handle from the C++ side, and queue an item
    let queue = unsafe { &*queue };
    queue.produce(value);
}

#[no_mangle]
pub extern "C" fn consume_task(queue: *mut SharedQueue) -> i32 {
    // Unsafely take the queue handle from the C++ side, and dequeue an item
    let queue = unsafe { &*queue };
    queue.consume()
}

#[no_mangle]
pub extern "C" fn start_producer(queue: *mut SharedQueue) {
    let queue = unsafe { Arc::from_raw(queue) };
    let nb_iterations = 10;

    thread::spawn(move || {
        for i in 1..=nb_iterations {
            queue.produce(i);
            thread::sleep(Duration::from_millis(250)); // Introduce some delay required to produce tasks
        }
    });
}
