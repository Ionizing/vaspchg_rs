#![allow(unused_variables)]
#![allow(dead_code)]
// #![allow(unused_imports)]

use std::io::{self, Write, BufRead, Seek, SeekFrom, BufReader, BufWriter};
use std::path::Path;
use std::fs::File;
use std::marker::PhantomData;

use vasp_poscar::{
    Poscar,
    failure::Error as PoscarError,
};
use ndarray::Array3;
use regex::Regex;
use regex::internal::Input;
use std::process::exit;

pub struct ChgBase {
    // Essential part
    pos:        Poscar,
    chg:        Array3<f64>,
    aug:        Option<String>,
    ngrid:      [usize; 3],

    // Optional part
    chgdiff:    Vec<Array3<f64>>,
    augdiff:    Vec<String>,
}

pub enum ChgType {
    Chg,
    Chgcar,
    Parchg,
}


impl ChgBase {
    pub fn from_file(path: &(impl AsRef<Path> + ?Sized)) -> io::Result<Self> {
        let file = File::open(path)?;
        let mut file = BufReader::new(file);
        Self::from_reader(&mut file)
    }

    pub fn from_reader(file: &mut (impl BufRead+Seek)) -> io::Result<Self> {
        file.seek(SeekFrom::Start(0))?;
        let pos = Self::_read_poscar(file).unwrap();
        let chg = Self::_read_chg(file)?;
        let aug = Self::_read_raw_aug(file).ok();
        let (chgdiff, augdiff) = Self::_read_optional_parts(file).unwrap();
        let ngrid = chg.shape().to_owned();
        let ngrid = [ngrid[0], ngrid[1], ngrid[2]];
        Ok(
            ChgBase { pos, chg, aug, chgdiff, augdiff, ngrid }
        )
    }

    fn _read_optional_parts(file: &mut (impl BufRead+Seek))
        -> io::Result<(Vec<Array3<f64>>, Vec<String>)> {
        let mut chgdiff = vec![];
        let mut augdiff = vec![];

        while let Ok(chg) = Self::_read_chg(file) {
            chgdiff.push(chg);
            if let Ok(aug) = Self::_read_raw_aug(file) {
                augdiff.push(aug);
            }
        }
        Ok((chgdiff, augdiff))
    }

    fn _read_poscar(file: &mut (impl BufRead+Seek)) -> Result<Poscar, PoscarError> {
        let mut buf = String::new();
        while let Ok(n) = file.read_line(&mut buf) {
            if n + 1 == buf.len() - buf.trim_end().len() {
                break;
            }
        }
        Poscar::from_reader(
            io::Cursor::new(buf.into_bytes())
        )
    }

