mod behaviour;
mod braille;

use std::env;
use behaviour::{interactive, args};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() <= 1 {
        interactive::perform();
    }
    else {
        args::perform(&args);
    }
}

