use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

pub fn run(){
    let (tx,rc) =channel();
    let tx1 =tx.clone();


    thread::spawn(move|| {
        let msg =vec![
            String::from("hi"),
            String::from("from"),
            String::from("rust"),
            String::from("thread"),
        ];
        for m in msg {
            let s=tx.send(m).unwrap();
            // println!("values:{} sent",m);
            thread::sleep(Duration::from_millis(3));
        }



    });

    thread::spawn(move|| {
        let msg =vec![
            String::from("thread"),
            String::from("rust"),
            String::from("is"),
            String::from("safe"),
        ];
        for m in msg {
            let s=tx1.send(m).unwrap();
            thread::sleep(Duration::from_millis(30));
        }



    });



    for received in rc {
        println!("received:{}",received)
    }
}





