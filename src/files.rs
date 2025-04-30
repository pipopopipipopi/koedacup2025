use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader};

pub fn read_data(path: &str) -> Result<Vec<(f64, f64)>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        let x: f64 = parts[0].parse()?;
        let y: f64 = parts[1].parse()?;
        data.push((x, y));
    }

    Ok(data)
}

pub fn read_predict_data(path: &str) -> Result<Vec<f64>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let x: f64 = line.parse()?;
        data.push(x);
    }

    Ok(data)
}
