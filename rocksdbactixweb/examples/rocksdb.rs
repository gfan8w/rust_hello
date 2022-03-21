use rocksdb::{DB, Direction, IteratorMode};

// 详细介绍：https://www.daimajiaoliu.com/daima/60eb18b0119ec07

pub fn main () {
    //put_data();
    iter_all();
}

fn put_data() {
    let path ="./data";
    // db is automatically closed at end of lifetime
    {
        let db = DB::open_default(path).unwrap();
        let k =b"ppg";
        db.put(k,b"ppg sandy").unwrap();
        match db.get(k) {
            Ok(Some(v)) => {
                println!("retrieved value:{}", String::from_utf8(v).unwrap())
            },
            Ok(None) => {
                println!("value not found")
            },
            Err(e) => {
                println!("error:{}",e)
            }
        }
    }
}


fn iter_all(){

    let path ="/Users/mmac/rdb/var/folders/cm/xl4yx08j4zq_nj6gq0bhqwqw0000gn/T/substrateQwUrS5/chains/dev/db/full";
    //let path ="./data";
    {
        let mut db = DB::open_default(path).unwrap();

        let mut iter = db.iterator(IteratorMode::Start); // Always iterates forward
        for (key, value) in iter {
            println!("Saw {:?} {:?}", String::from_utf8(key.to_vec()).unwrap(), String::from_utf8(value.to_vec()).unwrap());
        }
        /*iter = db.iterator(IteratorMode::End);  // Always iterates backward
        for (key, value) in iter {
            println!("Saw {:?} {:?}", key, value);
        }
        iter = db.iterator(IteratorMode::From(b"my key", Direction::Forward));
        // From a key in Direction::{forward,reverse}
        for (key, value) in iter {
            println!("Saw {:?} {:?}", key, value);
        }

        // You can seek with an existing Iterator instance, too
        iter = db.iterator(IteratorMode::Start);
        iter.set_mode(IteratorMode::From(b"another key", Direction::Reverse));
        for (key, value) in iter {
            println!("Saw {:?} {:?}", key, value);
        }*/
    }
}













