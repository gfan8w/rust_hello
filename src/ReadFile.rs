use std::collections::HashMap;
use std::borrow::Borrow;
use std::io::{Read, Error};
use std::str;
use std::str::FromStr;

struct ToDo {
    // 使用 Rust 内置的 HashMap 来保存 key - val 键值对。
    map: HashMap<String, bool>,
    // 使用json来读取、保存数据
    json: HashMap<String, bool>,
}

impl ToDo {

    fn new()->Result<ToDo, std::io::Error> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        let mut content =String::new();
        file.read_to_string(&mut content);

        let mut map :HashMap<String,bool>= HashMap::new();
        for line in content.lines() {
            let splitline=line.splitn(2,"\t").collect::<Vec<&str>>();
            if(splitline.len()>0){
                map.insert(String::from(splitline[0]),true);
            }
            if(splitline.len()>1){
                map.insert(String::from(splitline[0]),bool::from_str(splitline[1]).unwrap());
            }
        }

        // 文件选项不再需要 mut f 来绑定，因为我们不需要像以前一样手动将内容分配到 String 中。Serde 会来处理相关逻辑。
        // 我们将文件拓展名更新为了 db.json。
        // serde_json::from_reader [文档] 将为我们反序列化文件。它会干扰 map 的返回类型，
        // 并会尝试将 JSON 转换为兼容的 HashMap。如果一切顺利，我们将像以前一样返回 Todo 结构。
        // Err(e) if e.is_eof() 是一个匹配守卫，可让我们优化 Match 语句的行为。
        // 如果 Serde 作为错误返回一个过早的 EOF（文件结尾），则意味着该文件完全为空（例如，在第一次运行时，或如果我们删除了该文件）。
        // 在那种情况下，我们从错误中恢复并返回一个空的 HashMap。
        // 对于其它所有错误，程序会立即被中断退出。
        let jsonFile = std::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open("db.json")?;
        let mut json:HashMap<String, bool> = HashMap::new();
        match serde_json::from_reader(jsonFile){
            Ok(j) => {
                json=j;
            },
            Err(e) if e.is_eof() => {
                json=HashMap::new();
            },
            Err(e) =>panic!("json error:{}",e),
        }

        Ok(ToDo{map,json})


       /* let map = content
            .lines()
            .map(|line| line.splitn(2,"/t").collect::<Vec<&str>>())
            .map(|kv|(kv[0],kv[1]))
            .map(|(k,v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect::<HashMap<String,bool>>();
        Ok(ToDo{map})*/

    }


    fn insert(&mut self, key:String){
        // 在我们的 map 中新增一个新的元素。
        // 我们默认将其状态值设置为 true
        // 在 Rust 中，每个变量默认是不可变的。
        // 如果你想改变一个值，你需要使用 mut 关键字来给相关变量加入可变性。
        // 由于我们的函数需要通过修改 map 来添加新的值，因此我们需要将其设置为可变值。
        // & 认为这个变量是一个指针，指向内存中保存这个值的具体地方，并不是直接存储这个值。
        // 在 Rust 属于中，这被认为是一个借用（borrow），意味着函数并不拥有该变量，而是指向其存储位置。
        self.map.insert(key.clone(),true);

        self.json.insert(key.clone(),true);
    }
    fn save(&self) ->Result<(),std::io::Error> {
        let mut content=String::new();
        for (k,v) in &self.map{
            let record=format!("{}\t{}\n",k,v);
            content.push_str(&record);
        }
        std::fs::write("db.txt",content);

        // 保存为json格式
        self.saveJson();

        Ok(())
    }

    fn saveJson(&self) ->Result<(), Box<dyn std::error::Error>> {
        // Box<dyn std::error::Error>。这次我们返回一个包含 Rust 通用错误实现的 Box。
        // 简而言之，Box 是指向内存中分配的指针。
        // 由于打开文件时可能会返回 Serde 错误，所以我们实际上并不知道函数会返回这两个错误里的哪一个。
        // 因此我们需要返回一个指向可能错误的指针，而不是错误本身，以便调用者处理它们。
        // 我们当然已经将文件名更新为 db.json 以匹配文件名。
        // 最后，我们让 Serde 承担繁重的工作：将 HashMap 编写为 JSON 文件。
        let jf = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;

        serde_json::to_writer_pretty(jf, &self.json)?;

        Ok(())

    }


    fn complete(&mut self, key: &String) -> Option<()> {
       match self.map.get_mut(key) {
           Some(x) => Some(*x=!*x),
           None =>None,
       }
    }

}

pub fn run(){
    // 运行时 需要加参数
    let action = std::env::args().nth(1).expect("please specify an action");
    let item= std::env::args().nth(2).expect("please specify and value");

    let mut data =ToDo::new().expect("init db.txt failed");

    if(action=="add"){
        data.insert(item);
        match data.save(){
            Ok(t) => println!("saved data"),
            Err(why) =>println!("error when saving:{}",why),
        }
    }else if(action=="complete"){
        //如果我们检测到返回了 Some 值，则调用 todo.save 将更改永久存储到我们的文件中。
        // 我们匹配由 todo.complete(&item) 方法返回的 Option。
        // 如果返回结果为 None，我们将向用户打印警告，来提供良好的交互性体验。
        // 我们通过 &item 将 item 作为引用传递给“todo.complete”方法，
        // 以便 main 函数仍然拥有该值。这意味着我们可以再接下来的 println! 宏中继续使用到这个变量。
        // 如果我们不这样做，那么该值将由“complete”用于，最终被意外丢弃。
        // 如果我们检测到返回了 Some 值，则调用 todo.save 将此次更改永久存储到我们的文件中。
        match data.complete(&item) {
            None => println!("no {} found",item),
            Some(_) => match data.save() {
                Ok(_) => {println!("todo saved")}
                Err(why) => {println!(" got error：{}",why)}
            }
        }
    }



}


