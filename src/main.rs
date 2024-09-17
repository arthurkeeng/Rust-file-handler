extern crate helloWork;

use helloWork::CommanLineArg;
use std::{env, process};
fn main() {
    let args: Vec<String> = env::args().collect();

    let arguments = CommanLineArg::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments : {}", err);
        process::exit(1);
    });

    if let Err(e) = helloWork::run(arguments) {
        println!("application error : {} ", e);
        process::exit(1);
    };
}
