use tokio::time::{Duration, Instant, sleep};
use std::thread;

async fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let tow_seconds = Duration::new(2, 0);
    let _ = sleep(tow_seconds).await;
    return  2;
}

#[tokio::main(worker_threads = 1)]
async fn main() {
    let now = Instant::now();
    let future_one = tokio::spawn(do_something(1));
    let future_two = tokio::spawn(do_something(2));
    let two_seconds = Duration::new(2, 0);
    thread::sleep(two_seconds);
    let _ = future_one.await.unwrap();
    let _ = future_two.await.unwrap();
    println!("time elapsed {:?}", now.elapsed());
}
