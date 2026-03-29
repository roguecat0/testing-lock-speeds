use std::env::Args;
use std::hint::black_box;
use std::sync::{Arc, Mutex, RwLock, mpsc};
use std::thread;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let num_threads = 10;
    let sec = 5;
    let time = Duration::from_secs(sec);
    let value = Arc::new(Mutex::new(0));
    let (tx, rx) = mpsc::channel();
    let mut res = 0;
    let end = Instant::now() + time;

    for _ in 0..num_threads {
        let txx = tx.clone();
        let val = value.clone();
        let _ = thread::spawn(move || {
            let mut ops = 0;
            while Instant::now() < end {
                for _ in 0..100 {
                    let guard = val.lock().unwrap();
                    black_box(*guard);
                    ops += 1;
                }
            }
            txx.send(ops).unwrap();
        });
    }
    for i in 0..num_threads {
        let ops = rx.recv().unwrap();
        println!("thread: {i}, cnt: {ops}");
        res += ops;
    }

    println!("result: {}, r/s: {:2}", res, res as f32 / sec as f32);

    Ok(())
}
