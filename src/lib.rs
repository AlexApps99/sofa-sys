#![allow(non_snake_case)]
#![allow(clippy::excessive_precision)]

#[cfg(not(feature = "generate"))]
mod sofa_c;
#[cfg(feature = "generate")]
mod sofa_c {
    include!(concat!(env!("OUT_DIR"), "/sofa_c.rs"));
}

mod sofam;

pub use sofa_c::*;
pub use sofam::*;
