use std::{collections::HashMap};
use std::{fs, fs::File};

pub struct DB {
    base: HashMap<Vec<String>, Vec<String>>,
}

impl DB
{    
    pub fn new() -> Self {
        DB {
            base: HashMap::new(),
        }
    }
    pub fn from_str(str: &str, primary_key_length: usize, delimiter: &str) -> Self {
        let lines: Vec<&str> = str.split("\n").collect();
        let mut db = Self::new();
        for line in lines {
            let keys_values: Vec<&str> = line.split(delimiter).collect();
            if keys_values.len() < primary_key_length {
                break;
            }
            let keys_values: Vec<String> = keys_values.iter().map(|&str| str.to_string()).collect();
            db.insert(keys_values[0..primary_key_length].to_vec(), keys_values[primary_key_length..].to_vec());
        }
        db
    }
    pub fn to_string(&self, delimiter: &str) -> String {
        let mut buf = String::new();
        for (k, v) in self.base.iter() {
            buf += &k.join(&delimiter);
            buf += &delimiter;
            buf += &v.join(&delimiter);
            buf += "\n";
        }
        buf.trim().to_string()
    }
    pub fn from_file(path: &str, primary_key_length: usize, delimiter: &str) -> Self {
        match File::open(path) {
            Err(_) => {
                File::create(path).unwrap();
            }
            _ => ()
        }
        let string = fs::read_to_string(path).expect("Unable to read file");
        DB::from_str(&string.trim(), primary_key_length, delimiter)
    }
    pub fn to_file(&self, path: &str, delimiter: &str) -> (){
        let string = self.to_string(delimiter);
        fs::write(path, string).expect("Unable to write file");
    }
    pub fn get<T>(&self, k: &Vec<T>) -> Option<&Vec<String>>
    where
        T: ToString,
    {
        let k: Vec<String> = k.iter().map(|x| x.to_string()).collect();
        self.base.get(&k)
    }
    pub fn insert<T1, T2>(&mut self, k: Vec<T1>, v: Vec<T2>) -> Option<Vec<String>>
    where
        T1: ToString,
        T2: ToString
    {
        let k = k.iter().map(|x| x.to_string()).collect();
        let v = v.iter().map(|x| x.to_string()).collect();
        self.base.insert(k, v)
    }
    pub fn contains_key<T>(&self, k: &Vec<T>) -> bool
    where
        T: ToString,
    {
        let k: Vec<String> = k.iter().map(|x| x.to_string()).collect();
        self.base.contains_key(&k)
    }
}