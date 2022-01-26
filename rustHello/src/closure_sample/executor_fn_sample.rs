
// 一个 Fn closure 的例子，很多类库中使用的闭包技巧

// 有些trait是静态的，第一个参数不是self ，adhoc_polymorphism_howto_trait 例子里的 parse 方法是 trait 的静态方法，
// 因为它的第一个参数和 self 无关，所以在调用时需要使用 T::parse(str) ，例如 u8::parse("123)
trait Executor {
    fn exec(&self, cmd: &str) -> Result<String, &'static str>;
}


struct BashExector {
    env: String
}

impl Executor for BashExector {
    fn exec(&self, cmd: &str) -> Result<String, &'static str> {
        Ok(format!("fake cmd:{}, env:{}",cmd,self.env))
    }
}


fn run_main(){

    let env ="PATH=/usr/bin".to_string();
    let cmd ="cat /etc/passwd".to_string();

    let bash =BashExector{
        env:env.clone()
    };

    // use bash to exec cmd
    let r = execute(&cmd,bash);
    println!("exec r:{:?}",r);

    // exec 能执行 BashExecutor的命令，那如下的代码 ，要如何执行呢？尝试补足代码
    // adhoc exector to run cmd
    let r1 = execute(&cmd, |cmd: &str|{
        Ok(format!("glad to exec: cmd:{}",cmd))
    });

    println!("exec adhoc:{:?}",r1);



}

fn execute(cmd: &str, e: impl Executor) -> Result<String, &'static str> {
    e.exec(cmd)
}


///补充的代码：给 泛型F 实现Executor,
impl<F> Executor for F
where F: Fn(&str) -> Result<String, &'static str> {
    fn exec(&self, cmd: &str) -> Result<String, &'static str> {
        self(cmd)  // 这个 Self 是 fn cmd_fn(str:&str) ->Result... 好像就是F的约束，是函数签名， ？？？
                   // 这里的 self 是|cmd: &str|{Ok(format!("glad to exec: cmd:{}",cmd)      ？？ //todo()!
        //println!("haha");    // 把 它替换为下面2句话，上面的 glad to exec 根本就不会执行！！！所以说 self实例是那段函数
        //Ok("aa".to_string())
    }
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_main() {
        run_main();
    }
}

































