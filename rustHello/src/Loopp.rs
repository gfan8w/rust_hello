

pub fn run() {

    println!("demo show loop:");

    let mut data =vec![1,2,3,4];
    for item in data.iter_mut() {  //data 这里已经borrow了，它是一个borrowed的可变引用
        //data.push(*item+1)         //这里会有问题，因为又一次使用了可变引用。不允许2次使用可变引用
    }

    /*
    iter()的签名是: iter(&self) 是引用
    into_iter()的签名是 into_iter(self)， 把自己移动进去了
    iter_mut() 的签名是 iter_mut(&mut self)， 把可变的引用移动进去
    */



    let mut a: Vec<i32> = vec![1,2,3,4,5];

    for i in &a {  // 这里的 &a 也可以用 a，如果用a 就会发生移动，后续 println!("{:?}", a); 只要再次使用a 就会失败
        // ^不可变的循环
        let i: &i32 =i;  // 这里是不可变引用
        print!("{} ",i)
    }

    println!("{:?}", &a);

    println!();

    for i in &mut a {
        // ^ 可变的循环遍历
        let i: &mut i32 = i; // 元素是可变的指针，指向集合元素
        *i +=3;
        print!("{} ",i);
    }
    println!("{:?}", &a);

    for i in a {
        print!("{} ", i); //值遍历
    }
    // 从这里以后，a 无法再适用，因为已经move 到for loop里
    // println!("{:?}", &a); 这里会报错，a已经被move了，这里无法borrow

    println!();

    let name = vec!["hello", "world", "rustacean"];
    for (pos,val) in name.iter().enumerate() { // enumerate返回(i,value)的元组
        print!("{}:{}, ",pos,val);
    }

    println!("{:?}",name);

    println!();

    // 循环遍历数组
    for i in 0..name.len() {
        print!("{}:{}, ", i, name[i])
    }
    println!();


    let pets =vec!["Bob","Ferris","Frank"];
    for name in pets.iter() {  // iter对每个元素是借用，原始集合不会改变，后续还可以使用原始集合。所以 name是 引用，"Ferris"前要加 &
        match name {
            &"Ferris" => {
                println!("There** is a rustacean among us! {}",name);
            }
            _ => {println!("hello {}",name)}
        }
    }

    println!("{:?}", pets);  // 这里还可以使用 pets。



    let mylikes = vec!["Bob","Ferris","Frank"]; // 如果这里 是 ["Bob","Ferris","Frank"]，为什么不会报错？
    for like in mylikes.into_iter() {  // mylikes的所有权已经移动走了，移动到迭代器里去了，后面无法使用mylikes
        match like {
            "Ferris" => {println!("There is a rustacean among us!");}
            _ => println!("Hello {}",like)
        }
    }

    //println!("{:?}",mylikes)
    // 这句话 ^ 会报错

    let v = vec!["a".to_string(), "b".to_string()];
    for s in v.into_iter() {
        // s has type String, not &String, 如果用iter()迭代，这里的s 是 &String，因为 iter()迭代是引用里面的元素
        println!("{}", s);
    }



    let mut rustacean = vec!["Bob", "Ferris", "Frank"];
    for ru in rustacean.iter_mut(){ // iter_mut 会生成一个可变的借用，可实现就地更改集合数据
        *ru =match  ru {
            &mut "Ferris" =>"There is a rustacean among us!",
            _ =>"Hello "
        }
    }
    println!("iter_mut:{:?}", rustacean);

    // &mut "Ferris" =>{
    //                 let mut bb=String::new();
    //                 //let aa= ru.to_string();
    //                 bb +=*ru;
    //                 bb+=" There is a rustacean among us!";
    //                 bb.as_str()},


    //死循环
    /*let ones = std::iter::repeat(1);
    let least = ones.min().unwrap(); // Oh no! An infinite loop!
    // `ones.min()` causes an infinite loop, so we won't reach this point!
    println!("The smallest number one is {}.", least);*/


}

