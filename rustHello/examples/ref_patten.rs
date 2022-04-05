


#[derive(Clone, Debug)]
struct Point { x: String, y: String }


/// ref
pub fn main() {

    let c = 'Q';

    // 在赋值语句左侧的 ref 的借用 与 赋值语句右侧的 & 的引用是等效的。
    let ref ref_c1 = c;
    let ref_c2 =&c;
    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let p =Point{x: "32".into(), y:"y".into()};

    let _copy_x ={
        // ref 在解构中也是可以使用的
        let Point {x: ref xx,y:_} =p;
        ""
    };

    let mut mut_point = p;
    println!("mut point:{:?}",mut_point);

    {
        // ref mut 解构一个可变的结构体
        let Point{x:ref mut mut_x, y:_} =mut_point;
        *mut_x+=" 55";
    }

    println!("mut point:{:?}",mut_point);




}



