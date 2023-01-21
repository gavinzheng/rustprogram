use std::sync::atomic;
use atomic::Ordering::SeqCst;
use std::thread;

fn main() {
  let number =atomic::AtomicUsize::new(100);
  let prev = number.fetch_add(1,SeqCst);
  assert_eq!(prev,100);

  let prev = number.swap(200, SeqCst);
  assert_eq!(prev,101);
  assert_eq!(number.load(SeqCst),200);
}