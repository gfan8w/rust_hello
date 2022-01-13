///
/// 这里主要演示把mod分散到各个文件里去，模块化更好，
/// 所有mod 在同一个文件的版本是front_of_house，hosting,
/// 分散到各个mod 文件的版本是 front_of_house2，hosting2
/// restaurant2是文件分散版本的restaurant，
/// 其实restaurant2 这个文件不需要，以下内容可以直接放到lib.rs,但这里偏要保持lib.rs的纯净，故特意放置一个restaurant2。
///
/// 1）建立src/front_of_house2.rs文件，里面引用 hosting2 模块，
/// 2）建立src/front_of_house 文件夹，里面放置 hosting2.rs 代码文件
/// 3）在lib.rs中放置 `mod front_of_house2`;使得 restaurant2 能找到它
/// 4）在外部其他地方可以使用 hosting2 和eat_at_restaurant，但无法访问front_of_house2

//在外部其他地方可以使用 hosting2 和eat_at_restaurant，但无法访问front_of_house2
pub use super::front_of_house2::hosting2;

pub fn eat_at_restaurant() {
    hosting2::add_to_waitlist();
    hosting2::add_to_waitlist();
    hosting2::add_to_waitlist();
}