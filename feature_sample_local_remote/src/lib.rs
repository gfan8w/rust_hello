
#[cfg(feature = "memorydisk")]
use memorydisk; //引用了另外一个依赖，注意该依赖被标记为 optional，optional表示隐式的引入一个跟dependency同名的feature，
                // 其他feature要包含本例的feature才能编译通过


// 该项目有一个 default的feature：local_computer，在目标那里使用该库的时候，演示disable default，使用 remote_computer

// 这里有2个process，所以，local_computer 和 remote_computer 只能enable一个，不能2个同时enable

pub fn process(){
    #[cfg(feature="local_computer")]
    process_local_computer();

    #[cfg(feature="remote_computer")]
    process_remote_computer();

}

#[cfg(feature="local_computer")]
pub fn process_local_computer(){
    println!("process local_computer");

    #[cfg(feature = "memorydisk")]
    memorydisk::write();
}

#[cfg(feature="remote_computer")]
pub fn process_remote_computer(){
    println!("process remote_computer");

    #[cfg(feature = "memorydisk")]
    memorydisk::write();
}

#[cfg(feature="local_computer")]
pub fn process_local_version(){
    println!("process local computer version");

    #[cfg(feature = "memorydisk")]
    memorydisk::write();
}

#[cfg(feature="remote_computer")]
pub fn process_remote_version(){
    println!("process remote computer version");
    memorydisk::write();
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
