use std::collections::VecDeque;

fn main() {
    let queue = std::sync::Mutex::new(VecDeque::new());

    std::thread::scope(|s| {
        // Consuming thread
        let t = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();
            if let Some(item) = item {
                dbg!(item);
            } else {
                std::thread::park();
            }
        });

        // Producing thread
        for i in 0..10 {
            queue.lock().unwrap().push_back(i);
            t.thread().unpark();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}