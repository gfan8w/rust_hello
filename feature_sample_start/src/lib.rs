///这里作为 feature演示的入口，从这里开始调用其他库，比如这里调用了 local_or_remote
/// 这里的依赖关系是 feature_sample(feature_sample_start) -> feature_sample_local_remote  -> feature-sample-memorydisk
/// 1) feature_sample_start 这个，即本库，外部调用它的时候，设置为 disable default，enable ico。
/// 2）feature_sample_local_remote，在本库调用它的时候，设置为 disable default，enable remote_computer，
///    remote_computer隐式的依赖 feature-sample-memorydisk
/// 3）feature-sample-memorydisk，在feature_sample_local_remote库依赖它的时候，写为 optional，然后 remote_computer 必须包含它
///
///
/// cargo tree -f "{p} {f}" 输出如下：显示一个紧凑的列表，有哪些features被使用了。
//rustHello v0.1.0 (/Volumes/datadisk/tech/github/gfan8w/rusthellosample/rustHello)
// ├── feature_sample_entry v0.3.4 (/Volumes/datadisk/tech/github/gfan8w/rusthellosample/feature_sample_start) bmp,default,ico,png,webp (*)
//
//
// feature-sample-memorydisk v0.1.0 (/Volumes/datadisk/tech/github/gfan8w/rusthellosample/feature-sample-memorydisk) default,disk,memory
//
// feature_sample_entry v0.3.4 (/Volumes/datadisk/tech/github/gfan8w/rusthellosample/feature_sample_start) bmp,default,ico,png,webp
// └── feature_sample_local_remote v0.3.5 (/Volumes/datadisk/tech/github/gfan8w/rusthellosample/feature_sample_local_remote) default,local_computer,memorydisk,remote_computer
//     └── feature-sample-memorydisk v0.1.0 (/Volumes/datadisk/tech/github/gfan8w/rusthellosample/feature-sample-memorydisk) default,disk,memory
//
// feature_sample_local_remote v0.3.5 (/Volumes/datadisk/tech/github/gfan8w/rusthellosample/feature_sample_local_remote) default,local_computer,memorydisk,remote_computer (*)
/// 另外一个命令：更有用，反向的显示一个模块所以来的feature，例如我们看`feature-sample-memorydisk`这个模块的feature激活路线，
/// 如果是直接运行 rustHello程序，则整个激活路径是：
/// rustHello(default) =>
///        feature_sample_entry(ico->bmp) =>
///                   feature_sample_local_remote(remote_computer->memorydisk) =>
///                                                                feature-sample-memorydisk(disk)
/// cargo tree -e features -i feature-sample-memorydisk
//
// feature-sample-memorydisk v0.1.0 (/Volumes/datadisk/tech/github/gfan8w/rusthellosample/feature-sample-memorydisk)
// ├── feature-sample-memorydisk feature "default" (command-line)
// ├── feature-sample-memorydisk feature "disk"
// │   └── feature_sample_local_remote v0.3.5 (/Volumes/datadisk/tech/github/gfan8w/rusthellosample/feature_sample_local_remote)
// │       ├── feature_sample_local_remote feature "default" (command-line)
// │       ├── feature_sample_local_remote feature "local_computer"
// │       │   └── feature_sample_local_remote feature "default" (command-line)
// │       ├── feature_sample_local_remote feature "memorydisk"
// │       │   └── feature_sample_local_remote feature "remote_computer"
// │       │       └── feature_sample_entry v0.3.4 (/Volumes/datadisk/tech/github/gfan8w/rusthellosample/feature_sample_start)
// │       │           ├── feature_sample_entry feature "bmp"
// │       │           │   └── feature_sample_entry feature "ico"
// │       │           │       └── rustHello v0.1.0 (/Volumes/datadisk/tech/github/gfan8w/rusthellosample/rustHello)
// │       │           │           └── rustHello feature "default" (command-line)
// │       │           │       └── feature_sample_entry feature "default" (command-line)
// │       │           ├── feature_sample_entry feature "default" (command-line)
// │       │           ├── feature_sample_entry feature "ico" (*)
// │       │           ├── feature_sample_entry feature "png"
// │       │           │   └── feature_sample_entry feature "ico" (*)
// │       │           └── feature_sample_entry feature "webp"
// │       │               └── feature_sample_entry feature "default" (command-line)
// │       └── feature_sample_local_remote feature "remote_computer" (*)
// └── feature-sample-memorydisk feature "memory"
//     └── feature-sample-memorydisk feature "default" (command-line)
///
#[cfg(feature = "webp",)]
mod webp;

///这里只要ico、bmp、png满足一个即可
#[cfg(any(feature = "ico",feature = "bmp",feature = "png"),)]
mod ico;

 //这里使用了外部的一个crate，通过disable default，enable remote_computer 的方式来使用
use local_or_remote;

fn get_from_location(){
    local_or_remote::process();
}

fn get_image(){

    //只有enable bmp 这个feature的时候才会编译这个函数，如果是enable ico 也算是有效的
    #[cfg(feature = "bmp",)]
    ico::bmp::show();

    //只有enable png 这个feature的时候才会编译这个函数，如果是enable ico 也算是有效的
    #[cfg(feature = "png",)]
    ico::png::show();

    //只有enable webp 这个feature的时候才会编译这个函数
    #[cfg(feature = "webp",)]
    webp::show();
}


pub fn process_image(){
    get_from_location();
    get_image();
}




