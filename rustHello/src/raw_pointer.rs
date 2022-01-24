



pub fn run(){
    let x = 1;
    let mut y = 4;;
    let x_raw = &x as *const i32;
    let y_mut_raw = &mut y as *mut i32;

    println!("x_raw pointer:{:?}",x_raw);
    println!("y_mut_raw pointer:{:?}", y_mut_raw);

}

















