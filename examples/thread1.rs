use std::{sync::mpsc, thread, time::Duration};

use anyhow::{Result, anyhow};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        std::thread::spawn(move || producer(i, tx));
    }

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("Received message from producer {}: {}", msg.idx, msg.value);
        }
    });

    consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;

    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        // The diagnostic indicates "the trait bound `StandardUniform: rand::distr::Distribution<usize>` is not satisfied".
        // This means `rand::random::<usize>()` is not directly supported by the current `rand` configuration or version
        // for generating a `usize` value uniformly across its entire range.
        // A common workaround is to generate a `u64` (which `rand::random` typically supports) and then cast it to `usize`.
        // This will provide a random `usize` value, potentially truncated if `usize` is smaller than `u64` (e.g., on 32-bit systems).
        let value = rand::random::<u64>() as usize;
        tx.send(Msg::new(idx, value))?;
        thread::sleep(Duration::from_millis(1000));
    }
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Msg { idx, value }
    }
}
