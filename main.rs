use std::sync::{Arc, Mutex};
use std::thread;

mod a;
mod b;

pub struct Data {
    value: i32,
    enabled: bool,
}

fn main() {
    let data = Arc::new(Mutex::new(Data { value: 10, enabled: true }));

    let data_for_a = Arc::clone(&data);
    let data_for_b = Arc::clone(&data);

    let thread_a = thread::spawn(move || {
        a::modify_data_in_loop(data_for_a);
    });

    let thread_b = thread::spawn(move || {
        b::read_data_in_loop(data_for_b);
    });

    thread_a.join().unwrap();
    thread_b.join().unwrap();
}
