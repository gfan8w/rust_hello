macro_rules! sayhello {
    // `()` indicates that the macro takes no argument.
    () => {
       // The macro will expand into the contents of this block.
        println!("hello macro!")
    };
}

macro_rules! create_function {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for variable/function names.

    ($func_name: ident) =>{
        fn $func_name() {
            // The `stringify!` macro converts an `ident` into a string.
            println!("You called {:?}()", stringify!($func_name));
        }
    }
}

// Create functions named `foo` and `bar` with the above macro.
create_function!(foo);
create_function!(bar);


macro_rules! print_result {
    // This macro takes an expression of type `expr` and prints
    // it as a string along with its result.
    // The `expr` designator is used for expressions.
    ($expression: expr) =>{
        // `stringify!` will convert the expression *as it is* into a string.
        println!("{:?} = {:?}",
                    stringify!($expression),
                    $expression
        );
    }
}

//重载
//Macros can be overloaded to accept different combinations of arguments.
// In that regard, macro_rules! can work similarly to a match block:

// `test!` will compare `$left` and `$right`
// in different ways depending on how you invoke it:
macro_rules! test {
    // Arguments don't need to be separated by a comma.
    // Any template can be used!
    ($left:expr, and $right:expr) => {
        println!("{:?} and {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    };
  // ^ each arm must end with a semicolon.
    ($left:expr; or $right:expr) =>{
        println!("{:?} or {:?} is {:?}",
                    stringify!($left),
                    stringify!($right),
                    $left || $right)
    }
}

//重复Repeat

//Macros can use + in the argument list to indicate that an argument may repeat at least once,
// or *, to indicate that the argument may repeat zero or more times.

// In the following examples, surrounding the matcher with $(...),+ will match
// one or more expression, separated by commas. Also note that the semicolon is
// optional on the last case.

// `find_min!` will calculate the minimum of any number of arguments.
macro_rules! find_min{
    // Base case:
    ($x:expr) =>($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) =>{
        // Call `find_min!` on the tail `$y`
        std::cmp::min($x, find_min!($($y),+))
    }
}


macro_rules! calculate{
    (eval $e:expr) =>{{
        {
            let val: usize = $e; //Force types to be integers.
            println!("{} = {}", stringify!($e), val);
        }
    }}  //two pairs of braces in the macro. The outer ones are part of the syntax of macro_rules!
}

#[macro_export] // 注解说明宏应该是可用的。 如果没有该注解，这个宏不能被引入作用域。
macro_rules! vec {
    ( $( $x:expr ),* ) =>{
        {
            let mut temp_vc = Vec::new();
            $(
                temp_vc.push($x);
            )*
             temp_vc
         }
    }
}
/*
使用 macro_rules! 和宏名称开始宏定义，且所定义的宏并 不带 感叹号。名字后跟大括号表示宏定义体，在该例中宏名称是 vec 。

vec! 宏的结构和 match 表达式的结构类似。此处有一个单边模式 ( $( $x:expr ),* ) ，后跟 => 以及和模式相关的代码块。
如果模式匹配，该相关代码块将被执行。假设这是这个宏中唯一的模式，则只有这一种有效匹配，其他任何匹配都是错误的。更复杂的宏会有多个单边模式。

宏定义中有效模式语法和在第十八章提及的模式语法是不同的，因为宏模式所匹配的是 Rust 代码结构而不是值。回过头来检查下示例 19-28 中模式片段什么意思。
对于全部的宏模式语法，请查阅参考。

首先，一对括号包含了全部模式。接下来是后跟一对括号的美元符号（ $ ），其通过替代代码捕获了符合括号内模式的值。$() 内则是 $x:expr ，
其匹配 Rust 的任意表达式或给定 $x 名字的表达式。

$() 之后的逗号说明一个逗号分隔符可以有选择的出现代码之后，这段代码与在 $() 中所捕获的代码相匹配。
紧随逗号之后的 * 说明该模式匹配零个或多个 * 之前的任何模式。

当以 vec![1, 2, 3]; 调用宏时，$x 模式与三个表达式 1、2 和 3 进行了三次匹配。

现在让我们来看看这个出现在与此单边模式相关的代码块中的模式：在 $()* 部分中所生成的 temp_vec.push() 为在匹配到模式中的 $() 每一部分而生成。
$x 由每个与之相匹配的表达式所替换。当以 vec![1, 2, 3]; 调用该宏时，替换该宏调用所生成的代码会是下面这样：


let mut temp_vec = Vec::new();
temp_vec.push(1);
temp_vec.push(2);
temp_vec.push(3);
temp_vec

*/





//主入口函数
pub fn main() {
    // This call will expand into `println!("Hello");`
    sayhello!();

    foo();
    bar();

    print_result!(1u32 + 1);
    print_result!({
        let x= 1u32;
        x*x+2*x-1
    });

    //重载
    test!(1i32+1 ==2i32, and 2i32*2 == 4i32);
    test!(true; or false);

    //重复
    println!("{}",find_min!(1u32));
    println!("{}",find_min!(1u32+2,2u32));
    println!("{}",find_min!(1u32+2,2u32,2u32*3));


    calculate!{
        eval 1+2 //hehe, rust中不存在eval关键词，但这里也能工作
    }

    calculate!{
        eval (1+2)*(3/4)
    }

    let v = vec![1,2,3];

    let j=0;

}





