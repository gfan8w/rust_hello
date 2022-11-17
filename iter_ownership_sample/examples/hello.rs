
pub struct ObjectList {
    pub kind: String,

    pub items: Vec<Object>,

}

pub struct Object {
    /// The kind of item this is. For objects, this is always `storage#object`.
    pub kind: String,
    /// The ID of the object, including the bucket name, object name, and generation number.
    pub id: String,
    /// The link to this object.
    pub self_link: String,
    /// The name of the object. Required if not specified by URL parameter.
    pub name: String,
}

#[derive(Debug)]
struct Car {
    id: String,
    brand: String,
    model: String,
    year: u16,
}

/*
    iter()的签名是: iter(&self) 是引用
    into_iter()的签名是 into_iter(self)， 把自己移动进去了
    iter_mut() 的签名是 iter_mut(&mut self)， 把可变的引用移动进去
*/

fn select_car() {

    let cars: [Car; 3] = [
        Car {
            id: "1".to_owned(),
            brand: "Ford".to_owned(),
            model: "Bronco".to_owned(),
            year: 2022,
        },
        Car {
            id: "2".to_owned(),
            brand: "Hyundai".to_owned(),
            model: "Santa Fe".to_owned(),
            year: 2010,
        },
        Car {
            id: "3".to_owned(),
            brand: "Dodge".to_owned(),
            model: "Challenger".to_owned(),
            year: 2015,
        },
    ];

    let car_id: String = "3".into();

    // iter是借用集合里的每个元素，集合不变，后续还能使用
    let car_index_option = cars.iter().position(|car: &Car| {
        // 每次循环时的car是一个指针，是引用
        let cc  = car;

        // let dd = *car;  //因为没有实现copy，无法移动

        let Car{id: my_id, .. } = cc;  // 可以， 部分移动？？？？
        let a_id = my_id;
        println!("a_id:{}", a_id);

        //let b_id = cc.id;             //无法把id的所有权移动到 b_id 上
        //println!("b_id:{}", b_id);


        &car.id == &car_id

    });
    println!("car index:{:?}", car_index_option);

    // 先理解 select_i64() 的例子，再来理解这段
    let cc = cars
        .iter()
        .map(|v: &Car| &v.id)
        .filter(|car_id: &&String |{
            let c = car_id;
            println!("{:?}", c);
            c.as_str()>=&&"2"
        }).collect::<Vec<&String>>();
    println!("cc:{:?}",cc);

    let dd = cars
        .iter()
        .filter(|car |{
            let c = *car;  // 这里本来发生了移动，但我没使用，编译器直接忽略了??????
            println!("{:?}", c);
            (**car).id>"2".to_string()
        }).collect::<Vec<_>>();
    println!("dd:{:?}",dd)
}

fn select_i64(){
    let current_word_id:i64 =1;
    let wordIds:Vec<i64> = Vec::new();

    // 这个版本无法工作。
    // let current_word_id:i64 =1;
    // let wordIds:Vec<i64> = Vec::new();;
    // let current_sentences: Vec<i64> = wordIds
    //     .iter()
    //     .filter(|sen| sen == current_word_id)
    //     .collect();

    /*
    into_iter()的签名是 into_iter(self)， 把自己移动进去了
    iter_mut() 的签名是 iter_mut(&mut self)， 把可变的引用移动进去
    iter()的签名是: iter(&self) 是引用，它返回值是 Iter<'_, T>，可以转换为：Iterator<Item=&i64>,
    filter(..) 函数的P的约数写的是 P: FnMut(&Self::Item) -> bool,它要求传入的是 &Self::Item， 即 & &64
    如下的map，它的 参数F: FnMut(Self::Item) ，传入的是自身(移动了！！？？)，对应此场景，自身是 &i64
    如下的collect， 它的参数入参是 Self，是拿走所有权的
    */

    let current_sentences: Vec<i64> = wordIds
        .iter() // 这里出来结果变为 &i64
        .map(|v| *v)// 这里进入是&i64，出来是 i64
        .filter(|sen| sen == &current_word_id)  //这里进入是 &i64，与之对比的也要是 &i64的current_word_id
        .collect::<Vec<i64>>();    //这里进入的s &i64, 本身是什么，进入的就是什么，本身是&64

    println!("{:?}",current_sentences);

    wordIds.iter().filter(|x: &&i64|{
        let a = x;
        println!("{}", a);

        a>&&1  // 因为 a 是 &&i64，使得 1 与之匹配，是用 &&1
    });

    //使用rust doc上的建议，这样写：
    // https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.filter
    wordIds.iter().filter(|&x| {
        let a = *x;
        a > 1i64
    });

    //次之的写法: 这个写法，有2个大坑！！！！！ 如果是对象，会移动
    // destructure `&&i64` to `i64`
    wordIds.iter().filter(|&&x| {
        let a = x;
        a > 1i64
    });

    wordIds.iter().filter(|x| {
        let a = **x;
        a > 1i64
    });

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_car(){
        select_car();
    }

    #[test]
    fn test_collect_i64(){
        select_i64()
    }
}




fn main() {
    select_car()
}