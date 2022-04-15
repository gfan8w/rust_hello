pub mod hello;


/*
声明宏（declarative macro），课程里出现过的比如像 vec![]、println!、以及 info!，它们都是声明宏。
声明宏可以用 macro_rules! 来描述，



Rust 还支持允许我们深度操作和改写 Rust 代码语法树的过程宏（procedural macro），更加灵活，更为强大。
Rust 的过程宏分为三种：

`函数宏（function-like macro）：看起来像函数的宏，但在编译期进行处理。
比如我们之前用过的 sqlx 里的 query 宏，它内部展开出一个 expand_query 函数宏。
你可能想象不到，看上去一个简单的 query 处理，内部有多么庞大的代码结构。

`属性宏（attribute macro）：可以在其他代码块上添加属性，为代码块提供更多功能。比如 rocket 的 get / put 等路由属性。

`派生宏（derive macro）：为 derive 属性添加新的功能。这是我们平时使用最多的宏，
比如
 #[derive(Debug)] 为我们的数据结构提供 Debug trait 的实现、
 #[derive(Serialize, Deserialize)] 为我们的数据结构提供 serde 相关 trait 的实现。

*/