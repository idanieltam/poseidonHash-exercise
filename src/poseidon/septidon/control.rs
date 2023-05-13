use super::params::GATE_DEGREE_5;
use super::util::query;
use halo2_proofs::arithmetic::FieldExt;
use halo2_proofs::circuit::{Region, Value};
use halo2_proofs::plonk::{Column, ConstraintSystem, Error, Expression, Fixed, VirtualCells};
use halo2_proofs::poly::Rotation;

#[derive(Clone, Debug)]
pub struct ControlChip{
    is_last: Column<Fixed>
}

pub struct ControlSignals<F:FieldExt>{
    //Signals that control the switches between steps of the permutation
    pub break_full_rounds: Expression<F>,   
    pub break_partial_round: Expression<F>,
    pub transition_round: Expression<F>,

    //A selector that can disable all chips on all rows
    pub selector: Expression<F>,
}

impl ControlChip {
    pub fn configure<F:FieldExt>(cs: &mut ConstraintSystem<F>) -> (Self, ControlSignals<F>) {
        let is_last = cs.fixed_column();
        let signals = query(cs, |meta|{
            let signal_middle = meta.query_fixed(is_last, Rotation(4));
            let signal_last = meta.query_fixed(is_last, Rotation::cur());
            let middle_or_last = signal_middle.clone() + signal_last.clone();

            ControlSignals {
                break_full_rounds: middle_or_last,
                break_partial_round: signal_last,
                transition_round: signal_middle,
                selector: Self::derive_selector(is_last, meta),
            }
        });

        let chip = Self {
            is_last,
        };
        (chip, signals)
    }

    // Assign the fixed positions of the last row of permutations
    pub fn assign<F:FieldExt>(&self, region: &mut Region<'_,F>) -> Result<(),Error> {
        region.assign_fixed(||"", self.is_last, 7, ||Value::known(F::one()))?;
        Ok(())
    }

    fn derive_selector<F:FieldExt>(
        is_last: Column<Fixed>,
        meta: &mut VirtualCells<'_,F>,
    )-> Expression<F> {
        if GATE_DEGREE_5 {
            Expression::Constant(F::one())
        }
        else {
            (0..8_i32).map(|i| 
            meta.query_fixed(is_last, Rotation(i)))
            .reduce(|acc, x| acc + x)
            .unwrap()
        }
    }
}