#![allow(unused_variables)]
#![allow(dead_code)]

use std::io::{
    self,
    Write,
    Read,
    BufRead,
    Seek,
    SeekFrom,
};
use std::path::Path;
use std::fs::File;

use vasp_poscar::{
    Poscar,
    failure::Error as PoscarError,
};
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

    fn _read_poscar<T>(file: &mut T) -> Result<Poscar, PoscarError>
        where T: BufRead+Seek
    {
        file.seek(SeekFrom::Start(0))
            .expect("Unreachable.");
        let mut stream = io::Cursor::<Vec<u8>>::new(vec![]);
        let mut buf = String::new();
        while let Ok(_) = file.read_line(&mut buf) {
            if buf.trim().is_empty() {
                break;
            } else {
                stream.write(buf.as_bytes()).expect("Write to stream failed");
            }
            buf.clear();
        }

        stream.seek(SeekFrom::Start(0));
        Poscar::from_reader(stream)
    }

    fn _read_chg(file: &mut impl BufRead) -> Array3<f64> {
        todo!();
    }

    fn _read_raw_aug(file: &mut impl BufRead) -> String {
        todo!();
    }

    pub fn write_to(path: &impl AsRef<Path>) -> io::Result<()> {
        todo!();
    }

    fn _write_chg(file: &mut impl Write, chg: &Array3<f64>, volume: f64) {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::exit;

    #[test]
    // #[ignore]
    fn test_read_poscar_from_chgcar() {
        let mut stream = io::Cursor::new("\
unknown system
   1.00000000000000
     2.969072   -0.000523   -0.000907
    -0.987305    2.800110    0.000907
    -0.987305   -1.402326    2.423654
   Li
     1
Direct
  0.000000  0.000000  0.000000

   32   32   32
 0.44062142953E+00 0.44635237036E+00 0.46294638829E+00 0.48881056285E+00 0.52211506729E+00
 0.56203432815E+00 0.60956087775E+00 0.66672131696E+00 0.73417916031E+00 0.80884817972E+00
 0.88351172791E+00 0.94912993844E+00 0.10000382501E+01 0.10353398391E+01 0.10568153616E+01
 0.10677009023E+01 0.10709392990E+01 0.10677009023E+01 0.10568153616E+01 0.10353398391E+01");
        ChgBase::_read_poscar(&mut stream).unwrap();
    }
}