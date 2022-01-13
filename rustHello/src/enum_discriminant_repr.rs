use std::mem;
use std::mem::size_of;

/// 每个成员是u8类型 后面的值不能重复，不能超过最大值，如果给B 指定 255，则编译报错
#[repr(u8)]
enum MyEnum {
    A = 0,
    B , //=255,   // 编译报错
    C,
}

///枚举类型后面数值类型，英文里叫 discriminant。 后面的值不能重复，不能超过最大值，
/// discriminant 的值默认是 isize 类型
enum Foo {
    Bar,
    Baz =100,
    Quux
}

/// 零变量 Zero-variant Enums
enum ZeroVariants {}


pub fn run () {

    let size = size_of::<MyEnum>(); // 占多少个字节， u8 占 1个字节，可以把结构体上的 u8 改为 u32，u64 查看效果

    println!("size of MyEnum:{}",size);

    let foo_baz_disc = Foo::Baz;


    let foo_baz_discriminant = Foo::Baz as u32;
    println!("foo_baz_discriminant:{}",foo_baz_discriminant);

    let discm_myenum_a =mem::discriminant(& Foo::Baz);
    println!("foo_baz_discriminant:{:?}",discm_myenum_a);



}

