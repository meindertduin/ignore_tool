use curl::{easy::{Easy, Easy2, Handler, WriteError}, multi::Easy2Handle};
use curl::multi::Multi;
use fmt::write;
use std::{fmt::{self, Display}, time::Duration};


struct EasyCollector {
    buffer: Vec<u8>,
    ignore_type: IgnoreType,
}

impl Handler for EasyCollector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.buffer.extend_from_slice(data);
        Ok(data.len())
    }
}

impl EasyCollector {
    fn new(ignore_type: IgnoreType) -> EasyCollector {
        EasyCollector {
            buffer: Vec::new(),
            ignore_type,
        }
    }
}

struct IgnoreFilesClient {
    multi: Multi,
    easy_handles: Vec<Easy2Handle<EasyCollector>>,
}

impl IgnoreFilesClient {
    fn new() -> IgnoreFilesClient {
        let multi = Multi::new();

        let mut multi = Multi::new();
        multi.pipelining(true, true).unwrap();         

        IgnoreFilesClient {
            multi,
            easy_handles: Vec::new(),
        }
    }

    fn add_ignore(&mut self, url: &str, ignore_type: IgnoreType){
        let mut easy = Easy2::new(EasyCollector::new(ignore_type));
        easy.url(url).unwrap();
        easy.get(true).unwrap(); 
        let easy_handle = self.multi.add2(easy).unwrap();
        self.easy_handles.push(easy_handle);
    }

    fn send_requests(&self) -> &Vec<Easy2Handle<EasyCollector>>{
       while self.multi.perform().unwrap() > 0 {
            self.multi.wait(&mut [], Duration::from_secs(1)).unwrap();
       } 

       self.easy_handles.as_ref() 
    }
}

#[derive(Debug)]
enum IgnoreType {
    Rust,
    C,
}

impl Display for IgnoreType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn create_new_ingore(path: &String, args: &Vec<String>){ 
    let mut client = IgnoreFilesClient::new();
    client.add_ignore("https://raw.githubusercontent.com/github/gitignore/master/Rust.gitignore", IgnoreType::Rust);
    client.add_ignore("https://raw.githubusercontent.com/github/gitignore/master/C.gitignore", IgnoreType::C);
    let ignore_request_handlers = client.send_requests();
    

    for handler in ignore_request_handlers {
        let message = String::from_utf8_lossy(&handler.get_ref().buffer);
        println!("Handler === {}", handler.get_ref().ignore_type);
        println!("{}", message);
    }

}





