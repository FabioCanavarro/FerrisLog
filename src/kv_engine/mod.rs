use std::error::Error;

pub trait KvEngine{
    fn get(&self, key: String) -> Result<Option<String>,Box<dyn Error>>;
    fn set(&self, key: String, val: String) -> Result<(),Box<dyn Error>>;
    fn remove(&self, key: String) -> Result<(),Box<dyn Error>>;
}
