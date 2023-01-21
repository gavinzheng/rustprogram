use std::thread;
use std::sync::mpsc::channel;

fn main() {
  let (tx, rx) = channel();
  let tx2 = tx.clone();
  thread::spawn(move || tx.send(100));
  thread::spawn(move || tx2.send(600));

  println!("{:?}",rx.recv().unwrap());
  println!("{:?}",rx.recv().unwrap());
}