

pub fn run(){
    println!("demo how to do string concatenation!");

    let mut a ="hello".to_string();
    let b ="world".to_string();
    let cont =" ";
    let bb =&b;

    let mut c = String::new();
    c+=&a;
    c+=cont;
    c+= bb;
    println!("{}",c);

    a+=cont;
    a+= &b;

    println!("{}",a);





}




