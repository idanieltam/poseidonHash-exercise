use std::convert::TryInto;
use std::fmt;
use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::{Field, FieldExt},
    circuit::{AssignedCell, Chip, Layouter},
    plonk::{ConstraintSystem, Error},
};

mod pow5;
pub use pow5::{Pow5Chip, Pow5Config, StateWord, Var};

mod septidon;
pub use septidon::{CachedConstants, SeptidonChip};

pub mod primitives;
use primitives::{Absorbing, ConstantLength, Domain, Spec, SpongeMode, Squeezing, State};
use std::fmt::Debug as DebugT;