# Estimates the value of pi using rand and multithreading

[![Build Status](https://travis-ci.org/amitdash291/rust_pi_estimation.svg?branch=master)](https://travis-ci.org/amitdash291/rust_pi_estimation)

**How to make things parallel**:
- Spawn a thread and let it do `1:1 threads`
```
thread::spawn(|| {
    println!("in new thread");
    //do independent tasks
});
```
- thread::spawn will give us the thread with which we can wait for it to complete `handle.join()`
- `move` the ownership of the variable into the thread
```
let v = vec!(1, 2, 3);
thread::spawn(move || {
    println!("in new thread, {:?}", v);
    //do independent tasks
});
```

**Message Passing** *(Single owner)*:
- Using channels `let (tx, rx) = mpsc::channel()`
- Create multiple products using clone `mpsc::Sender::clone(&tx)`

**Shared memory** *(Multiple owners)*:
- Using `Mutex`. `let m = Mutex::new(5);` Locking can be acquried using `m.lock().unwrap();`
  - Example: Standup with duster/mike

**Trials**
* For 8_000_000 with 8 threads/process/routine : Rust: 1.472596486s, Elixir: 1.397912s, Go: 1.807347568s
* For 80_000_000 with 8 threads/process/routine : Rust: 16.956556741s, Elixir: 21.696663s, Go: 18.606728517s
* For 160_000_000 with 16 threads/process/routine : Rust: 34.451532214s, Elixir: 37.053626s, Go: 45.713308951s
