#![feature(test)]

extern crate test;
extern crate rand;

use rand::Rng;
use std::thread;
use std::time::Instant;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Mutex, Arc};

fn main() {
    let now = Instant::now();
    estimate_pi(8_000_000, 8);
    let time_taken = now.elapsed();
    println!("Time taken in seconds: {}.{}", time_taken.as_secs(), time_taken.subsec_nanos());
}

pub fn estimate_pi(total_points: i32, total_threads: i32) {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let arc_tx = Arc::new(Mutex::new(tx));

    compute_on_multiple_threads(total_points, total_threads, arc_tx);

    let mut total_in_circle_points = 0;
    for i in rx {
        total_in_circle_points += i;
    };

    print_value_of_pi(total_points, total_in_circle_points);
}

fn print_value_of_pi(total_points: i32, total_in_circle_points: i32) {
    println!("Value of pi: {}", 4. * ((total_in_circle_points) as f64) / (total_points as f64));
}

fn compute_on_multiple_threads(total_count: i32, num_of_threads: i32, arc_tx: Arc<Mutex<Sender<i32>>>) {
    for _i in 0..num_of_threads {
        let arc_tx = Arc::clone(&arc_tx);
        thread::spawn(move || {
            let in_circle_count = get_in_circle_count(total_count / num_of_threads);
            let tx = arc_tx.lock().unwrap();
            tx.send(in_circle_count).unwrap();
        });
    }
}

fn get_in_circle_count(loop_count: i32) -> i32 {
    let mut rng = rand::thread_rng();

    (0..loop_count)
        .map(|_| {
            let a = rng.gen_range(-1f64, 1f64);
            let b = rng.gen_range(-1f64, 1f64);
            (a * a) + (b * b)
        })
        .filter(|x| x <= &(1.0 as f64))
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_estimate_pi(b: &mut Bencher) {
        b.iter(|| estimate_pi(800_000, 8));
    }
}
