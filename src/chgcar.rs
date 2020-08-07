#![allow(dead_code)]
#![allow(unused)]

use crate::base;

use std::io;
use std::io::{Write, BufWriter};
use std::path::Path;
use std::fs::File;

pub struct ChgcarType;
pub type Chgcar = base::ChgBase<ChgcarType>;
use base::ChgWrite;

impl ChgWrite for Chgcar {
    fn write_file(&self, path: &impl AsRef<Path>) -> io::Result<()> {
        let mut file = File::open(path)?;
        let mut buf = BufWriter::new(vec![0u8; 0]);
        self.write_writer(&mut buf)?;
        file.write_all(buf.buffer())
    }

    fn write_writer(&self, file: &mut impl Write) -> io::Result<()> {
        write!(file, "{:>9.6}", self.get_poscar());
        write!(file, "\n");

        Self::_write_chg(file, self.get_total_chg(), 5)?;

        assert!(self.get_total_aug().is_some(), "No augmentation data found in this chg.");
        write!(file, "{}\n", self.get_total_aug().unwrap())?;

        if let Some(chgdiff) = self.get_diff_chg() {
            Self::_write_chg(file, &chgdiff[0], 5)?;
            write!(file, "{}\n", self.get_diff_aug().unwrap()[0])?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use crate::base::ChgWrite;

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
    #[ignore]
    fn test_writer() {
        let mut istream = io::Cursor::new(SAMPLE);
        let chgcar = Chgcar::from_reader(&mut istream).unwrap();

        let mut ostream = io::Cursor::new(vec![0u8; 0]);
        chgcar.write_writer(&mut ostream);
        println!("{}", String::from_utf8(ostream.get_ref().clone()).unwrap());
    }
}