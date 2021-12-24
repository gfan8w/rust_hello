
///这里以餐厅前厅举例，餐厅一般包含2部分，一部分是前厅，给接待顾客使用，第二不分手后厨，处理食物用，客人不可见、无感知
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }

    pub mod cooking {
        pub fn heat_steak(){}
    }
}

pub use crate::restaurant::front_of_house::hosting; // 没这句话，下面三个hosting 不能直接引用,不包括前面的pub,
// pub的作用是从新导出。Re-exporting，对内屏蔽细节，对外可见hosting
// 因为cooking没有导出，所以外部不可见

/// 客人只知道 在餐厅吃饭
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    //没有使用use，就只能全名称限定的访问
    super::restaurant::front_of_house::cooking::heat_steak();
}