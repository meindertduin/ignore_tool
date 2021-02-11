use curl::multi::Multi;
use curl::{easy::{Easy, Easy2, Handler, WriteError}, multi::Easy2Handle};
use std::{fmt::{self, Display}, time::Duration};

pub struct IgnoreFilesClient {
    multi: Multi,
    easy_handles: Vec<Easy2Handle<EasyCollector>>,
}

impl IgnoreFilesClient {
    pub fn new() -> IgnoreFilesClient {
        let multi = Multi::new();

        let mut multi = Multi::new();
        multi.pipelining(true, true).unwrap();         

        IgnoreFilesClient {
            multi,
            easy_handles: Vec::new(),
        }
    }

    pub fn add_ignore(&mut self, ignore_data: IgnoreClientData){
        let mut easy = Easy2::new(EasyCollector::new(ignore_data.ignore_type));
        easy.url(ignore_data.url.as_str()).unwrap();
        easy.get(true).unwrap(); 
        let easy_handle = self.multi.add2(easy).unwrap();
        self.easy_handles.push(easy_handle);
    }

    pub fn send_requests(&mut self) -> &mut Vec<Easy2Handle<EasyCollector>>{
        while self.multi.perform().unwrap() > 0 {
            self.multi.wait(&mut [], Duration::from_secs(1)).unwrap();
        }

        self.easy_handles.as_mut() 
    }
}



pub struct EasyCollector {
    pub buffer: Vec<u8>,
    pub ignore_type: String,
}

impl Handler for EasyCollector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.buffer.extend_from_slice(data);
        Ok(data.len())
    }
}

impl EasyCollector {
    fn new(ignore_type: String) -> EasyCollector {
        EasyCollector {
            buffer: Vec::new(),
            ignore_type,
        }
    }
}

pub struct IgnoreClientData {
    pub url: String,
    pub ignore_type: String,
}

pub fn cap_first_char(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
