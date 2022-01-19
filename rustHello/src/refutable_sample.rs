




pub fn run(){

    println!("refutabe test");

    irrefutable_test();

    test();


}

fn irrefutable_test(){

    if let x =5 {      //  这里会有一个警告：irrefutable `if let` pattern， 这里  x=5 是确定的，无需 if，直接 let x =5 即可
        println!("irrefutable_test:{}",x)
    }


}

fn test(){
    let pets2 = vec!["Bob".to_string(),"Ferris".to_string(), "Frank".to_string()];
    let ferris="Ferris".to_string();
    for name in pets2.iter() {
        // match表达式是refutable的，不能用外面的变量，要用的话只能写在if后面
        match name {
            // &ferris => {                         // 这段代码有问题，全部匹配到了这个地方，IDE 还提示 ferris1144 未用到 奇怪 ？？？？？
            temp if(*temp)==ferris => {
                println!("hello** , I am rustacean.")
            }
            _ => {println!("hello- {}",name)}  //最后一个分支 arm 必须是 irrefutable的，不可辩驳的 就是覆盖所有情况的意思
        }
    }


    let pets =vec!["Bob","Ferris","Frank"];
    for name in pets.iter() {             // iter对每个元素是借用，原始集合不会改变，后续还可以使用原始集合。所以 name是 引用，"Ferris"前要加 &
        match name {
            &"Ferris" => {
                println!("There** is a rustacean among us! {}",name);
            }
            _ => {println!("hello {}",name)}
        }
    }

    //match的模式是耗尽（exhaustive）模式，最后又一个地方兜底，匹配住所有可能

    let dice_roll = 9;
    let match_val =7;
    match dice_roll {
        3 => println!("dice 3"),
        tmp if tmp == match_val => println!("dice 7"),
        other => println!("other:{}",other),              // catch-all pattern 匹配所有，必须有一个分支匹配所有，耗尽所有的匹配可能，如果不在意other的值，我们可以改为 _
        // _ => println!("other value")                        //如果不在意 other的值，无需获取，可以用 _ 代替 other
    }



}










