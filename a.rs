// src/a.rs
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn modify_data_in_loop(data: Arc<Mutex<super::Data>>) {
    loop {
        {
            let mut data = data.lock().unwrap();
            data.value += 1; // Increment the value
        } // Mutex is released here after this block

        thread::sleep(Duration::from_secs(1));
    }
}
