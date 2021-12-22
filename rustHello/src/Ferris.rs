use std::io::{stdout, BufWriter};
use std::collections::BTreeMap;



#[cfg(ccmtest)]
pub const DAYS: u32 = 3 ;

#[cfg(not(ccmtest))]
pub const DAYS: u32 = 24 ;




pub fn run(){
    let message ="Hello I'am a rustacean!";
    let width =message.len();
    let stdout=stdout();

    let mut buf=BufWriter::new(stdout.lock());
    ferris_says::say(message.as_bytes(),width,&mut buf).unwrap();

    let v =vec![1,2,3,4,5];
    let vt =v.iter();
    let s=vt.sum::<i32>();
    let s1:Vec<_>=v.iter().map(|x| x+1).collect();
    println!("s1:{:?}",s1);

    println!("DAY:{:?}",DAYS)

}


#[test]
fn test2() {
    /* let mut event_vec = vec![];
     for (i,(x,y)) in  map.range(&4..).into_iter().enumerate(){
         println!("i:{},x{} :y{}",i,x,y);
         event_vec.push(y);
         map.remove(x);
         if i==3{
             break;
         }
     }

     println!("event_vec::{:?}:", event_vec);
     println!("map::{:?}:", map);*/


    let mut map = BTreeMap::new();
    map.insert(3, "a");
    map.insert(4, "b");
    map.insert(5, "c");
    map.insert(6, "e");
    map.insert(7, "f");
    map.insert(8, "g");


    let mut event_vec = vec![];

    for (i, (x, y)) in map.range(&4..).into_iter().enumerate() {
        println!("i:{},x{} :y{}", i, x, y);
        event_vec.push(y);
        if i == 3 {
            break;
        }
    }

    //map.remove(&7); // 放这里不行

    println!("event_vec::{:?}:", event_vec);
    map.remove(&7);
    println!("map::{:?}:", map);
}