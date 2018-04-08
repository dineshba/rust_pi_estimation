#![feature(test)]

extern crate test;
extern crate rand;

use rand::Rng;
use std::thread;
use std::thread::JoinHandle;

fn main() {
    estimate_pi(8_000_000, 8);
}

pub fn estimate_pi(total_points: i32, total_threads: i32) {
    let handles = compute_on_multiple_threads(total_points, total_threads);

    let mut total_in_circle_points = 0;

    for handle in handles {
        total_in_circle_points += handle.join().expect("Thread computation failed!");
    }

    print_value_of_pi(total_points, total_in_circle_points);
}

fn print_value_of_pi(total_points: i32, total_in_circle_points: i32) {
    println!("Value of pi: {}", 4. * ((total_in_circle_points) as f64) / (total_points as f64));
}

fn compute_on_multiple_threads(total_count: i32, num_of_threads: i32) -> Vec<JoinHandle<i32>> {
    (0..num_of_threads).map(|_| compute_on_single_thread(total_count / num_of_threads)).collect()
}

fn compute_on_single_thread(loop_count: i32) -> JoinHandle<i32> {
    return thread::spawn(move || {
        get_in_circle_count(loop_count)
    });
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
