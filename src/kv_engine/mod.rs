use std::error::Error;
use std::{
    fs::{self, File}, io::{BufRead, BufReader, Seek, SeekFrom, Write}
};
use command::Command;
use error::KvError;
use crate::kvstore::error;
use crate::kvstore::command;
use crate::kvstore::KvStore;

const COMPACTION_THRESHOLD: u64 = 1024;

pub trait KvEngine{
    fn get(&self, key: String) -> Result<Option<String>,Box<dyn Error>>;
    fn set(&mut self, key: String, val: String) -> Result<(),Box<dyn Error>>;
    fn remove(&mut self, key: String) -> Result<(),Box<dyn Error>>;
}

impl KvEngine for KvStore{
    fn set(&mut self, key: String, val: String) -> Result<(), Box<dyn Error>> {
        let cmd = Command::set(key.clone(), val.clone());

        let mut f = File::options()
            .read(true)
            .append(true)
            .open(&self.path)
            .unwrap();

        let start_pos = f.seek(SeekFrom::End(0)).unwrap();
        let _ = serde_json::to_writer(&mut f, &cmd);
        let _ = f.write_all(b"\n");
        self.table.insert(key, start_pos);

        let size = fs::metadata(&self.path);

        let length = size.unwrap().len();

        if length > COMPACTION_THRESHOLD {
            let _ = self.compaction();
        }

        Ok(())
    }

    fn get(&self, key: String) -> Result<Option<String>,Box<dyn Error>> {
        let val = self.table.get(&key);
        match &val {
            Some(_) => (),
            None => return Ok(None),
        }

        let file = File::options().read(true).open(&self.path).unwrap();

        let mut f = BufReader::new(file);

        // Seek from val to the \n
        let _ = f.seek(SeekFrom::Start(*val.unwrap()));
        let mut line = String::new();
        let _ = f.read_line(&mut line);
        let res = serde_json::from_str::<Command>(&line.to_string());
        match res {
            Ok(re) => match re {
                Command::Set { key: _, val } => Ok(Some(val)),
                _ => Ok(None),
            },
            Err(_) => Err(Box::new(KvError::ParseError)),
        }
    }

    fn remove(&mut self, key: String) -> Result<(),Box<dyn Error>> {
        let cmd = Command::rm(key.clone());

        let mut f = File::options()
            .read(true)
            .append(true)
            .open(&self.path)
            .unwrap();

        let _ = serde_json::to_writer(&mut f, &cmd);
        let _ = f.write_all(b"\n");
        match self.table.remove(&key) {
            Some(_) => Ok(()),
            None => Err(Box::new(KvError::RemoveError)),
        }
    }
}
