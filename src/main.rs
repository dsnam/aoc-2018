mod day_one;

use std::env;
use std::io::Error;

// run with args <day number> <inputfile>. input files go in the project root
fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let filename = &args[2];

    match day.as_ref() {
        "1" => { day_one::run(filename)?; },
        _ => { println!("Invalid day"); },
    }

    Ok(())
}
