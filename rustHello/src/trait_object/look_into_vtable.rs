use std::fmt::{Debug, Display};
use std::mem;

pub fn veiry_address(){

    let s1 ="hello".to_string();
    let s2 ="world".to_string();

    // Display / Debug trait object for s1
    let display_s1: &dyn Display = &s1;
    let debug_s1: &dyn Debug =&s1;

    // Display / Debug trait object for s2
    let display_s2: &dyn Display =&s2;
    let debug_s2: &dyn Debug = &s2;

    // 强行把 triat object 转换成两个地址 (usize, usize)
    // 这是不安全的，所以是 unsafe

    let(add_display_s1,add_display_vtab_s1):(usize,usize) = unsafe { mem::transmute(display_s1) };
    let(add_debug_s1,add_debug_vtable_s1):(usize,usize) = unsafe { mem::transmute(debug_s1) };
    let(add_display_s2,add_display_vtab_s2):(usize,usize) = unsafe { mem::transmute(display_s2) };
    let(add_debug_s2,add_debug_vtable_s2):(usize,usize) = unsafe { mem::transmute(debug_s2) };

    // s 和 s1 在栈上的地址，以及 veiry_address 在 TEXT 段的地址
    println!("s1: {:p}, s2: {:p}, main:{:p}", &s1, &s2, veiry_address as *const ());


    // trait object(s1 / Display) 的 ptr 地址和 vtable 地址
    println!("s1 dispaly trait object ptr:0x{:x}, vtable:0x{:x}, v:0x{:x}, v:0x{:p}", add_display_s1, add_display_vtab_s1, &add_display_vtab_s1,&add_display_vtab_s1);

    println!("s1 debug trait object ptr:0x{:x}, vtable:0x{:x}", &add_debug_s1, add_debug_vtable_s1);

    println!("s2 dispaly trait object ptr:0x{:x}, vtable:0x{:x}", add_display_s2,add_display_vtab_s2);

    println!("s2 debug trait object ptr:0x{:x}, vtable:0x{:x}", add_debug_s2,add_debug_vtable_s2);

    //debug 和 display对象s1的地址应该相同
    assert_eq!(add_display_s1,add_debug_s1);

    //debug 和 display对象s2的地址应该相同
    assert_eq!(add_display_s2,add_debug_s2);

    // display的vtable，s1 和 s2的应该相同
    assert_eq!(add_display_vtab_s1,add_display_vtab_s2);

    // debug的vtable，s1 和 s2的应该相同
    assert_eq!(add_debug_vtable_s1,add_debug_vtable_s2);


}







