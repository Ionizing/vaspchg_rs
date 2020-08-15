use std::io;
use std::io::Read;
use std::fs::{File, remove_file};
use std::path::{PathBuf};

use flate2::read::GzDecoder;

use vaspchg_rs::{
    ChgType,
    ChgBase,
};

use crate::get_fpath_in_curr_dir;

#[test]
fn test_read() -> io::Result<()> {
    let path = get_fpath_in_curr_dir!("CHGCAR.NiO_SOC.gz");
    let file = File::open(path)?;
    let mut gz = GzDecoder::new(file);
    let mut s = String::new();

    gz.read_to_string(&mut s)?;
    let mut stream = io::Cursor::new(s.as_bytes());

    let chg = ChgBase::from_reader(&mut stream)?;
    let mut stream = io::Cursor::new(vec![0u8; 0]);
    chg.write_writer(&mut stream, ChgType::Chgcar)?;
    assert_eq!(141111, String::from_utf8(stream.get_ref().clone()).unwrap().lines().count());
    chg.write_file(&get_fpath_in_curr_dir!("CCHGCAR_soc_test.vasp"), ChgType::Chgcar)?;
    remove_file(&get_fpath_in_curr_dir!("CCHGCAR_soc_test.vasp"))?;
    Ok(())
}
