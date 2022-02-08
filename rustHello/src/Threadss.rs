use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};
use std::rc::Rc;

/*

 Send / Sync
定义是：
 pub unsafe auto trait Send {}
 pub unsafe auto trait Sync {}

这两个 trait 都是 unsafe auto trait，
 auto 意味着编译器会在合适的场合，自动为数据结构添加它们的实现，
 而 unsafe 代表实现的这个 trait 可能会违背 Rust 的内存安全准则，如果开发者手工实现这两个 trait ，要自己为它们的安全性负责。

Send/Sync 是 Rust 并发安全的基础：
- 如果一个类型 T 实现了 Send trait，意味着 T 可以安全地从一个线程移动到另一个线程，
  也就是说所有权可以在线程间移动。
- 如果一个类型 T 实现了 Sync trait，则意味着 &T 可以安全地在多个线程中共享。
- 一个类型 T 满足 Sync trait，当且仅当 &T 满足 Send trait。
  对于 Send/Sync 在线程安全中的作用，
  可以这么看:
    > 如果一个类型 T: Send，那么 T 在某个线程中的独占访问是线程安全的；
    > 如果一个类型 T: Sync，那么 T 在线程间的只读共享是安全的。

  对于我们自己定义的数据结构，如果其内部的所有域都实现了 Send / Sync，
  那么这个数据结构会被自动添加 Send / Sync 。

  基本上原生数据结构都支持 Send / Sync，也就是说，绝大多数自定义的数据结构都是满足 Send / Sync 的。

  标准库中，不支持 Send / Sync 的数据结构主要有：
    * 裸指针 *const T / *mut T。它们是不安全的，所以既不是 Send 也不是 Sync。
    * UnsafeCell 不支持 Sync。也就是说，任何使用了 Cell 或者 RefCell 的数据结构不支持 Sync。
    * 引用计数 Rc 不支持 Send 也不支持 Sync。所以 Rc 无法跨线程。


*/


// Rc 既不是 Send，也不是 Sync
fn rc_is_not_send_and_sync()
{
    let a = Rc::new(1);
    let b = a.clone();
    let c = a.clone();
    /*thread::spawn(move || {
        println!("c= {:?}", c);
    });*/  // 这里会报错
}

//refcell 是 send的，在某个线程中的独占访问是线程安全的
fn refcell_is_send_but_not_sync()
{
    let a = RefCell::new(1);
    thread::spawn(move || {
        println!("c= {:?}", a);
    });
}

// refcell 不是 sync的。不能在多个线程间只读共享访问。
// 这里用arc包一下，是因为 rc 不能跨线程，改为用arc跨线程
fn arc_is_send_refCell_is_not_sync(){
    // RefCell 现在有多个 Arc 持有它，虽然 Arc 是 Send/Sync，但 RefCell 不是 Sync
    let a = Arc::new(RefCell::new(1));
    let b = Arc::clone(&a);
    let c =Arc::clone(&a);
    thread::spawn(move || {
        //println!("c:{:?}",c); // 这句报错，RefCell 无法安全的被共享
    });
    println!("c:{:?}",c);
}


// Arc 可以多线程共享且修改数据
fn arc_mutex_are_send_and_sync(){

    let a =Arc::new(Mutex::new(1));
    let b = Arc::clone(&a);
    let c =Arc::clone(&a);

    // 要完成跨线程，还得先clone，在move，在编译器眼里，是move ownership的。
    thread::spawn(move ||{
        let mut aa = a.lock().unwrap();
        *aa+=1;
    });

    {
        let mut bb = b.lock().unwrap();
        *bb += 4;
    } // 加个代码块，让 lock自动释放

    thread::sleep(Duration::from_secs(1));

    println!("a:{:?}",b.lock().unwrap());
}




pub fn run () {
    let v =(1..10).collect::<Vec<i32>>();

    //spawn的约束
    // 'static 意思是闭包捕获的自由变量必须是一个拥有所有权的类型，或者是一个拥有静态生命周期的引用；
    //  Send 意思是，这些被捕获自由变量的所有权可以从一个线程移动到另一个线程。
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


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_refcell_is_send_can_move_to_thread() {
        refcell_is_send_but_not_sync();
        thread::sleep(Duration::from_secs(1)); // sleep 1s 等待线程打印内容
    }

    #[test]
    fn test_refcell_is_not_sync_can_not_share_in_other_thread() {
        arc_is_send_refCell_is_not_sync();
        thread::sleep(Duration::from_secs(1)); // sleep 1s 等待线程打印内容
    }


    #[test]
    fn test_arc_mutex_are_send_and_sync() {
        arc_mutex_are_send_and_sync();
        thread::sleep(Duration::from_secs(1)); // sleep 1s 等待线程打印内容
    }

}



