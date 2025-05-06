use std::error::Error;
use crate::kvstore::{error::KvError, KvStore};



// NOTE: t{command} stands for KvEngine command
pub trait KvEngine{
    fn tget(&self, key: String) -> Result<Option<String>,Box<dyn Error>>;
    fn tset(&mut self, key: String, val: String) -> Result<(),Box<dyn Error>>;
    fn tremove(&mut self, key: String) -> Result<(),Box<dyn Error>>;
}

impl KvEngine for KvStore{
    fn tget(&self, key: String) -> Result<Option<String>,Box<dyn Error>> {
        Ok(self.get(key)?)
    }
    fn tset(&mut self, key: String, val: String) -> Result<(),Box<dyn Error>> {
        Ok(self.set(key, val)?)
    }
    fn tremove(&mut self, key: String) -> Result<(),Box<dyn Error>> {
        Ok(self.remove(key)?)
    }
}

impl KvEngine for sled::Db{
    fn tremove(&mut self, key: String) -> Result<(),Box<dyn Error>> {
        self.remove(key.as_bytes())?;
        Ok(())
    }
    fn tset(&mut self, key: String, val: String) -> Result<(),Box<dyn Error>> {
        self.insert(key.as_bytes(), val.as_bytes())?;
        Ok(())
    }
    fn tget(&self, key: String) -> Result<Option<String>,Box<dyn Error>> {
        let val = self.get(key.as_bytes())?;
        match val{
            Some(val) => {
                Ok(
                    Some(
                        String::from_utf8_lossy(&val.to_vec()[..]).to_string()
                    )
                )
            },
            None => Err(Box::new(KvError::EngineError))
        }
        
    }
}

