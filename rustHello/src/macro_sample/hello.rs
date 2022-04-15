

///
// 在一个宏中，可以有多个分支，宏根据不同的参数展开到不同的代码。每个分支可以接收多个参数，这些参数使用$符号开头，然后跟着一个 token 类型：
//
// item ——一个项（item），像一个函数，结构体，模块等。
// block ——一个块 （block）（即一个语句块或一个表达式，由花括号所包围）
// stmt —— 一个语句（statement）。比如一个赋值语句。
// pat ——一个模式（pattern）
// expr —— 一个表达式（expression）
// ty ——一个类型（type）。比如 Vec。
// ident—— 一个标识符（indentfier）。比如一个变量名。
// path —— 一个路径（path）（例如，foo，::std::mem::replace，transmute::<_, int>，...）
// meta —— 一个元数据项；一般是在 #[...]  和  #![...]  属性内部的数据。
// tt—— 一个词法树。单个的 token 树。
// vis——一个可能为空的Visibility修饰符。比如 pub、pub(crate)。
///




macro_rules! add {
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


macro_rules! add_as {
    // using a ty token type for macthing datatypes passed to maccro
    // 这里用一个类型标识符
    ($a:expr, $b:expr, $typ:ty) => {
         $a as $typ + $a as $typ
    };
}


///重复的 token 类型被$()包裹，后面跟着一个分隔符和一个*或一个+，
/// 表示这个 token 将会重复的次数。分隔符用于多个 token 之间互相区分。
/// $()后面跟着*和+用于表示重复的代码块。在上面的例子中，+$a是一段重复的代码。
macro_rules! add1_as {

    (
        // repeated block
        $($a:expr)
        // seperator
        ,
        // zero or more
        *
    ) => {
        {
            // to handle the case without any arguments
            0
            // block to be repeated,前面有个 + 是 跟前面 做加法
            $(+$a)*
        }
    };
}

///TT muncher 以递归方式分别处理每个 token，每次处理单个 token 也更为简单。这个宏有三个分支：
//
// 第一个分支处理是否单个参数通过的情况
// 第二个分支处理是否两个参数通过的情况
// 第三个分支使用剩下的参数再次调用add宏
macro_rules! add2_as {
    // first arm in case of single argument and last remaining variable/number
    ($a:expr) => {
        $a
    };
    // second arm in case of two arument are passed and stop recursion in case of odd number ofarguments,只匹配偶数个参数
    ($a:expr, $b:expr) =>{
        {
            $a + $b
        }
    };
    // add the number and the result of remaining arguments，递归调用
    ($a:expr, $($b:tt)*) => {
        {
            $a + add2_as!($($b)*)
        }
    }
}

//下面演示：
//解析结构体的名字及其字段
//一个struct（即结构体）声明在其开头有一个可见性关键字（比如pub ） ，后面跟着struct关键字，然后是struct的名字和struct的主体。


//如下是一个没有成员的结构体
//$vis将会拥有可见性，$struct_name将会拥有一个结构体名。为了让一个结构体是公开的，我们只需要添加pub关键字并忽略$vis变量。
macro_rules! make_public1 {
    // use vis type for visibility keyword and ident for struct name
    ($vis: vis struct $struct_name:ident {}) => {
        {
            pub struct $struct_name { }
        }
    };
}

// 包含多个字段的结构体
//一个struct可能包含多个字段，这些字段具有相同或不同的数据类型和可见性。
// ty token 类型用于数据类型，vis用于可见性，ident用于字段名。我们将会使用*用于零个或更多字段。
macro_rules! make_public2 {
    // use vis type for visibility keyword and ident for struct name
    (
        $vis: vis struct $struct_name:ident
        {
            $(
             // vis for field visibility, ident for field name and ty for field data type
                $field_vis:vis $field_name:ident : $field_type:ty
            ) , *
        }
    ) => {
        {
            pub struct $struct_name
            {
                $(
                  pub $field_name : $field_type ,
                )*
            }
        }
    };
}

//从struct中解析元数据
//通常，struct有一些附加的元数据或者过程宏，比如#[derive(Debug)]。这个元数据需要保持完整。解析这类元数据是通过使用meta类型来完成的。
macro_rules! make_public3 {
    // use vis type for visibility keyword and ident for struct name
    (
        // meta data about struct
        $(#[$meta: meta])*
        $vis: vis struct $struct_name:ident
        {
            $(
                // meta data about field
                $(#[$field_meta:meta])*
                $field_vis:vis $field_name:ident : $field_type:ty
            ) , * $(,)+
        }
    ) => {
        {
            $(#[$meta])*
            pub struct $struct_name
            {
                $(
                $(#[$field_meta])*
                  pub $field_name : $field_type ,
                )*
            }
        }
    };
}









struct Stuent {}


pub fn run(){
    println!("Here demo Macro");
    let a=add!(1,2);
    println!("a:{}",a);
    let a1= add!(5);
    println!("a:{}",a1);

    let a2 = add_as!(1,2, u8);
    println!("a2 as u8:{}", a2);

    let a3 =add1_as!(1,2);
    println!("a3, add repeat:{}",a3);

    // 这个允许传入0 个参数
    let a4 =add1_as!();
    println!("a3, add repeat:{}",a4);

    let a5 =add2_as!(1,2,3,4,5);
    println!("a3, add repeat:{}",a5);

    //给结构体 S  变为 pub
    let s1=make_public1!{
        struct S { }
    };

    println!("{:?}",stringify!(s1));


    let s2=make_public2!{
        struct S { name:String, age:u32, gender:bool }
    };

    println!("{:?}",stringify!(s2));

    let s3=make_public3!{
        #[derive(Debug)]
        struct S { name:String, age:u32, gender:bool, }  //这里必须以 逗号结尾，请仔细看宏的定义
    };

    println!("{:?}",stringify!(s3));


    expand();



    println!("Demo Macro End");
}



fn expand(){

    struct Name{
        n:i64,
        t:i64,
        g:i64,
    }

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Name {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Name { n: ref __self_0_0, t: ref __self_0_1, g: ref __self_0_2
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Name");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                                                        "n", &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                                                        "t", &&(*__self_0_1));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                                                        "g", &&(*__self_0_2));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }


    let s=Name{
        n: 1,
        t: 2,
        g: 3
    };

    println!("{:?}",s);


}









//参考：https://zhuanlan.zhihu.com/p/353421021

































