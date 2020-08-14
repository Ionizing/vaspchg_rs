use std::io;
use std::io::Read;
use std::path::{ PathBuf, Path };
use std::fs::File;

use flate2::read::GzDecoder;

use vaspchg_rs::{
    ChgBase,
    ChgType,
};

use crate::get_fpath_in_curr_dir;

#[test]
fn test_read() -> io::Result<()> {
    let path = get_fpath_in_curr_dir!("CHGCAR.nospin.gz");
    let mut file = File::open(path)?;
    let mut gz = GzDecoder::new(file);
    let mut s = String::new();

    gz.read_to_string(&mut s)?;
    let mut stream = io::Cursor::new(s.as_bytes());

    let chg = ChgBase::from_reader(&mut stream);

    Ok(())
}
