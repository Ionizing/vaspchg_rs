# vaspchg-rs

This is a crate aimed at reading/writing volumetric data file produced by [VASP](www.vasp.at).
Mostly the files should be [`CHGCAR`](https://www.vasp.at/wiki/index.php/CHGCAR),
[`CHG`](https://www.vasp.at/wiki/index.php/CHG) and [`PARCHG`](https://www.vasp.at/wiki/index.php/PARCHG)

# Example
```rust
use vaspchg_rs::{ChgBase, ChgType};

fn main() -> std::io::Result<()> {
    // Reading volumetric data
    let chgcar = ChgBase::from_file("CHGCAR")?;

    // Writing volumetric data to another file
    chgcar.write_file("another_CHGCAR", ChgType::Chgcar)?;

    // manipulating volumetric data
    let pos = chgcar.get_poscar().clone();
    let mut chg = chgcar.get_total_chg().clone();
    chg *= pos.scaled_volume();

    // construct another ChgBase object
    let new_chg = ChgBase::from_builder(chg, vec![], pos);
    new_chg.write_file("new_CHGCAR", ChgType::Parchg)?;
    Ok(())
}
```
 
# Usage/Document

Clone this repository then run `cargo doc` to see the documents.

Detailed usage could be found in ChgBase and unit tests in `tests/` folder.
