use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use color_backtrace::termcolor::BufferWriter;


//泛型参数的约束都发生在开头 struct 或者 enum 的定义中，
// 其实，很多时候，我们也可以在不同的实现下逐步添加约束
// 这里演示 参数多态，泛型参数约束


/*
参数多态 (parametric polymorphism) 是指，代码操作的类型是一个满足某些约束的参数，而非具体的类型。 参数多态通过泛型来支持

特设多态 （adhoc polymorphism） 是指同一种行为有多个不同实现的多态。
比如加法，可以 1+1，也可以是 “abc” + “cde”、matrix1 + matrix2、甚至 matrix1 + vector1。
在面向对象编程语言中，特设多态一般指函数的重载。  特设多态通过 trait 来支持

子类型多态（subtype polymorphism）是指，在运行时，子类型可以被当成父类型使用。 子类型多态可以用 trait object 来支持，
*/


/// 定义一个带有泛型参数 R 的 reader，此处我们不限制 R
struct MyReader<R> {
    reader: R,
    buf: String,
}


/// 实现 new 函数时，我们不需要限制 R
impl<R> MyReader<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buf: String::with_capacity(1024)
        }
    }
}

/// 定义 process 时，我们需要用到 R 的方法，此时我们限制 R 必须实现 Read trait
impl<R> MyReader<R> where R: Read {
    fn process(&mut self) -> std::io::Result<usize> {
        self.reader.read_to_string(&mut self.buf)
    }
}


fn run_read()  {
    let f = File::open("db.txt").unwrap();
    let mut my_read = MyReader::new(BufReader::new(f));
    if let Ok(s) = my_read.process() {
        println!("total size read: {}", s);
    }
}





pub fn run(){
    run_read();
    run_write();
}


#[derive(Debug)]
struct MyWrite<W> {
    write: W
}


impl<W: Write> MyWrite<W> {
    //原始1）
    /*pub fn new(addr: &str) -> Self {
        let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
        //let aa =BufWriter::new(stream);
        Self {

            write:BufWriter::new(stream)
        }
    }*/

    //修改后1）
    /*pub fn new(write: W) -> Self {
        let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
        //let aa =BufWriter::new(stream);
        Self {
            write
        }
    }*/


    //修改2）
    /*pub fn new(stream: W) -> Self {
        //let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
        //let aa =BufWriter::new(stream);
        Self {

            write:BufWriter::new(stream)
        }
    }*/




    pub fn write(&mut self, buf: &str) ->std::io::Result<()> {
        let bytes =buf.as_bytes();
        self.write.write_all(bytes);
        self.write.flush()
    }
}


//修改3） 修改3 的改法是 给MyWrite实现特定类型的，比如这里可以给TcpStream实现write， 也可以给File 实现write
impl MyWrite<BufWriter<TcpStream>> {
    pub fn new(add: &str) -> Self {
        let tcp =TcpStream::connect(add).unwrap();
        Self {
            write: BufWriter::new(tcp)
        }
    }
}

// 修改3）如果给MyWrite实现了 File 这个特定类型的，后面调用new 的时候，要用turbofish指定类型，因为有2个一个是TcpStream，一个是File
impl MyWrite<BufWriter<File>> {
    pub fn new(add: &str) -> Self {
        let f = File::open(add).unwrap();
        Self {
            write: BufWriter::new(f)
        }
    }
}






fn run_write(){
    //原始1）
    //let mut my_write =MyWrite::new("127.0.0.1:8080");

    //修改后1）
    /*let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let buff_write =BufWriter::new(stream);
    let mut my_write =MyWrite::new(buff_write);
*/

    //修改后2）
    /*let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let mut my_write = MyWrite::new(stream);*/



    //修改3）, 要用turbofish指定类型，因为有2个一个是TcpStream，一个是File
    //let mut my_file_write = MyWrite::<BufWriter<File>>::new("./buf_my_write.txt");
    //my_file_write.write("hello");
    //let mut my_write =MyWrite::<BufWriter<TcpStream>>::new("127.0.0.1:8080");





    //my_write.write("hello");
}















