use std::{
    fs::File,
    io,
    io::BufRead,
    path::Path
};

pub mod wrong;
pub mod correct;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}