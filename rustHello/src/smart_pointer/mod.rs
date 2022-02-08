mod pointer;
mod MyAllocator;
mod cow_sample;
mod url_parse;
mod cow_serde;
mod cow_mutex;
pub mod Box_sample;
mod stack_string;


/*

在 Rust 中，凡是需要做资源回收的数据结构，且实现了 Deref/DerefMut/Drop，都是智能指针。

智能指针一定是一个胖指针，但胖指针不一定是一个智能指针。

智能指针 String 和 &str 的区别：
String 除了多一个 capacity 字段，似乎也没有什么特殊。
但 String 对堆上的值有所有权，而 &str 是没有所有权的，

这是 Rust 中智能指针和普通胖指针的区别。

String是智能指针，
用于在堆上分配内存的 Box 和 Vec、
用于引用计数的 Rc 和 Arc 。
很多其他数据结构，如  PathBuf、Cow<'a, B>、MutexGuard、RwLockReadGuard 和 RwLockWriteGuard 等也是智能指针。





*/

















