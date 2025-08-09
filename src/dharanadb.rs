use std::io::{BufReader, Write};
use std::{collections::HashMap, fs::File};

use serde_derive::{Deserialize, Serialize};
use std::io::BufRead;

use crate::errors::{DharanaError, SingleResult};

pub struct DharanaStore {
    map: HashMap<String, String>,
    file: File,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum CommandType {
    Get { key: String },
    Set { key: String, value: String },
    Remove { key: String },
}

#[derive(Serialize, Deserialize, Debug)]
struct Command {
    c: CommandType,
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Log {
    list: Vec<Command>,
}

impl DharanaStore {
    pub fn new() -> Result<DharanaStore, DharanaError> {
        let file = File::options()
            .create(true)
            .append(true)
            .read(true)
            .open("db.txt")?;

        let f_size = file.metadata()?.len();
        println!("File size {}", f_size);

        let mut obj = DharanaStore {
            map: HashMap::new(),
            file,
        };

        obj.read()?;

        Ok(obj)
    }

    fn read(&mut self) -> Result<bool, DharanaError> {
        let file = File::options()
            .create(true)
            .write(true)
            .read(true)
            .open("db.txt")?;
        // let mut buf : [u8];
        // file.read(buf);
        let buf_reader = BufReader::new(file);

        for line in buf_reader.lines() {
            let line_str = line?;
            let document: CommandType = serde_json::from_str(&line_str)?;

            match document {
                CommandType::Set { key, value } => {
                    self.map.insert(key, value);
                }
                CommandType::Remove { key } => {
                    self.map.remove(&key);
                }
                _ => {}
            }
        }

        Ok(true)
    }

    fn write(&mut self, command: CommandType) -> Result<bool, DharanaError> {
        let j = serde_json::to_string(&command)?;

        writeln!(self.file, "{}", j)?;

        Ok(true)
    }

    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, value: String) -> Result<bool, DharanaError> {
        self.map.insert(key.clone(), value.clone());
        let command = CommandType::Set { key, value };
        self.write(command)?;
        Ok(true)
    }

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&self, key: String) -> SingleResult<Option<String>> {
        match self.map.get(&key) {
            Some(val) => Ok(Some(val.to_owned())),
            None => Ok(None),
        }
    }

    /// Remove a given key.
    pub fn remove(&mut self, key: String) -> Result<bool, DharanaError> {
        self.map.remove(&key);

        self.write(CommandType::Remove { key: key })?;

        Ok(true)
    }
}
