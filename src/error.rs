#![allow(dead_code)]
#![allow(unused_imports)]

use std::error::Error;
use std::fmt;

// TODO: discriminate parse failure due to fortran's fucking exceeded length and
//       optional part loss, now I treat all failure as lacking optional part
