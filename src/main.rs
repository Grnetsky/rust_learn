use std::{env,process};
use minigrep::Config;

fn main() {
    let args:Vec<String> = env::args().collect();
    let config= Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments:{}",err);
        process::exit(1);
    });
   if let Err(_e) =  minigrep::run(config){
       // eprintln!("Application Error:{}",e);
       process::exit(1)
   };
}
