use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn parse_file_line_by_line<P, F>(path: P, mut process_line: F) -> io::Result<()>
where
    P: AsRef<Path>,
    F: FnMut(String),
{
    let file = File::open(path)?;

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        process_line(line);
    }

    Ok(())
}
