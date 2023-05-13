use std::marker::PhantomData;
use halo2_proofs::arithmetic::FieldExt;

use super::p128pow5t3::P128Pow5T3Constants;
use super::{Mds, Spec};

#[derive(Debug)]
pub struct P128Pow5T3Compact<Fp> {
    _marker: PhantomData<Fp>,
}

impl<Fp:P128Pow5T3Constants> Spec<Fp,3,2> for P128Pow5T3Compact<Fp> {
    fn full_rounds() -> usize { Fp::full_rounds() }
    fn partial_rounds() -> usize { Fp::partial_rounds() } 
    fn sbox(val:Fp) -> Fp {
        val.pow_vartime([5])
    }
    fn secure_mds() -> usize {
        unimplemented!()
    }
    fn constants() -> (Vec<[Fp;3]>,Mds<Fp,3>,Mds<Fp,3>) {
        let (mut rc,mds,mds_inv) = (Fp::round_constants(),Fp::mds(),Fp::mds_inv());
        let first_partial = Self::full_rounds()/2;
        let after_partial = first_partial + Self::partial_rounds();
        //?

        //Propagate the constants of each partial round to the next
        for i in first_partial..after_partial {
            // Extract the constants rc[i][1] and rc[i][2] that do not pass through the S-box.
            // Leave the value 0 in their place.
            // rc[i][0] stays in place.
            let rc_tail = vec_remove_tail(&mut rc[i]);

            // Pass forward through the MDS matrix.
            let rc_carry = mat_mul(&mds, &rc_tail);
            
            //Accumulate the carried constants into the next round.
            vec_accumulate(&mut rc[i+1], &rc_carry);

        }
        //Now constants are accumlated into next full round
        (rc,mds,mds_inv)    
    }
}

fn mat_mul<Fp:FieldExt, const T:usize>( mat: &Mds<Fp,T>,input: &[Fp;T]) -> [Fp;T] {
    let mut output = [Fp::zero();T];
    for i in 0..T {
        for j in 0..T {
            output[i] += mat[i][j] * input[j];
        }
    }
    output
}

fn vec_accumulate<Fp:FieldExt, const T:usize>( vec: &mut [Fp;T],input: &[Fp;T]) {
    for i in 0..T {
        vec[i] += input[i];
    }
}

fn vec_remove_tail<Fp:FieldExt, const T:usize>( vec: &mut [Fp;T]) -> [Fp;T] {
    let mut tail = [Fp::zero();T];
    for i in 1..T {
        tail[i] = vec[i];
        vec[i] = Fp::zero();
    }
    tail
}