use std::collections::vec_deque::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use crate::vehicle::Vehicle;

#[derive(Debug)]
pub struct Gate {
    pub queue: Arc<Mutex<VecDeque<Vehicle>>>,
    pub cond: Arc<Condvar>,
}

impl Gate {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::with_capacity(10))),
            cond: Arc::new(Condvar::new()),
        }
    }

    pub fn empty(&self) -> bool {
        self.queue.lock().unwrap().is_empty()
    }

    pub fn launch_thread(&self) {
        let queue = self.queue.clone();
        let cond = self.cond.clone();
        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            loop {
                let mut lock = queue.lock().unwrap();
                while lock.is_empty() {
                    lock = cond.wait(lock).unwrap();
                }
                let vehicle = lock.pop_front().unwrap();
                drop(lock);
                thread::sleep(vehicle.payment_duration(&mut rng) / 10);
            }
        });
    }

    pub fn nb_cars(&self) -> usize {
        self.queue.lock().unwrap().len()
    }
}
