use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::Duration;

fn some_func(lock: Arc<Mutex<u64>>, i: u32) {
    loop {
        let mut val = lock.lock().unwrap();
        if 1000 <= *val {
            return;
        }
        *val += 1;
        let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化します
        let sec = rng.gen_range(0..1000);
        sleep(Duration::from_millis(sec));
        println!(
            "{}{}{} {:>5} {:>4}",
            (vec![" "; i as usize]).join(""),
            i,
            (vec![" "; 10 - i as usize]).join(""),
            *val,
            sec
        );
    }
}

#[test]
fn main_test() {
    let lock = Arc::new(Mutex::new(0));

    let mut ths: Vec<JoinHandle<()>> = Vec::new();

    for i in 0..10 {
        let lock_tmp = lock.clone();
        ths.push(thread::spawn(move || {
            some_func(lock_tmp, i);
        }));
    }

    for e in ths {
        e.join().unwrap();
    }
}
