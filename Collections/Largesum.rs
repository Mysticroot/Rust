use std::sync::mpsc;
use std::thread;

fn main() {
    // Sum 1..=10^8 using multiple threads and message passing
    let n: u64 = 100_000_000;
    let threads: u64 = 4; // adjust based on your CPU
    let chunk = n / threads;

    let (tx, rx) = mpsc::channel::<u128>();
    let mut handles = Vec::new();

    for i in 0..threads {
        let start = i * chunk + 1; // inclusive
        let end = if i == threads - 1 { n } else { (i + 1) * chunk }; // inclusive
        let tx_i = tx.clone();

        let handle = thread::spawn(move || {
            let mut sum: u128 = 0;
            for j in start..=end {
                sum += j as u128;
            }
            tx_i.send(sum).unwrap();
        });
        handles.push(handle);
    }

    // Drop the original sender so the receiver can finish when all clones are dropped
    drop(tx);

    let mut final_sum: u128 = 0;
    for _ in 0..threads {
        final_sum += rx.recv().unwrap();
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Final Sum 1..=10^8: {}", final_sum);
}
