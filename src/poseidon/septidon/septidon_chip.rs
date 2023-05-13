use halo2_proofs::circuit::{Region, Value};
use halo2_proofs::plonk::{ConstraintSystem, Error};

use super::control::ControlChip;
use super::full_round::FullRoundChip;
use super::loop_chip::LoopChip;
use super::params::{round_constant, CachedConstants};
use super::septuple_round::SeptupleRoundChip;
use super::state::Cell;
use super::transition_round::TransitionRoundChip;
use super::util::map_array;

// The configuration of the permutation

#[derive(Clone, Debug)]
pub struct SeptidonChip {
    control_chip : ControlChip,
    transition_chip : TransitionRoundChip,
    full_round_chip : FullRoundChip,
    partial_round_chip : SeptupleRoundChip,
}

impl SeptidonChip {
    pub fn configure<F:CachedConstants>(
        cs: &mut ConstraintSystem<F>,
    ) -> Self {
        let (control_chip, signals) = ControlChip::configure(cs);
        let q = || signals.selector.clone();
        let (full_round_chip, full_round_loop_body) = FullRoundChip::configure(cs);
        let (partial_round_chip, partial_round_loop_body) = SeptupleRoundChip::configure(cs,q());

        let transition_chip = {
            // The output of the transition round is the input of the partial round loop
            let output = partial_round_chip.input();
            TransitionRoundChip::configure(cs, signals.transition_round, output)
        };

        {
            // The output of the full round is the input of the transition round
            let output = transition_chip.input();
            LoopChip::configure(cs, q(), full_round_loop_body, signals.break_full_rounds, output)
        }

        {
            // The output of the partial rounds go horizontally into the second loop of full rounds
            // which runs parallel to the last 4 partial rounds indexed[-3;0]
            let full_round_sboxes = & full_round_chip.0.0;
            let output:[Cell;3] = [
                full_round_sboxes[0].input.rotated(-3),
                full_round_sboxes[1].input.rotated(-3),
                full_round_sboxes[2].input.rotated(-3),
            ];

            LoopChip::configure(cs, q(), partial_round_loop_body, signals.break_partial_round, output)
        };

        Self { control_chip, transition_chip, full_round_chip, partial_round_chip}
    }
}