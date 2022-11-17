

// 参考： https://micouy.github.io/rust-dereferencing/

/*

*/

struct Thing {
    field: String,
}

// Compiles. Straight and simple.
fn f1(thing: &Thing) -> &String {
    &thing.field
    // 你可以访问一个结构体的字段，获得一个对该字段的引用，然后返回该引用。
}

// Doesn't compile...
fn f2(thing: &Thing) -> &String {
    let tmp = thing.field;
    &tmp
    // 你可以访问一个结构体字段，然后把该字段内容移动出来(这不允许)，放入到局部本地tmp中
    // 然后返回该局部变量的引用（这是不允许的）
    // = 表示移动所有权
}

// Compiles?!
fn f3(thing: &Thing) -> &String {
    &(thing.field)
    // 跟f1是相同的，只是这里多了 (... ) 表示一个优先操作符，从中，我们可以看出 & 是作用在field上的。
}



// Doesn't compile...
fn g1(thing: &Thing) -> &String {
    let tmp = *thing;
    &tmp.field
    // 你解引用 thing，把它移动到本地局部变量(这不允许)， 然后返回一个指向本地变量的引用(这不允许)
}

// Compiles?!
fn g2(thing: &Thing) -> &String {
    &(*thing).field
    // 这个跟 f1 f3 是一样的，只不过这里是显示的解引用。然后访问结构体内的字段，返回一个引用。
}






/*
// Doesn't compile.
fn g1(thing: &Thing) -> &String {
    let tmp = *thing;
//            ┗━ Move data out of `thing`.   这个解释似乎是不对的的，* 只是解引用。 = 才是移动

    &tmp.field
}

// Compiles.
fn g2(thing: &Thing) -> &String {
    &(*thing).field
//  ┃ ┗━ Move data out of `thing`.   一个模糊的概念(vague idea)说：  * 解引用 是顺着指针(引用&)的路一路找下去，就找到了真实的值，然后努力把它从那个地址处移出来。
//  ┗━━━ No actually cancel that.                                         而 &*variable，在解引用前面放或包裹几个引用 就可以抵消这个路线。
//                                                                                这个是不对的
}
*/


/*
// Doesn't compile.
fn g1(thing: &Thing) -> &String {
    let tmp = *thing;
//          ┃ ┗━ Point directly to the referenced data.
//          ┗━━━ Try to copy RHS's value, otherwise move it into `tmp`.

    &tmp.field
}

// Compiles.
fn g2(thing: &Thing) -> &String {
    &(*thing).field
//  ┃ ┗━ Point directly to the referenced data.
//  ┗━━━ Create a reference to the expression's value with a matching lifetime.
}
*/



/*
#[repr(C)]
pub struct MyStruct {
    first: u32,
    second: u32,
}

pub fn f1(s: &MyStruct) -> u32 {
    s.second   // 返回值的操作，从汇编来看，是把对应位置的[rdi+4]的 32 bits值拷贝到 eax 寄存器
}

pub fn f2(s: &MyStruct) -> &u32 {
    &s.second   // 返回引用，dereference-then-reference 的操作，只是一些基本的地址加减操作。 把rdi的值放到rax， rax加4，返回就是 结果。
}

汇编：

example::f1:
        mov     eax, dword ptr [rdi + 4] # Move the dword (32 bits) found at [rdi+4] to eax
        ret                              # Return from function

example::f2:
        mov     rax, rdi                 # Move rdi to rax
        add     rax, 4                   # Increment rax by 4
        ret                              # Return from function



*/








fn main(){}