use std::sync::Arc;
use rocksdb::DB;


pub trait KVStore {
    fn init(file_path: &str) -> Self;
    fn save(&self, key: &str, value: &str) -> bool;
    fn find(&self, key: &str) -> Option<String>;
    fn delete(&self, key: &str) -> bool;
}


#[derive(Clone)]
pub struct RocksDb {
    //Actix-web的router里会使用这个db，但是DB对象没有实现clone，所以我们用Arc包裹它，让它可以在多线程里安全的使用
    db: Arc<DB>,
}


impl KVStore for RocksDb {
    fn init(file_path: &str) -> Self {
        RocksDb {db: Arc::new(DB::open_default(file_path).unwrap())}
    }

    fn save(&self, key: &str, value: &str) -> bool {
        let a: &[u8] = key.as_bytes().as_ref();
        self.db.put(a,value.as_bytes()).is_ok()
    }

    fn find(&self, key: &str) -> Option<String> {
        match self.db.get(key.as_bytes()) {
            Ok(Some(v)) =>{
                let mut result=String::from("");
                unsafe {
                     result= String::from_utf8_unchecked(v);
                }
                println!("retrieve key:{}, result:{}",key,result);
                Some(result)
            },
            Ok(None) =>{
                println!("retrieve key:{}, result:None",key);
                None
            },
            Err(e) => {
                println!("retrieve key:{}, error:{}",key,e);
                None
            }
        }
    }

    fn delete(&self, key: &str) -> bool {
        self.db.delete(key.as_bytes()).is_ok()
    }
}












