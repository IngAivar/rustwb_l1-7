use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(_) => break,
                Err(_) => panic!("Error receiving from channel"),
            }
        }
    });

    // Симулируем работу потока
    thread::sleep(std::time::Duration::from_secs(3));

    // Отправляем сигнал о завершении
    tx.send(()).unwrap();

    handle.join().unwrap();
}