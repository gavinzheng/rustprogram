// sync_channels.rs

use std::thread; 
use std::sync::mpsc; 

fn main() { 
    let (tx, rx) = mpsc::sync_channel(1);
    let tx_clone = tx.clone();

    let _ = tx.send(0);

    thread::spawn(move || { 
        let _ = tx.send(100);
    }); 

    thread::spawn(move || {
        let _ = tx_clone.send(200);
    }); 

    println!("通过通道接收到 {} ", rx.recv().unwrap());
    println!("通过通道接收到 {} ", rx.recv().unwrap());
    println!("通过通道接收到 {} ", rx.recv().unwrap());
    println!("通过通道接收到 {:?} ", rx.recv());
}
