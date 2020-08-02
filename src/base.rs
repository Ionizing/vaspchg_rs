#![allow(unused_variables)]
#![allow(dead_code)]
// #![allow(unused_imports)]

use std::io::{self, Write, BufRead, Seek, SeekFrom, BufReader};
use std::path::Path;
use std::fs::File;
use std::marker::PhantomData;

use vasp_poscar::{
    Poscar,
    failure::Error as PoscarError,
};
use ndarray::Array3;
use regex::Regex;

pub(crate) struct CHG;
pub(crate) struct CHGCAR;
pub(crate) struct PARCHG;

struct ChgBase<T> {
    pos:        Poscar,
    chg:        Array3<f64>,
    chgdiff:    Option<Array3<f64>>,
    aug:        Option<String>,
    augdiff:    Option<String>,

    ngrid: [usize; 3],
    _dummy: PhantomData<T>,
}

// type Chgcar = PhantomData<_>;

pub(crate) trait ChgBaseWrite<T> {
    fn write_file(&self) -> io::Result<()>;
}

impl<T> ChgBase<T> {
    pub fn from_file(path: &impl AsRef<Path>) -> io::Result<Self> {
        let file = File::open(path)?;
        let mut file = BufReader::new(file);
        Self::from_reader(&mut file)
    }

    pub fn from_reader(file: &mut (impl BufRead+Seek)) -> io::Result<Self> {
        file.seek(SeekFrom::Start(0))?;
        let pos = Self::_read_poscar(file).unwrap();
        let chg = Self::_read_chg(file)?;
        let aug = Self::_read_raw_aug(file).ok();
        let chgdiff = Self::_read_chg(file).ok();
        let augdiff = Self::_read_raw_aug(file).ok();
        let ngrid = chg.shape().to_owned();
        let ngrid = [ngrid[0], ngrid[1], ngrid[2]];
        Ok(
            ChgBase { pos, chg, chgdiff, aug, augdiff, ngrid, _dummy: PhantomData }
        )
    }

    fn _read_poscar(file: &mut (impl BufRead+Seek)) -> Result<Poscar, PoscarError> {
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

    fn _read_chg(file: &mut (impl BufRead+Seek)) -> io::Result<Array3<f64>> {
        let mut lines = file.lines().map(|l| l.unwrap());
        let ngrid_line = lines.next().unwrap();
        let ngrid = ngrid_line.split_ascii_whitespace()
            .take(3)
            .map(|t| t.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let mut len = 0;
        let buf = lines
            .take_while(|l| {
                if !l.starts_with("aug") {
                    true
                } else {
                    len = l.len() as i64 + 1;   // "+1" means taking account of the '\n'.
                    false
                }
            })
            .fold(Vec::new(), |mut acc, l| {
                acc.extend(l.split_ascii_whitespace()
                    .map(|t| t.parse::<f64>().unwrap()));
                acc
            });
        file.seek(SeekFrom::Current(0 - len))?; // move cursor back in front of 'augmentation'
        let chg = Array3::<f64>::from_shape_vec((ngrid[2], ngrid[1], ngrid[0]), buf).unwrap();
        Ok(
            chg.reversed_axes()
        )
    }

    fn _read_raw_aug(file: &mut (impl BufRead+Seek)) -> io::Result<String> {
        let re = Regex::new(r"^(\s*\d+){3}").unwrap();
        let lines = file.lines().map(|l| l.unwrap());
        let mut len = 0;
        let raw_aug = lines
            .take_while(|l| {
                if !re.is_match(l) {
                    true
                } else {
                    len = l.len() as i64 + 1;
                    false
                } })                // take until " NGXF NGYF NGZF"
            .fold(String::new(), |acc, x| acc + "\n" + &x);  // Join all the lines with \n
        file.seek(SeekFrom::Current(0 - len))?;
        Ok(raw_aug)
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
    struct DummyType;
    type DummyChgType = ChgBase<DummyType>;

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
augmentation occupancies 2 15
  0.2743786E+00 -0.3307158E-01  0.0000000E+00  0.0000000E+00  0.0000000E+00
  0.1033253E-02  0.0000000E+00  0.0000000E+00  0.0000000E+00  0.3964234E-01
  0.5875445E-05 -0.7209739E-05 -0.3625569E-05  0.1019266E-04 -0.2068344E-05
    2    3    4
 0.44062142953E+00 0.44635237036E+00 0.46294638829E+00 0.48881056285E+00 0.52211506729E+00
 0.56203432815E+00 0.60956087775E+00 0.66672131696E+00 0.73417916031E+00 0.80884817972E+00
 0.88351172791E+00 0.94912993844E+00 0.10000382501E+01 0.10353398391E+01 0.10568153616E+01
 0.10677009023E+01 0.10709392990E+01 0.10677009023E+01 0.10568153616E+01 0.10353398391E+01
 0.10677009023E+01 0.10709392990E+01 0.10677009023E+01 0.12668153616E+01
augmentation occupancies 1 15
  0.2743786E+00 -0.3307158E-01  0.0000000E+00  0.0000000E+00  0.0000000E+00
  0.1033253E-02  0.0000000E+00  0.0000000E+00  0.0000000E+00  0.3964234E-01
  0.5875445E-05 -0.7209739E-05 -0.3625569E-05  0.1019266E-04 -0.2038144E-05
augmentation occupancies 2 15
  0.2743786E+00 -0.3307158E-01  0.0000000E+00  0.0000000E+00  0.0000000E+00
  0.1033253E-02  0.0000000E+00  0.0000000E+00  0.0000000E+00  0.3964234E-01
  0.5875445E-05 -0.7209739E-05 -0.3625569E-05  0.1019266E-04 -0.0038244E-05
";

    #[test]
    fn test_read_poscar() {
        let mut stream = io::Cursor::new(SAMPLE.as_bytes());
        DummyChgType::_read_poscar(&mut stream).unwrap();

        // after read_poscar, stream's cursor should be at "   32   32   32"
        let mut it = stream.lines().map(|l| l.unwrap());
        assert_eq!(it.next(), Some("    2    3    4".to_owned()));
    }

    #[test]
    fn test_read_chg() {
        let mut stream = io::Cursor::new(SAMPLE.as_bytes());
        DummyChgType::_read_poscar(&mut stream).unwrap();

        let chg = DummyChgType::_read_chg(&mut stream).unwrap();
        assert_eq!(&[2usize, 3, 4], chg.shape());
        assert_eq!(chg[[1, 2, 3]], 0.10568153616E+01);
    }

    #[test]
    fn test_read_aug() {
        let mut stream = io::Cursor::new(SAMPLE.as_bytes());
        DummyChgType::_read_poscar(&mut stream).unwrap();
        DummyChgType::_read_chg(&mut stream).unwrap();

        let aug = DummyChgType::_read_raw_aug(&mut stream).unwrap();
        dbg!(&aug);
        assert!(aug.ends_with("-0.2068344E-05"));

        if let Some(line) = stream.lines().map(|l| l.unwrap()).next() {
            assert!(line.split_ascii_whitespace().all(|s| s.parse::<usize>().is_ok()));
        }
    }
}