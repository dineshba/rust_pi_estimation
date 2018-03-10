#![feature(test)]

extern crate test;
extern crate rand;

// use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use std::thread;
use std::thread::JoinHandle;

fn main() {
    estimate_pi();
}

pub fn estimate_pi() {
    let mut rng = rand::thread_rng();
    // let random_number = rng.gen_range(1,101);
    let between = Range::new(-1f64, 1f64);
    
    let total = 100_000;
    let mut in_circle_main = 0;

    let computation1 = compute_on_thread(total / 2);

    for _ in 0..(total/2) {
        let a = between.ind_sample(&mut rng);
        let b = between.ind_sample(&mut rng);
        if a*a + b*b <= 1. {
            in_circle_main += 1;
        }
    }

    let in_circle_thread1 = computation1.join().unwrap();

    println!("Value of pi - {}", 4. * ((in_circle_main + in_circle_thread1) as f64) / (total as f64));
    // println!("Generated random number - {}", random_number);
}

fn compute_on_thread(loop_count: i32) -> JoinHandle<i32> {
    return thread::spawn(move|| {
        let mut internal_in_circle = 0;
        let mut rng = rand::thread_rng();
        let between = Range::new(-1f64, 1f64);

        for _ in 0..loop_count {
            let a = between.ind_sample(&mut rng);
            let b = between.ind_sample(&mut rng);
            if a*a + b*b <= 1. {
                internal_in_circle += 1;
            }
        }
        return internal_in_circle;
    });
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
