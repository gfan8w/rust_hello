use std::fmt::{Debug, Formatter};

trait Playable : Debug {
    fn play(&self);
    fn pause(&self) {println!("pause");}
    fn get_duration(&self) -> f32;
}

// Audio类型，实现Trait Playable
struct Audio {name: String, duration: f32}

impl Debug for Audio {
    // 这里手动实现Debug，而不是直接使用 #[derive(Debug)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"<name:[{}], duration:[{}] >", self.name, self.duration)
    }
}

impl Playable for Audio {
    fn play(&self) {println!("listening audio: {}", self.name);}
    fn get_duration(&self) -> f32 {self.duration}
}

// Video类型，实现Trait Playable
#[derive(Debug)]
struct Video {name: String, duration: f32}
impl Playable for Video {
    fn play(&self) {println!("watching video: {}", self.name);}
    fn pause(&self) {println!("video paused");}
    fn get_duration(&self) -> f32 {self.duration}
}

// 将Audio的实例或Video的实例当作Playable的Trait Object来使用
fn run(){

    let x: &dyn Playable = & Audio{ name: "audio".to_string(), duration: 2.0 };

    let y: &dyn Playable = & Video{ name: "audio".to_string(), duration: 2.0 };

    x.play();
    x.pause();
    x.get_duration();

    y.play();
    y.pause();
    y.get_duration();

    let arr: [&dyn Playable;2] = [x, y];
    println!("{:#?}", arr)  //使用println!的调试输出格式{:#?}，要让Playable实现名为std::fmt::Debug的Trait
                            // 因为Playable自身也是一个Trait，所以使用Trait继承的方式来继承Debug。
                            // 继承Debug后，要求实现Playable Trait的类型也都要实现Debug Trait，
                            // 因此在Audio和Video之前使用#[derive(Debug)]来实现Debug Trait。
}

/*
当 Audio, Video 类型实现了 Trait Playable 后，就可以将类型Audio, Video当作Trait Playable 来使用。
这在概念上来说似乎是正确的，但根据Rust语言的特性，Rust没有直接实现这样的用法。原因之一是，Rust中不能直接将Trait当作数据类型来使用。

Rust真正支持的用法是：虽然Trait自身不能当作数据类型来使用，
但Trait Object可以当作数据类型来使用。因此，可以将实现了 Trait Playable 的类型 Audio, Video 当作Trait Playable 的Trait Object来使用。
也就是说，Trait Object是Rust支持的一种数据类型，它可以有自己的实例数据，就像Struct类型有自己的实例对象一样。

可以将Trait Object和Slice做对比，它们在不少方面有相似之处。

对于类型T，写法[T]表示类型T的Slice类型，由于Slice的大小不固定，因此几乎总是使用Slice的引用方式&[T]，
Slice的引用保存在栈中，包含两份数据：Slice所指向数据的起始指针和Slice的长度。

对于Trait Playable，写法 dyn Playable 表示Trait Playable 的Trait Object类型，
由于Trait Object的大小不固定，因此几乎总是使用Trait Object的引用方式 &dyn Playable，
Trait Object的引用保存在栈中，包含两份数据：Trait Object所指向数据的指针和指向一个虚表vtable的指针。

Trait Object大小不固定：这是因为，对于Trait Playable，类型Audio可以实现Trait Playable，类型Video也可以实现Trait Playable，因此Trait Object没有固定大小
几乎总是使用Trait Object的引用方式：
1.虽然Trait Object没有固定大小，但它的引用类型的大小是固定的，它由两个指针组成，因此占用两个指针大小，即两个机器字长
一个指针指向实现了Trait Playable 的具体类型的实例，也就是当作Trait Playable来用的类型的实例，比如Audio类型的实例、Video类型的实例等
另一个指针指向一个虚表vtable，vtable中保存了Video或Audio类型的实例对于可以调用的实现于Playable的方法。
当调用方法时，直接从vtable中找到方法并调用。之所以要使用一个vtable来保存各实例的方法，是因为实现了Trait Playable的类型有多种，
这些类型拥有的方法各不相同，当将这些类型的实例都当作Trait Playable来使用时(此时，它们全都看作是Trait Playable类型的实例)，有必要区分这些实例各自有哪些方法可调用
Trait Object的引用方式有多种。例如对于Trait Playable，其Trait Object类型的引用可以是 &dyn Playable、Box<dyn Playable>、Rc<dyn Playable>等



*/


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        run()
    }
}

