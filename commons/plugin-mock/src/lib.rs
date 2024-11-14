#[macro_use]
extern crate lazy_static;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

type KVStore = BTreeMap<String, i64>;

lazy_static! {
    static ref kv_store: Arc<RwLock<KVStore>> = Arc::new(RwLock::new(KVStore::default()));
}

pub fn kv_read(key: String) -> Result<i64, extism_pdk::Error> {
    println!("kv_read...: {}=", &key);
    loop {
        match kv_store.read() {
            Ok(kv) => {
                let value = kv.get(&key).map(|x| x.clone()).unwrap_or_else(|| 0i64);
                println!("kv_read over: {}={}", &key, value);
                return Ok(value);
            }
            Err(_e) => {
                std::thread::sleep(Duration::from_millis(3));
                continue;
            }
        }
    }
}
pub fn kv_write(key: String, value: i64) -> Result<Option<i64>, extism_pdk::Error> {
    println!("kv_write...: {}=", &key);
    loop {
        match kv_store.write() {
            Ok(mut kv) => {
                let old = kv.insert(key.clone(), value);
                println!("kv_read over: {}={}", &key, value);
                return Ok(old);
            }
            Err(_e) => {
                std::thread::sleep(Duration::from_millis(3));
                continue;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{kv_read, kv_write};

    #[test]
    fn it_works() -> Result<(), extism_pdk::Error> {
        let mut x = kv_read("key".into())?;
        x += 1;
        kv_write("key".into(), x)?;
        let x = kv_read("key".into())?;
        assert_eq!(1, x);
        Ok(())
    }
}
