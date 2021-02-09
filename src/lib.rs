use curl::easy::{Easy, Easy2, Handler, WriteError};
use curl::multi::Multi;
use std::time::Duration;


struct EasyCollector {
    buffer: Vec<u8>,
}

impl Handler for EasyCollector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.buffer.extend_from_slice(data);
        Ok(data.len())
    }
}

impl EasyCollector {
    fn new() -> EasyCollector {
        EasyCollector {
            buffer: Vec::new(),
        }
    }
}

pub fn create_new_ingore(path: &String, args: &Vec<String>){ 
    let mut rust_handle = Easy2::new(EasyCollector::new());
    rust_handle.url("https://raw.githubusercontent.com/github/gitignore/master/Rust.gitignore").unwrap();
    rust_handle.get(true).unwrap();

    let mut c_handle = Easy::new();
    c_handle.url("https://raw.githubusercontent.com/github/gitignore/master/C%2B%2B.gitignore").unwrap();

    let mut multi = Multi::new();
    multi.pipelining(true, true).unwrap();
    let rust_handle = multi.add2(rust_handle).unwrap();

    while multi.perform().unwrap() > 0 {
        multi.wait(&mut [], Duration::from_secs(1)).unwrap();
    }

    let handler = rust_handle.get_ref();
    
    println!("{}",message);
}




