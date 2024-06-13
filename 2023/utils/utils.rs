use std::fs::File;
use std::io::{
    Result as ioResult,
    BufReader
};

pub fn load_file(file_name: &str) -> ioResult<BufReader<File>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

trait 

/*trait FileName {
    fn from_fname(file_name: &str) -> ioResult<Self> where Self: Sized;
}

impl<File> FileName for BufReader<File> {
    fn from_fname(file_name: &str) -> ioResult<Self> {
        let file = File::open(file_name)?;
        let reader = Self::new(file);

        Ok(reader)
    }
}*/

fn main() {}