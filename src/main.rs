#![feature(test)]

extern crate test;
extern crate rand;

use rand::Rng;
use std::thread;
use std::thread::JoinHandle;

fn main() {
    estimate_pi();
}

pub fn estimate_pi() {
    let total = 100_000;

    let in_circle_main = get_in_circle_count(total / 2);

    let in_circle_thread1 = compute_on_thread(total / 2)
        .join()
        .expect("In circle point count sample space computation on a parallel thread failed!");

    println!("Value of pi - {}", 4. * ((in_circle_main + in_circle_thread1) as f64) / (total as f64));
}

fn compute_on_thread(loop_count: i32) -> JoinHandle<i32> {
    return thread::spawn(move || {
        get_in_circle_count(loop_count)
    });
}

fn get_in_circle_count(loop_count: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let mut in_circle = 0;

    for _ in 0..(loop_count) {
        let a = rng.gen_range(-1f64, 1f64);
        let b = rng.gen_range(-1f64, 1f64);
        if (a * a) + (b * b) <= 1. {
            in_circle += 1;
        }
    }

    in_circle
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_estimate_pi(b: &mut Bencher) {
        b.iter(|| estimate_pi());
    }
}
