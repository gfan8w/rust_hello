use std::fmt::Formatter;

///
/// 本代码演示 如何显示复杂类型
///


#[derive(Debug)]
struct Student {
    name: String,
    age: i16,
    address: String,
    salary: f64
}

///定义一个元组，列表，内部包含一个 vec，如何把它优雅的打印出来？
struct List(Vec<u8>);


struct City {
    ///name of the city
    name: &'static str,
    ///Latitude
    lat: f32,
    ///Longitude
    lon: f32
}



/// 给一个结构体显示详细信息，需要实现Display 这个trait
impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Student[name:{}, age:{}, address:{}, salary:{}]",
                 self.name, self.age, self.address, self.salary)

        //write!("Teacher[name:{}, age:{}, address:{}, salary:{}]", self.name, self.age, self.address, self.salary)
        // 这样会报错：，第一个参数前面需要把 Formatter传入，f是一个缓冲区
        // 22 |                  self.name, self.age, self.address, self.salary)
        //    |                  ^^^^^^^^^
        //    |
        // help: you might be missing a string literal to format with
        //    |
        // 22 |                  "{} {} {} {}", self.name, self.age, self.address, self.salary)
        //    |                  ^^^^^^^^^^^^^^
    }
}

///
/// 复杂类型，自定义输出格式。
///
impl std::fmt::Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let vec = &self.0; // 抽取出内部的对象

        // f 是缓冲区，往里写入数据,
        write!(f,"【")?;

        for (idx,val) in vec.iter().enumerate() {   // enumerate的带 index 和 value
            if (idx!=0) {
                write!(f,"， ")?;
            }
            write!(f,"{idx}:{val}",val=val,idx=idx)?
        }

        write!(f,"】")

        // 输出类似 ：【0:1， 1:2， 2:3】 这样
    }
}


impl std::fmt::Display for City {
    // f是一个缓冲区，格式化的字符串必须写入这个缓冲区
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lat_c = if self.lat>=0f32 {'N'} else {'S'};
        let lon_c = if self.lon>=0.0 {'E'} else {'W'};
        write!(f,"{}:{:.3}°{} {:.3}°{}", self.name,self.lat,lat_c,self.lon,lon_c)
    }
}




pub fn run (){
    let dad = Student {
        name:String::from("lint"),
        age:32,
        address:String::from("Shanghai Pudong"),
        salary: 32.45
    };
    // 如果要显示复杂对象，必须手工实现 Debug 的trait 或者 Display的trait
    println!("dad info is:{:?}", dad); // 调用debug ， {:#?} 加个 #是美化打印
    println!("teacher is {}",dad);      //调用format， 即Display

    let son = Student{
        name: String::from("Lilei"),
        age: 10,
        address: String::from("Hangzhou xihu"),
        salary: 0.0
    };

    println!("dad is :{da}, son is :{so}", so=son,da=dad); // 占位符的使用



    let list =List(vec![10,20,30,40]);
    println!("list:{}",list);


    for city in [
        City{name:"Dublin",     lat:53.347778,  lon:-6.259722},
        City{name:"Oslo",       lat:59.95,      lon:10.75},
        City{name:"Vancouver",  lat:49.25,      lon:-123.1}     ].iter() {  // iter是借用集合里的每个元素，集合不变，后续还能使用

        //注意，这里用了iter(), 每次循环时的city是一个指针，是引用
        //let a = *city; // 这里无法把所有权move到a上去

        // 调用自定义的Display
        println!("{}",*city);

        let b = city;
        println!("{}",b);
    }



}




