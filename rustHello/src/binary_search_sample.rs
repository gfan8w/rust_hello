






pub fn run() {
    // https://stackoverflow.com/questions/37804973/why-does-slicebinary-search-return-an-incorrect-result

    let mut xs = vec!["slot", "s"];
    let u = xs.binary_search(&"slot");
    println!("{:?}", u);

    let slot ="slot";
    let index = xs.iter().position(|x| *x == slot).unwrap();
    let rmv =xs.remove(index);
    println!("remove:{:?}, collection:{:?}", rmv, xs);

    let slot ="slot".to_string();
    let mut xs = vec!["slot".to_string(), "s".to_string()];
     xs.retain(|v| *v !=slot);
    let mut b =slot;
    b+="a";
    println!("b:{:?}",b);
    getB(b);        //这里要演示一下 slot： String 会不会移动，看起来好像没有移动
    println!("collection:{:?}", xs);
}

fn getB(b: String){

}