    fn _read_chg(file: &mut (impl BufRead+Seek)) -> io::Result<Array3<f64>> {
        let mut lines = file.lines().map(|l| l.unwrap());
        let nl = lines.next();
        let ngrid_line = if nl.is_none() {
            return Err(
                io::Error::new(io::ErrorKind::Other, "End of file reached.")
            );
        } else { nl.unwrap() };
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
            })
            .into_iter()
            .take(ngrid.iter().product())
            .collect::<Vec<_>>();

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

    fn _write_chg(file: &mut impl Write, chg: &Array3<f64>, num_per_row: usize) -> io::Result<()> {
        let chg = chg.clone().reversed_axes();
        chg.shape().iter().rev()
            .try_for_each(|n| write!(file, " {:>4}", n))?;
        write!(file, "\n")?;
        chg.as_slice().unwrap()
            .chunks(num_per_row)
            .try_for_each(|l| {
                l.iter().try_for_each(|n| write!(file, " {:>17.10E}", n)).unwrap();
                write!(file, "\n")
            })?;
        Ok(())
    }

    pub fn write_writer(&self, file: &mut impl Write, chgtype: ChgType) -> io::Result<()> {
        write!(file, "{:>9.6}", self.get_poscar());
        write!(file, "\n");

        Self::_write_chg(file, self.get_total_chg(), 5)?;
        match chgtype {
            ChgType::Chgcar => {
                assert!(self.get_total_aug().is_some(),
                        "No augmentation data found, cannot save as CHGCAR");
                write!(file, "{}\n", self.get_total_aug().unwrap())?;
            },
            _ => {}
        }

        for i in 0 .. self.get_diff_chg().len() {
            Self::_write_chg(file, &self.get_diff_chg()[i], 5)?;
            match chgtype {
                ChgType::Chgcar => {
                    write!(file, "{}\n", &self.get_diff_aug()[i])?;
                },
                _ => {}
            }
        }

        Ok(())
    }

    pub fn write_file(&self, path: &(impl AsRef<Path> + ?Sized), chgtype: ChgType) -> io::Result<()> {
        let mut file = File::open(path)?;
        let mut buf = BufWriter::new(vec![0u8; 0]);
        self.write_writer(&mut buf, chgtype)?;
        file.write_all(buf.buffer())
    }

    pub fn get_poscar(&self) -> &Poscar             { &self.pos }
    pub fn get_total_chg(&self) -> &Array3<f64>     { &self.chg }
    pub fn get_diff_chg(&self) -> &Vec<Array3<f64>> { &self.chgdiff }
    pub fn get_ngrid(&self) -> &[usize; 3]          { &self.ngrid }
    pub fn get_total_aug(&self) -> Option<&String> {
        if let Some(aug) = &self.aug {
            Some(aug)
        } else { None }
    }
    pub fn get_diff_aug(&self) -> &Vec<String>      { &self.augdiff }
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
    // #[ignore]
    fn test_read_poscar() {
        let mut stream = io::Cursor::new(SAMPLE.as_bytes());
        ChgBase::_read_poscar(&mut stream).unwrap();

        // after read_poscar, stream's cursor should be at "   32   32   32"
        let mut it = stream.lines().map(|l| l.unwrap());
        assert_eq!(it.next(), Some("    2    3    4".to_owned()));
    }

    #[test]
    // #[ignore]
    fn test_read_chg() {
        let mut stream = io::Cursor::new(SAMPLE.as_bytes());
        ChgBase::_read_poscar(&mut stream).unwrap();

        let chg = ChgBase::_read_chg(&mut stream).unwrap();
        assert_eq!(&[2usize, 3, 4], chg.shape());
        assert_eq!(chg[[1, 2, 3]], 0.10568153616E+01);
    }

    #[test]
    // #[ignore]
    fn test_read_aug() {
        let mut stream = io::Cursor::new(SAMPLE.as_bytes());
        ChgBase::_read_poscar(&mut stream).unwrap();
        ChgBase::_read_chg(&mut stream).unwrap();

        let aug = ChgBase::_read_raw_aug(&mut stream).unwrap();
        assert!(aug.ends_with("-0.2068344E-05"));

        if let Some(line) = stream.lines().map(|l| l.unwrap()).next() {
            assert!(line.split_ascii_whitespace().all(|s| s.parse::<usize>().is_ok()));
        }
    }

    #[test]
    fn test_from_reader() {
        let mut stream = io::Cursor::new(SAMPLE.as_bytes());
        let chgcontent = ChgBase::from_reader(&mut stream).unwrap();
        assert_eq!(&chgcontent.ngrid, &[2, 3, 4]);
        assert_eq!(chgcontent.chgdiff.len(), 1);
    }

    #[test]
    // #[ignore]
    fn test_write_chg() {
        let mut istream = io::Cursor::new(SAMPLE);
        let chgcar = ChgBase::from_reader(&mut istream).unwrap();

        let mut ostream = io::Cursor::new(vec![0u8; 0]);
        ChgBase::_write_chg(&mut ostream, chgcar.get_total_chg(), 5).unwrap();
        println!("{a}{a}", a=String::from_utf8(ostream.get_ref().clone()).unwrap());
    }

    #[test]
    fn test_print_aug() {
        let mut istream = io::Cursor::new(SAMPLE);
        let chgcar = ChgBase::from_reader(&mut istream).unwrap();
        dbg!(chgcar.aug);
    }
}