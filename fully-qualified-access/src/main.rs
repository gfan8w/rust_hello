fn main() {
    run_human_pilot();

    run_dog_animal();

}


fn run_human_pilot() {

    let person =Human;
    person.fly();           // 这里默认调用的是 Human实例的自身的 fly。

    Pilot::fly(&person); // 这样可以调到 Pilot 这个trait上实现的 fly
    Wizard::fly(&person);// 通过在方法前面指定Trait名字来限定rust知道去调哪个方法
    Human::fly(&person);// 这样等同于person.fly()


}

/// Human这个结构体实现了 fly，
/// Human实现了 Pilot 和 Wizard，他们各自都有 fly

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("THis is your captain speeking")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("up")
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously")
    }
}

// 上面的 trait 方法 都有个 self 参数，如果不同的对象实现了这些trait，还能通过 self的不同实例 知道具体是调哪个trait方法。
// trait的关联方法(associated function) 是不带 self的，它是 这个trait的一部分。下面演示相同的关联方法，如何区分调用

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}


impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn run_dog_animal(){
    println!("dog name:{}",Dog::baby_name()); // 这里只会访问 Dog 自身的函数，返回 spot。
    //Animal::baby_name();// Animal::baby_name是一个关联方法，而无法是一个Animal的实例方法。 实例相当于是 `_` 未知的
    let a = <Dog as Animal>::baby_name();  //将Dog 转换为 Animal 来访问Animal上的方法，必须用一个实例传进去
    println!("dog as an animal name:{}",a);
}





































