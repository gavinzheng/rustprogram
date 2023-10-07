use std::time::Duration;

fn main() {
    let n = std::sync::Mutex::new(0);
    std::thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                std::thread::sleep(Duration::from_secs(1)); // New!
            });
        }
    });
    println!("Final value: {}",n.into_inner().unwrap());
    //assert_eq!(n.into_inner().unwrap(), 1000);
}