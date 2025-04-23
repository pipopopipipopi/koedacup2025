use std::env;
use std::error::Error;

mod files;
use crate::files::read_data;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let train_file = &args[1];
    let train_data = read_data(&train_file)?;

    println!("{:?}", train_data);
    Ok(())
}
