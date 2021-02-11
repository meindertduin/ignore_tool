
use std::{fs, io::Write};

mod ignore_client;

pub fn create_new_ingore(path: &String, args: &Vec<String>){ 
    let mut client = ignore_client::IgnoreFilesClient::new();

    let client_data = get_ignore_client_data(args);

    for data in client_data {
        client.add_ignore(data);
    }

    let ignore_request_handlers = client.send_requests();
    
    let mut ignore_file = fs::File::create("dev.txt").unwrap();


    for handler in ignore_request_handlers {  
        match handler.response_code() {
            Ok(code) => {
                if code == 200 {
                    ignore_file.write(&handler.get_ref().buffer).unwrap();
                } else{
                    panic!("ignore of type {} could not be found", handler.get_ref().ignore_type)
                }
            },
            Err(err) => panic!("{:?}", err),
        };
    }

}


fn get_ignore_client_data(types: &Vec<String>) -> Vec<ignore_client::IgnoreClientData> {
    let mut client_data: Vec<ignore_client::IgnoreClientData> = Vec::new();

    let base_ignore_url = "https://raw.githubusercontent.com/github/gitignore/master/";

    for ignore in types 
    {
        let cap_ingore_type = ignore_client::cap_first_char(ignore.as_str());
        
        client_data.push(ignore_client::IgnoreClientData {
            url: format!("{}{}{}", base_ignore_url, cap_ingore_type.as_str(), ".gitignore"),
            ignore_type: cap_ingore_type,
        });
    } 

    client_data
}












