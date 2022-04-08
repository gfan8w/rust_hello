
struct Hello{
    name: String,
}

/// 函数参数 是self，还是 &self的区别：
/// 1）如果参数是self，说明该函数会消耗self对象，self在该函数调用后，就移动不存在了
/// 2) 如果是 &self，则不会发生
impl Hello {

    fn world(self,world: String) {
        println!("{},{}",self.name,world);
    }
    fn say(&self,world: String) {
        println!("{},{}",self.name,world);
    }
}

pub fn world_only_once() {
    let hello =Hello{name:"hello".to_string()};
    hello.world("java".to_string());
    // hello.world("rust".to_string());   // 无法再次调用

}

pub fn say_many_times() {
    let hello =Hello{name:"hello".to_string()};
    hello.say("world".to_string());
    hello.say("rust".to_string());  // say的self是引用，可以调用很多次
}

pub fn main() {
    say_many_times();
    world_only_once();

    let mut ave = vec!["hel".to_string(), "ok".to_string(), "sub".to_string()];
    ave.retain(|v| v!="hel");
    println!("{:?}",ave);
}