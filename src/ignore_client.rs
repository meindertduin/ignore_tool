use curl::multi::Multi;
use curl::{easy::{Easy, Easy2, Handler, WriteError}, multi::Easy2Handle};
use std::{fmt::{self, Display}, time::Duration};
use clap::Values;

pub fn get_ignore_client_data(types: Values) -> Vec<IgnoreClientData> {
    let mut client_data: Vec<IgnoreClientData> = Vec::new();

    let base_ignore_url = "https://raw.githubusercontent.com/github/gitignore/master/";

    for ignore in types 
    {
        let cap_ingore_type = cap_first_char(ignore); 
        if let Some(url) = get_raw_gitignore_url(ignore.to_uppercase().as_str()){
            client_data.push(IgnoreClientData {
                url, 
                ignore_type: cap_ingore_type,
            });
        } else {
            // let the user know that the package could not be found
            eprintln!("Error: ignore file of type {} could not be found and has not been added to .gitignore", ignore);
        }
    } 

    client_data
}


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

fn get_raw_gitignore_url(ignore_type: &str) -> Option<String> {
    let root_url = "https://raw.githubusercontent.com/github/gitignore/master/";

    let route = match ignore_type {
        "C" => Some("C.gitignore"),
        "C++" => Some("C++.gitignore"),
        "LUA" => Some("Lua.gitignore"),
        "Go" => Some("Go.gitignoe"),
        "NODE" => Some("Node.gitignore"),
        "OBJECTIVE-C" => Some("Objective-C.gitignore"),
        "PYTHON" => Some("Pyton.gitignore"),
        "RUST" => Some("Rust.gitignore"),
        "LINUX" => Some("community/Linux/Snap.gitignore"),
        "VUE" => Some("community/JavaScript/Vue.gitignore"),
        "JETBRAINS" => Some("Global/JetBrains.gitignore"),
        "VIM" => Some("Global/Vim.gitignore"),
        "VSCODE" => Some("VisualStudioCode.gitignore"),
        "VISUALSTUDIO" => Some("VisualStudio.gitignore"),
        _ => None,
    };

    match route {
        Some(value) => Some(format!("{}{}", root_url, value)),
        None => None,
    }
}
