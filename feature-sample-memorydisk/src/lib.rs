
///别的库依赖如下：会显示 write to local disk
///  [dependencies.memorydisk]
///  path ="../feature-sample-memorydisk"
///  package="feature-sample-memorydisk"
pub fn write() {
    #[cfg(feature = "disk")]
        wirte_disk();
}

#[cfg(feature = "disk")]
fn wirte_disk(){
    println!("write to local disk")
}

#[cfg(feature = "memory")]
fn write_memory(){
    println!("write to memory")
}

