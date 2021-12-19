use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};
use std::rc::Rc;

pub fn run () {
    let v =(1..10).collect::<Vec<i32>>();
    let t =thread::spawn(move|| {
        for i in v {                    // 这里的v等效于v.into_iter()，所有权自动进入for循环
            println!("{} in the spawned thread",i);

        }
    });


    //drop(v);


    t.join().unwrap();


    let v1 = [1,2,3,4];
    let t1 =thread::spawn(move|| {
        //println!("here is the vec:{:?}",v1);
        for i in v1 {
            println!("{} in the spawned thread",i);

        }
    });



    t1.join();

    //shared-memory 模式 定义一个锁 来增加一个共享变量的值
    let counter =Arc::new(Mutex::new(5));
    let mut handles =vec![];
    for _ in 1..=5 {
        let counter=Arc::clone(&counter);
        let t=thread::spawn(move||{
            let mut v=counter.lock().unwrap();
            let mut vv =*v;
            *v+=1;
        });
        handles.push(t)
    }

    for t in handles{
        t.join();
    }

    println!("counter after:{}",counter.lock().unwrap())








}






