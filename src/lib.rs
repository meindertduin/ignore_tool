use std::{fs, fs::File ,io::Write};
use clap::Values;

mod ignore_client;

pub fn create_new_ingore(path: &str, args: Values){ 
    let mut ignore_file = fs::File::create("dev.txt").unwrap();
    
    write_ingore_data(&mut ignore_file, args);
}

pub fn write_existing_ignore(path: &str, types: Values){
    let mut ignore_file = File::open(path).unwrap();
    write_ingore_data(&mut ignore_file, types)
}


fn write_ingore_data(mut ignore_file: &File, types: Values){
    let mut client = ignore_client::IgnoreFilesClient::new();
    
    let client_data = get_ignore_client_data(types);

    for data in client_data {
        client.add_ignore(data);
    }

    let ignore_request_handlers = client.send_requests();
    
    for handler in ignore_request_handlers {  
        match handler.response_code() {
            Ok(code) => {
                if code == 200 {
                    let collector = handler.get_ref();
                    
                    let ignore_header = format!("<--------------------{}-------------------->\r\n", 
                        collector.ignore_type.as_str());

                    ignore_file.write(ignore_header.as_bytes()).unwrap();
                    ignore_file.write(&collector.buffer).unwrap();
                } else{
                    panic!("ignore of type {} could not be found", handler.get_ref().ignore_type)
                }
            },
            Err(err) => panic!("{:?}", err),
        };
    }
     
}


fn get_ignore_client_data(types: Values) -> Vec<ignore_client::IgnoreClientData> {
    let mut client_data: Vec<ignore_client::IgnoreClientData> = Vec::new();

    let base_ignore_url = "https://raw.githubusercontent.com/github/gitignore/master/";

    for ignore in types 
    {
        let cap_ingore_type = ignore_client::cap_first_char(ignore);
        
        client_data.push(ignore_client::IgnoreClientData {
            url: format!("{}{}{}", base_ignore_url, cap_ingore_type.as_str(), ".gitignore"),
            ignore_type: cap_ingore_type,
        });
    } 

    client_data
}












