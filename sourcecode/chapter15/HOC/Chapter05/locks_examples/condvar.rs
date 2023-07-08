use std::thread;
use std::sync::{Arc, Condvar, Mutex};

fn main() {
    let total_readers = 3;
    let mutcondvar: Arc<(Mutex<(bool, u16)>, Condvar)> =
        Arc::new((Mutex::new((false, 0)), Condvar::new()));

    let mut readers = Vec::with_capacity(total_readers);
    for _ in 0..total_readers {
        let mutcondloop = Arc::clone(&mutcondvar);
        readers.push(thread::spawn(move || {
            let mut total_zeros = 0;
            let mut total_wakes = 0;
            let &(ref mtx, ref cnd) = &*mutcondloop;

            while total_zeros < 10 {
                let mut guard = mtx.lock().unwrap();
                while !guard.0 {
                    guard = cnd.wait(guard).unwrap();
                }
                
                total_wakes += 1;
                if guard.1 == 0 {
                    total_zeros += 1;
                }
                guard.0 = false;
            }
            // println!("total_zeros: {:?} total_wakes: {:?}", total_zeros, total_wakes);
            total_wakes
        }));
    }

    let _ = thread::spawn(move || {
        let &(ref mtx, ref cnd) = &*mutcondvar;
        loop {
            let mut guard = mtx.lock().unwrap();
            guard.1 = guard.1.wrapping_add(1);
            guard.0 = true;
            // println!("notify_all guard: {:?} ", guard);
            cnd.notify_all();
        }
    });

    for jhs in readers {
        println!("{:?}", jhs.join().unwrap());
    }
}
