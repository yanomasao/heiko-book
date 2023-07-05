use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn some_func(lock: Arc<Mutex<u64>>, name: &str) {
    loop {
        let mut val = lock.lock().unwrap();
        if 1000 <= *val {
            return;
        }
        *val += 1;
        let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化します
        let sec = rng.gen_range(0..100);
        sleep(Duration::from_millis(sec));
        println!("{} {} {}", name, *val, sec);
    }
}

#[test]
fn main_test() {
    let lock0 = Arc::new(Mutex::new(0));
    let lock1 = lock0.clone();

    let th0 = thread::spawn(move || {
        some_func(lock0, "th0    ");
    });

    let th1 = thread::spawn(move || {
        some_func(lock1, "    th1");
    });

    th0.join().unwrap();
    th1.join().unwrap();
}
