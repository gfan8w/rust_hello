mod blog;
mod draw;
mod formatter;
mod look_into_vtable;
mod SentenceIter;
mod important_trait;

use std::fs::File;
use std::io::Write;
use blog::post::run as blog_run;

pub fn run(){

    draw::run();

    //blog, OOP的设计方法
    blog_run();

    formatter::run();

    let_try_trait();

    look_into_vtable::veiry_address();
}


fn let_try_trait(){
    let mut f =File::create("./tmp_file.txt").unwrap();
    let w: &mut dyn Write = &mut f;
    w.write_all(b"hello").unwrap();

    //let w1 =w.by_ref();     // 这里无法通过编译，by_ref是变为 Self，但trait object 是抹去了 类型信息的。 返回类型中的 Self 需要是Sized，而 dyn Write 不是Sized。
    //w1.write_all(b" good");

}



