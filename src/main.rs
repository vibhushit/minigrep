use std::env;
use std::process;

use minigrep::Config;


fn main() {
    let args : Vec<String> = env::args().collect();
    //dbg!(args);


    // we want to check if there is value , if not use the err in anonymous function and print it.
    let config =  Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //we only want to check that run(config) return an Err then execute, because in success case it return (). unit type
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error:  {e}");
        process::exit(1);
    }
}



