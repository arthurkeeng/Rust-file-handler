use std::{error::Error, fs::File, io::prelude::*};

pub struct CommanLineArg {
    pub query: String,
    pub filename: String,
}

impl CommanLineArg {
    pub fn new(args: &[String]) -> Result<CommanLineArg, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        };
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(CommanLineArg { query, filename })
    }
}
pub fn run(arguments: CommanLineArg) -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();
    let mut file = File::open(arguments.filename)?;

    file.read_to_string(&mut contents)?;

    Ok(())
}
