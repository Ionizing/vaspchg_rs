#![allow(non_snake_case)]
mod spin { mod test; }
mod no_spin { mod test; }
mod soc { mod test; }
mod Fe3O4 { mod test; }

#[macro_export]
macro_rules! get_fpath_in_curr_dir {
    ($fname:expr) => {{
        let mut path = PathBuf::from(file!());
        path.pop();
        path.push($fname);
        path
    }};
}