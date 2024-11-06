use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

fn producer(tx: Sender<i32>) {
    for i in 1..10 {
        tx.send(i).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}

fn consumer(rx: Receiver<i32>) {
    loop {
        match rx.recv() { 
            Ok(item) => {
                println!("consumer {}", item);
                thread::sleep(Duration::from_secs(1));
            }
            Err(_) => {
                println!("consumer stop");
                break
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use super::*;

    #[test]
    fn it_works() {
        // 创建channel
        let (tx, rx) = mpsc::channel();
        
        // thread::spawn创建新线程
        // move闭包，把主线程变量的所有权转移到闭包中
        let producer_thread = thread::spawn(move || {
            producer(tx);
        });
        let consumer_thread = thread::spawn(move || {
            consumer(rx);
        });
        
        producer_thread.join().unwrap();
        consumer_thread.join().unwrap();
    }
}
