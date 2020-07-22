use std::io::{self, Result, Write};
use std::path::Path;
use std::io::Read;
use std::fs::File;

use vasp_poscar::Poscar;
use ndarray::Array3;

pub struct ChgBase {
    pos:        Poscar,
    chg:        Vec<Array3<f64>>,
    chgdiff:    Vec<Array3<f64>>,
    aug:        String,
    augdiff:    String,
}

impl ChgBase {
    pub fn from_file(path: &impl AsRef<Path>) -> io::Result<Self> {
        todo!();
    }

    fn _read_chg(file: &mut impl Read) -> Array3<f64> {
        todo!();
    }

    fn _read_raw_aug(file: &mut impl Read) -> String {
        todo!();
    }

    pub fn write_to(path: &impl AsRef<Path>) -> io::Result<()> {
        todo!();
    }

    pub fn _write_chg(file: &mut impl Write, chg: &Array3<f64>, volume: f64) {
        todo!();
    }
}