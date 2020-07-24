#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::io::{
    self,
    Write,
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
use rayon::prelude::*;

pub struct ChgBase {
    pos:        Poscar,
    chg:        Vec<Array3<f64>>,
    chgdiff:    Vec<Array3<f64>>,
    aug:        String,
    augdiff:    String,

    ngrid: [usize; 3],
}

impl ChgBase {
    pub fn from_file(path: &impl AsRef<Path>) -> io::Result<Self> {
        todo!();
    }

    pub fn from_reader<T>(file: &mut T) -> io::Result<Self>
        where T: BufRead+Seek
    {
        file.seek(SeekFrom::Start(0))?;
        let pos = Self::_read_poscar(file).unwrap();
        todo!();
    }

    fn _read_poscar<T>(file: &mut T) -> Result<Poscar, PoscarError>
        where T: BufRead+Seek
    {
        let mut buf = String::new();
        while let Ok(n) = file.read_line(&mut buf) {
            if 1 == n {
                break;
            }
        }
        Poscar::from_reader(
            io::Cursor::new(buf.into_bytes())
        )
    }

    fn _read_chg(file: &mut impl BufRead) -> io::Result<Array3<f64>> {
        let mut lines = file.lines().map(|l| l.unwrap());
        let ngrid_line = lines.next().unwrap();
        let ngrid = ngrid_line.split_ascii_whitespace()
            .take(3)
            .map(|t| t.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let mut buf = Vec::new();
        lines.take_while(|l| !l.trim_start().starts_with("aug"))
            .for_each(|l|
                buf.extend(
                    l.split_ascii_whitespace()
                        .map(|t| t.parse::<f64>().unwrap())
                )
            );
        let chg = Array3::<f64>::from_shape_vec((ngrid[2], ngrid[1], ngrid[0]), buf)
            .unwrap();
        Ok(
            chg.reversed_axes()
        )
    }

    fn _read_raw_aug(file: &mut impl BufRead) -> io::Result<String> {
        todo!();
    }

    pub fn write_to(path: &impl AsRef<Path>) -> io::Result<u64> {
        todo!();
    }

    fn _write_chg(file: &mut impl Write, chg: &Array3<f64>, volume: f64) -> io::Result<u64> {
        todo!();
    }

    fn _write_raw_aug(file: &mut impl Write, raw_aug: &str) -> io::Result<u64> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "\
unknown system
   1.00000000000000
     2.969072   -0.000523   -0.000907
    -0.987305    2.800110    0.000907
    -0.987305   -1.402326    2.423654
   Li
     1
Direct
  0.000000  0.000000  0.000000

    2    3    4
 0.44062142953E+00 0.44635237036E+00 0.46294638829E+00 0.48881056285E+00 0.52211506729E+00
 0.56203432815E+00 0.60956087775E+00 0.66672131696E+00 0.73417916031E+00 0.80884817972E+00
 0.88351172791E+00 0.94912993844E+00 0.10000382501E+01 0.10353398391E+01 0.10568153616E+01
 0.10677009023E+01 0.10709392990E+01 0.10677009023E+01 0.10568153616E+01 0.10353398391E+01
 0.10677009023E+01 0.10709392990E+01 0.10677009023E+01 0.10568153616E+01
augmentation occupancies 1 15
  0.2743786E+00 -0.3307158E-01  0.0000000E+00  0.0000000E+00  0.0000000E+00
  0.1033253E-02  0.0000000E+00  0.0000000E+00  0.0000000E+00  0.3964234E-01
  0.5875445E-05 -0.7209739E-05 -0.3625569E-05  0.1019266E-04 -0.2068344E-05
";

    #[test]
    fn test_read_poscar() {
        let mut stream = io::Cursor::new(SAMPLE.as_bytes());
        ChgBase::_read_poscar(&mut stream).unwrap();

        // after read_poscar, stream's cursor should be at "   32   32   32"
        let mut it = stream.lines().map(|l| l.unwrap());
        assert_eq!(it.next(), Some("    2    3    4".to_owned()));
    }

    #[test]
    fn test_read_ngrid() {
        let mut stream = io::Cursor::new(SAMPLE.as_bytes());
        ChgBase::_read_poscar(&mut stream).unwrap();

        let chg = ChgBase::_read_chg(&mut stream).unwrap();
        assert_eq!(&[2usize, 3, 4], chg.shape());
    }
}