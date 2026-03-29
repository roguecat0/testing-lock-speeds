use std::collections::HashMap;
use std::hint::black_box;
use std::sync::atomic;
use std::sync::{Arc, Mutex, RwLock, mpsc};
use std::thread;
use std::time::{Duration, Instant};
use std::{env, u64};

type BenchRes = HashMap<String, HashMap<i32, f64>>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    let sec = str::parse(&args[1])?;
    let num_threads = str::parse(&args[2])?;
    if args.len() > 3 {
        println!("{args:?}");
        let rps = bench_result(num_threads, sec);
        println!(
            "result: streads: {num_threads}, seconds: {sec}, million reads per seconds {:.2}",
            rps / 1000000.0
        );
        return Ok(());
    }
    let type1 = "mutex";
    let mut mutex_res: HashMap<i32, f64> = HashMap::new();
    for i in 1..(num_threads + 1) {
        let rps = bench_result(i, sec);
        mutex_res.entry(i).or_insert(rps);
    }
    let res: BenchRes = HashMap::from([(type1.to_string(), mutex_res)]);
    print_results_ai(&res, sec);

    Ok(())
}
fn bench_result(num_threads: i32, sec: u64) -> f64 {
    let time = Duration::from_secs(sec);
    let value = Arc::new(Mutex::new(0));
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
    res as f64 / sec as f64 / 1000000.0
}

fn print_results(res: &BenchRes, sec: u64) {
    println!("milling reads per seconds measured over {sec} seconds");
    print!("type / mrps | ");
    for i in res.values().next().unwrap().keys() {
        print!(" {i} |")
    }
    println!();
    for (k, v) in res.iter() {
        print!("{k} | ");
        for (_, v2) in v.iter() {
            print!(" {v2} |")
        }
        println!();
    }
}

fn print_results_ai(res: &BenchRes, sec: u64) {
    println!("million reads per second measured over {sec} seconds");

    if res.is_empty() {
        println!("(no results)");
        return;
    }

    let mut thread_counts: Vec<_> = res
        .values()
        .flat_map(|results| results.keys().copied())
        .collect();
    thread_counts.sort_unstable();
    thread_counts.dedup();

    let first_col_header = "type / mrps";
    let first_col_width = res
        .keys()
        .map(|name| name.len())
        .chain(std::iter::once(first_col_header.len()))
        .max()
        .unwrap_or(first_col_header.len());

    let mut col_widths = Vec::with_capacity(thread_counts.len());
    for thread_count in &thread_counts {
        let header = thread_count.to_string();
        let value_width = res
            .values()
            .filter_map(|results| results.get(thread_count))
            .map(|value| format!("{value:.2}").len())
            .max()
            .unwrap_or(0);
        col_widths.push(header.len().max(value_width));
    }

    let print_separator = || {
        print!("+-{}-+", "-".repeat(first_col_width));
        for width in &col_widths {
            print!("-{}-+", "-".repeat(*width));
        }
        println!();
    };

    print_separator();
    print!("| {:<first_col_width$} |", first_col_header);
    for (thread_count, width) in thread_counts.iter().zip(&col_widths) {
        print!(" {:>width$} |", thread_count, width = *width);
    }
    println!();
    print_separator();

    let mut rows: Vec<_> = res.iter().collect();
    rows.sort_by(|(left, _), (right, _)| left.cmp(right));

    for (lock_type, values) in rows {
        print!("| {:<first_col_width$} |", lock_type);
        for (thread_count, width) in thread_counts.iter().zip(&col_widths) {
            match values.get(thread_count) {
                Some(value) => print!(" {:>width$.2} |", value, width = *width),
                None => print!(" {:>width$} |", "-", width = *width),
            }
        }
        println!();
    }

    print_separator();
}
