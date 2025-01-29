use std::{
    sync::atomic::{AtomicUsize, Ordering},
    thread,
    time::Duration,
};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn print_and_count() {
    let count = COUNTER.fetch_add(1, Ordering::Relaxed);
    println!("{}", count);
}

fn ticker<F>(mut func: F)
where
    F: FnMut() + Send + 'static,
{
    loop {
        func();
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    ticker(print_and_count);
}
