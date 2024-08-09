use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, fs};

pub fn read_file(file_path: &str) -> io::Result<String> {
    println!(
        "Present working directory of exe {}",
        env::current_dir().unwrap().display()
    );
    fs::read_to_string(file_path)
}

pub fn write_file(file_path: &str, content: &str) -> io::Result<()> {
    fs::write(file_path, content)
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
