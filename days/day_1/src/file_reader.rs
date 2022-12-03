use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use crate::inventory_counter::Reader;
use crate::error::Result;

pub struct FileReader {
    file_path: String
}

impl FileReader {
    pub fn new(file_path: String) -> Self {
        FileReader { file_path }
    }
}

impl Reader for FileReader {
    fn read_inventory(&self) -> Result<Vec<Vec<u32>>> {
        let (_, elves) = read_lines(&self.file_path)
            .map_err(error_to_string)?
            .map(|maybe_e| maybe_e.map_err(error_to_string))
            .map(|ip| ip.ok().and_then(|val| val.parse::<u32>().ok()))
            .fold((Vec::new(), Vec::new()), |(mut elf, mut elves), maybe_cals| {
                if let Some(cals) = maybe_cals {
                    elf.push(cals);
                } else {
                    elves.push(elf);
                    elf = Vec::new();
                }
                (elf, elves)
            });
        Ok(elves)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn error_to_string<E: Debug>(e: E) -> String {
    format!("Error: {:?}", e)
}
