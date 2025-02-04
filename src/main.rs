use std::{env, process::exit};

pub mod args;
mod core;
mod password;
mod util;

const VERSION: f64 = 0.1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: args::Config = args::parse_arguments(&args).unwrap_or_else(|| {
        util::banner(&VERSION);
        exit(1);
        //args::Config::default()
    });
    // println!("{:?}", config);
    let res: Result<String, String> = core::process_args(&config);
    match res {
        Ok(valv) => println!("[+]: {}", valv),
        Err(err) => println!("ERROR: {}", err),
    }
    //println!("{:?}", config);
    //banner(); // show the banner of this program
}
