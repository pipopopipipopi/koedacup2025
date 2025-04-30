use std::env;
use std::error::Error;

mod files;
use crate::files::{read_data, read_predict_data};

mod lsm;
use crate::lsm::{lsm, predict_all};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let train_file = &args[1];
    let train_data = read_data(&train_file)?;

    let predict_file = &args[2];
    let predict_data = read_predict_data(&predict_file)?;

    let degree: &usize = &args[3].parse()?;

    let c = lsm(&train_data, *degree)?;
    let predict = predict_all(&predict_data, &c);

    eprintln!("{:?}", train_data);
    eprintln!("{:?}", lsm(&train_data, *degree));
    eprintln!("{:?}", predict);
    for i in 0..predict_data.len() {
        println!("{}, {}", predict_data[i], predict[i]);
    }
    Ok(())
}
