use std::io;
use std::path::{ PathBuf, Path };

use flate2;

use vaspchg_rs;

fn get_current_dir() -> PathBuf {
    let mut path = PathBuf::from(file!());
    path.pop();
    path
}

fn get_fpath_in_curr_dir(fname: &impl AsRef<Path>) -> PathBuf {
    let mut path = get_current_dir();
    path.push(fname);
    path
}

#[test]
fn test_read() -> io::Result<()> {

    Ok(())
}
