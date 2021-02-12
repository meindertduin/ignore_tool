use std::{fs, fs::File ,io::Write};
use clap::Values;
use fs::OpenOptions;

mod ignore_client;

pub fn create_new_ingore(path: &str, args: Values){
    let file_name = ".gitignore";
    let mut ignore_file = fs::File::create(format!("{}/{}", path, file_name)).unwrap();

    
    write_ingore_data(&mut ignore_file, args);
}

pub fn write_existing_ignore(path: &str, types: Values){
    let mut ignore_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    write_ingore_data(&mut ignore_file, types)
}


fn write_ingore_data(mut ignore_file: &File, types: Values){
    let mut client = ignore_client::IgnoreFilesClient::new();
    
    let client_data = ignore_client::get_ignore_client_data(types);

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

                    ignore_file.write_all(ignore_header.as_bytes()).unwrap();
                    ignore_file.write_all(&collector.buffer).unwrap();
                } else{
                    panic!("ignore of type {} could not be found", handler.get_ref().ignore_type)
                }
            },
            Err(err) => panic!("{:?}", err),
        };
    }     
}














