use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_content<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let data = std::fs::read_to_string(filename)?;
    Ok(data)
}

pub fn file_exists<P>(filename: P) -> bool
where
    P: AsRef<Path>,
{
    let path: &Path = filename.as_ref();
    return path.exists();
}
