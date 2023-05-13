use std::convert::TryInto;
use std::iter;

use halo2_proof::{
    arithmetic::FieldExt,
    circuit::{AssignedCell, Cell, Chip, Layouter, Region, Value},
    plonk::{Advice, Any, Selector, Column, ConstraintSystem, Error, Expression, Fixed},
    poly::Rotation,
};

use super::{
    primitives::{Absorbing, Domain, Mds, Spec, Squeezing, State},
    PaddedWord, PermuteChip, PoseidonInstructions, PoseidonSpongeInstructions,
};

/// Trait for a variable in the circuit
pub trait Var<F:FieldExt>: Clone + std:: fmt::Debug + From<AssignedCell<F,F>> {
    fn cell(&self) -> Cell;
    fn value(&self) -> Value<F>;
}

impl<F:FieldExt> Var<F> for AssignedCell<F,F> {
    fn cell(&self) -> Cell {
        self.cell()
    }

    fn value(&self) -> Value<F> {
        self.value().clone()
    }
}

// configure Pow5 Chip
#[derive(Clone,Debug)]
pub struct Pow5Config<F:FieldExt, const WIDTH:usize, const RATE:usize> {
    pub(crate) state: [Column<Advice>; WIDTH],
    
}
