use std::hint::black_box;
use std::sync::atomic;
use std::sync::{Arc, Mutex, RwLock, mpsc};
use std::thread;
use std::time::{Duration, Instant};

#[unsafe(no_mangle)]
pub fn bench_result(num_threads: u16, sec: u32) -> f32 {
    let time = Duration::from_millis(sec as u64);
    let value = Arc::new(Mutex::new(0));
    println!("running: threads = {num_threads}, seconds: {time:?}");
    let (tx, rx) = mpsc::channel();
    let mut res = 0;
    let end = Instant::now() + time;
    let end_flag = Arc::new(atomic::AtomicBool::new(false));

    for _ in 0..num_threads {
        let txx = tx.clone();
        let val = value.clone();
        let ef = end_flag.clone();
        let _ = thread::spawn(move || {
            let mut ops = 0;
            while !ef.load(atomic::Ordering::Relaxed) {
                let guard = val.lock().unwrap();
                black_box(*guard);
                ops += 1;
            }
            txx.send(ops).unwrap();
        });
    }
    while Instant::now() < end {}
    end_flag.store(true, atomic::Ordering::Relaxed);

    for _ in 0..num_threads {
        let ops = rx.recv().unwrap();
        res += ops;
    }
    res as f32 / sec as f32 / 1000.0
}
