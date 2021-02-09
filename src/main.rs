use std::env;
use ignore::create_new_ingore;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mode = &args[1];
    let path = &args[2];

    let args = Vec::from(&args[3..]);
    
    let config = Config::parse(mode, path,&args);

    if config.args.len() <= 0 {
        panic!("to few arguments given");    
    };

    run(&config);
}

fn run(config: &Config){
    match config.mode.as_str() {
        "create" => create_new_ingore(config.path, config.args),
        _ => panic!("please enter a valid mode"),
    }
}

struct Config <'a>{
    mode: &'a String,
    path: &'a String,
    args: &'a Vec<String>, 
}

impl<'a> Config<'a> {
    fn parse(mode: &'a String, path: &'a String,args: &'a Vec<String>) -> Config<'a>
    {
        Config {
            mode,
            path,
            args,
        }
    }
}
