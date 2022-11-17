

pub fn run(){
    println!("test match patten");
    match_Literal_variable();
    match_named_variable();
    match_multiple_patten();
    match_extra_cond_match_guards();
    match_rang_at();

    match_person();

}

/*

我们可以使用match进行Pattern Matching：

match expr {
    pattern => expr,
    _ => (),
}
被Match的expr称为scrutinee
若它是Value Expression（即rvalue），则会创建一个临时变量，Pattern中的变量都绑定到该临时变量
若它是Place Expression（即lvalue），则Pattern中的变量都直接绑定到该lvalue
上述含有=>的每一项称为一个Arm
Arm返回一个Expression则要加逗号，返回一个Block则不需要加逗号
最后一个Arm可以加逗号（可选）
| pat1 | pat2 => {}: 可以由多个Pattern构成一个Arm，开头可以加一个|（可选）
用于绑定的变量，必须出现在一个Arm的所有Pattern中，并且以同样方式绑定（例如都按值绑定）
pattern if expr => expr: 可以添加一个if expr作为match guard
对于一个Arm有多个Pattern的情况，match guard可以执行多次，每次失败后就尝试匹配下一个Pattern
Pattern Matching是exhaustive的，必须列举所有可能性，因此通常使用_ => {}表示Default Arm

*/




fn match_Literal_variable(){
    //Matching Literals 匹配字面值
    let x =2;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ =>println!("anything") //不在意其他值，使用 _ 表示不绑定变量
    }
}

/// Named variables are irrefutable patterns ,命名变量匹配是一种不可辩驳的模式，即它一定是完全匹配住内容，不给后续其他分支任何机会。
/// 那后续分支如何有机会做匹配呢？ 我个人觉得是通过不同的类型来区分，一个很明显的例子是 enum。enum里的不同成员其实是不同类型的。
///  像本例的 Option::Some 与 Option::None 也是不同类型的，所以如果 x传入的是None，其他分支依然有机会匹配执行到。
/// 有一个麻烦的地方时 match块内会开启一个新的代码块，该代码块内部的变量会遮盖同名的外部变量，
/// 例如 match 内部的`y` 是一个新变量，跟外部的 y=5的那个变量是没有关系的。所以当x =Some(..)的时候， 总是能匹配到 Some(y)，如果 x =None，则无法匹配到第二个条件，匹配到的是最后一个
/// 最后一个匹配分支里 使用x，因为我们没有在match块里定义 x，所以x 还是外面的x。
/// refutability 的 反义词是 satisfiability （无可辩驳性 <=> 满足性 ），参考：http://xion.io/post/code/rust-patterns-ref.html
//一般来说 match的匹配臂是要求refutable的（即必须有可能匹配到false，必须留有余地给别人），除了最后一个分支是兜底，是可以为irrefutable无可辩驳的永远匹配到true，最后一个要兜底
fn match_named_variable(){
    let x =Some(10);
    let y =5;
    match x {
        Some(1) => println!("one"),
        Some(y) => println!("matched:, y ={:?}",y),
        _ =>println!("Default case, x={:?}",x) //不在意其他值，使用 _ 表示不绑定变量
    }
}

fn match_multiple_patten() {
    let x = Some(2);

    match x {
        Some(1) | Some(2) => println!("one or two"),
        Some(3) => println!("three"),
        _ => println!("anything"),
    }


    let x = 5;
    match x {
        1..=5 => println!("match 1..5"),
        6..=10 => println!("match 6..10"),
        _ => println!("anything")
    }


    let members =(1,2,3,4,5,6);
    match members {
        (first,.., last) => println!("first:{},last:{}",first,last),  // 这里不能有二义性。例如(..,second,..) => println!(..)  这样就会报错
                                                                                // 这种拆解结构也叫 destructuring
        _ => println!("match all")
    }


    struct Point {
        x:i32,
        y:i32,
        z:i32
    }

    let p =Point{x:0,y:2,z:9};

    match p {
        Point{x,..} => {println!("x is:{}, ignore y,z",x)}  // 只关心 x 变量，其他的都省略， // 这种拆解结构也叫 destructuring
        // _ => println!("not found a point")   //加这句话有个警告，会匹配所有
    }

}

///带有match guards的额外匹配
fn match_extra_cond_match_guards() {

    let num = Some(4);

    match num {
        Some(x) if x<5 =>{println!("less than five:{}",x)} //首先匹配到Some(x),然后再检查 if x< 5
        Some(x) => {println!("matched:{}",x)} // 这里会匹配到 Some，只要不是None 都会匹配到这个
        None => ()
    }

    let y =4;

    // 这里 内部没有 y，所以不会发生遮盖 外部 y的情况，新建变量n不会遮盖任何值
    match num {
        Some(50) => println!("match num is 50"),
        Some(n) if n==y => println!("Matched num is :{}",n), // match guard if n == y 不是模式匹配，不会产生新的变量，y是外部的y，不是新创建的y。
        Some(n) => println!("got some {}",n),
        Node => println!("get none")
    }


    let z=false;
    match y {
        2 |3 |4 | 5 if z => {println!("matched y")},
        //这里 2|3|4|5 是模式匹配， if 后面不是模式匹配，
        // 这里相当于 (2|3|4|5) if z =>... 而不是  2|3|4| (5 if z)
        _ => println!("no matched y")
    }


    let x = 5;
    let y=0;
    match x {
        y  => println!("alway match here, can't go to other arm：{}",y),
        // 这里会有一个编译告警，无法触达下面的分支，这里100%匹配，
        // 这里的y遮盖了外部的y，这句话是个模式匹配，
        // 捕获到的变量放到y里，而不是说要匹配 x=0

        _ => println!("anything")
    }


    let mut x = String::from("hel");
    x+="lol";
    let x_str =x.as_str(); //动态构造一个 hello
    let y="hellob";
    match x_str {
        "hello"  => println!("&y alway match here, can't go to other arm：{}",y), //字变量
        _ => println!("anything string")
    }


}


enum MatchMessage {
    Hello {id : i32}
}

///Match 的 @ at 符号绑定变量
fn match_rang_at() {

    let msg = MatchMessage::Hello { id: 90 };  // 把id 改为 4，20，90 看不同的结果

    //新建变量id_variable，在模式匹配中新建变量
    match msg {
        MatchMessage::Hello { id: id_variable @ 1..=10 } => { println!("match:{}", id_variable) },
        // 这里能访问匹配到的值, @ 可以保存匹配到的值到一个变量中

        MatchMessage::Hello { id: 11..=50 } => { println!("Found an id in another range") },
        // 这里无法拿到匹配到的值，因为id 可能是 10~50里的任意一个

        MatchMessage::Hello { id } => { println!("Found some other id: {}", id) }
        // 因为这里没有指定范围，可以拿到id的值

        MatchMessage::Hello { id } => { println!("Found some other id: {}", id) }
    }
}

// 参考：https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#-bindings

#[derive(Debug)]
enum MatchPerson {
    Name {surname : String},
    Card {no: u32},

}

fn match_person() {
    let a_person = MatchPerson::Name{surname: "rust".into()};
    let ref a_person_ref = a_person; // 等同于 let a_person_ref = &a_person;
    match &a_person_ref {
        &MatchPerson::Name { ref surname } => println!("match person name:{}", surname),
        &MatchPerson::Card {no} =>println!("match person card id:{}",no),
    }
    println!("person:{:?}", a_person);
}


fn process_name_ref(name: &str) {
    println!("match person name:{}",name);
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_match_person(){
        match_person();

        let a ="helo".to_string().as_str().len();
        println!("a size:{}",a);

    }

}










