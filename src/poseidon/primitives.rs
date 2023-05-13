use std::convert::TryInto;
use std::fmt;
use std::iter;
use std::marker::PhantomData;

use halo2_proofs::arithmetic::FieldExt;

pub(crate) mod grain;
pub(crate) mod mds;

mod fields;
#[macro_use]
mod binops;

pub(crate) mod bn256;
#[cfg(test)]
pub(crate) mod pasta;

mod p128pow5t3;
mod p128pow5t3_compact;

pub use p128pow5t3::P128Pow5T3;
pub(crate) use p128pow5t3::P128Pow5T3Constants;
pub use p128pow5t3_compact::P128Pow5T3Compact;

use grain::SboxType;

/// The type used to hold permutation state.
pub(crate) type State<F, const T: usize> = [F; T];

/// The type used to hold sponge rate.
pub(crate) type SpongeRate<F, const RATE: usize> = [Option<F>; RATE];

/// The type used to hold the MDS matrix and its inverse.
pub(crate) type Mds<F, const T: usize> = [[F; T]; T];


