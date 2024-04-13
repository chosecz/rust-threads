// src/b.rs
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn read_data_in_loop(data: Arc<Mutex<super::Data>>) {
    loop {
        {
            let data = data.lock().unwrap();
            println!("Module B: Current value is {}, enabled: {}", data.value, data.enabled);
        } // Mutex is released here after this block

        thread::sleep(Duration::from_secs(1));
    }
}
