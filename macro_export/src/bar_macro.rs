

// #[macro_use]  // 以前的版本需要在这里加这个。
pub mod foo {
    //在foo 这个mod里定义了 bar 这个宏
    #[macro_export]   // 这个 macro_export是导出到最顶部的 crate::barz!(..) 这样访问。 这里只有加了这个，外部其他crate才可以访问
     macro_rules! barz {
        // macth like arm for macro
        // first arm match add!(1,2), add!(2,3) etc
        ($a:expr, $b:expr) =>{
            // macro expand to this code
            {
                // $a and $b will be templated using the value/variable provided to macro
                $a + $b
            }
        };
        // Second arm macth add!(1), add!(2) etc
        ($a: expr) =>{
            {
                $a
            }
        }
    }

    pub(crate) use barz; // 把 barz 这个宏 重新导出，在本模块的其他地方 才可以使用，但外部其他crate不能访问，Rust 1.32, 2019-01-17之后在这里导出，不再用#[macro_use]
}

//参考： https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files?rq=1


