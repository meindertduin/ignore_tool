use ignore::{create_new_ingore, write_existing_ignore,};
use clap::{App, Arg, ArgMatches};

fn main() {
    let matches = App::new("ignore")
        .version("0.1")
        .author("Meindert v D. <meindertvanduin99@gmail.com>")
        .about("simple cli for creating quick ignore files")
        .arg(Arg::with_name("create")
                .short("c")
                .multiple(true)
                .long("create")
                .value_name("IGNORE TYPE")
                .help("creates new ignore file with privided ignore types")
                .takes_value(true))
        .arg(Arg::with_name("write")
                .short("w")
                .multiple(true)
                .long("write")
                .value_name("IGNORE TYPE")
                .help("writes to the given ignore file with provided ignore types")
                .takes_value(true))
        .arg(Arg::with_name("path")
                .short("p")
                .long("path")
                .value_name("PATH")
                .help("the path where the ignore file should be writen to or created, defaults to current directory")
                .takes_value(true))
        .get_matches();

    run(matches);
}


fn run(matches: ArgMatches) { 
    let path = match matches.value_of("path") {
        Some(path) => path,
        None => ".",
    };
    
    if let Some(types) = matches.values_of("create") {
        create_new_ingore(path, types);
    } else if let Some(types) = matches.values_of("write") {
        write_existing_ignore(path, types);
    }    
    
}
