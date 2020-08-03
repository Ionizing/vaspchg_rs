use crate::base;

use std::io;
use std::io::Write;
use std::path::Path;

struct ChgcarType;
pub type Chgcar = base::ChgBase<ChgcarType>;

impl<ChgcarType> base::ChgWrite<ChgcarType> for Chgcar {
    fn write_file(&self, path: &impl AsRef<Path>) -> io::Result<()> {
        todo!();
    }

    fn write_writer(&self, file: &mut impl Write) -> io::Result<()> {
        todo!();
    }
}
