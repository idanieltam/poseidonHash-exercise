// Create cryptographically secure and random parameters for the hash function. The parameters 
//generated using Grain help in achieving the desired security level and desired properties 
//for the Poseidon hash function.
use std::{marker::PhantomData};

use bitvec::prelude::*;
use halo2_proofs::arithmetic::FieldExt;
use halo2_proofs::halo2curves::*;

const STATE: usize = 80;

#[derive(Clone, Debug, Copy)]
pub(super) enum FieldType {
    #[allow(dead_code)]
    Binary,
    PrimeOrder,
}

impl FieldType {
    fn tag(&self) -> u8 {
        match self {
            FieldType::Binary => 0,
            FieldType::PrimeOrder => 1,
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub(super) enum SboxType {
    Pow,
    #[allow(dead_code)]
    Inv,
}

impl SboxType {
    fn tag(&self) -> u8 {
        match self {
            SboxType::Pow => 0,
            SboxType::Inv => 1,
        }
    }
}

pub(super) struct Grain<F:FieldExt> {
    state: BitArr!(for 80, in u8, Msb0),
    next_bit:usize,
    _field: PhantomData<F>,
}

impl<F:FieldExt> Grain<F> {
    pub(super) fn new(sbox: SboxType, t:u16, r_f:u16, r_p:u16) -> Self{
        // to initialize the LFSR, we need to set the first 80 bits of the state
        let mut state = bitarr![u8, Msb0; 1; STATE];
        let mut set_bits = |offset:usize, len, value|{
            for i in 0..len {
                *state.get_mut(offset + len - 1 - i).unwrap() = (value >> i) & 1 != 0;

            }
        };

        set_bits(0,2,FieldType::PrimeOrder.tag() as u16);
        set_bits(2,4,sbox.tag() as u16);
        set_bits(6,12,F::NUM_BITS as u16);
        set_bits(18,12,t);
        set_bits(30,10,r_f);
        set_bits(40,10,r_p);

        let mut grain = Grain {
            state,
            next_bit:STATE,
            _field: PhantomData,
        };

        // Discard the first 160 bits of output
        for _ in 0..20 {
            grain.load_next_8_bits();
            grain.next_bit = STATE;
        }

        grain
    }

    fn load_next_8_bits(&mut self) {
        let mut new_bits = 0u8;
        for i in 0..8 {
            new_bits |= 
                ((self.state[i+62]
                ^ self.state[i+51]
                ^ self.state[i+38]
                ^ self.state[i+23]
                ^ self.state[i+13]
                ^ self.state[i]) as u8) << i;
        }
        self.state.rotate_left(8);
        self.next_bit -= 8;
        for i in 0..8 {
            *self.state.get_mut(self.next_bit + i).unwrap() = (new_bits >> i) & 1 != 0;
        }
    }

    fn get_next_bit(&mut self) -> bool {
        if self.next_bit == STATE {
            self.load_next_8_bits();
        }
        let ret = self.state[self.next_bit];
        self.next_bit += 1;
        ret
    }

    // Returns the next field element from this Grain instance, without rejection sampling
    pub(super) fn next_field_element(&mut self) -> F {
        // loop until we get an element from the field

        loop{
            let mut bytes = F::Repr::default();
            // Poseidon reference impl interprets the bits as a repr in MSB order, because
            // it's easy to do that in Python. Meanwhile, our field elements all use LSB
            // order. There's little motivation to diverge from the reference impl; these
            // are all constants, so we aren't introducing big-endianness into the rest of
            // the circuit (assuming unkeyed Poseidon, but we probably wouldn't want to
            // implement Grain inside a circuit, so we'd use a different round constant
            // derivation function there).

            let view = bytes.as_mut();
            for (i, bit) in self.take(F::NUM_BITS as usize).enumerate() {
                view[i/8] |= if bit {1 << (i % 8)} else {0};
            }

            F::from_bytes_wide(&bytes)
        }
    }
    pub(super) fn next_field_element_without_rejection(&mut self) -> F {
        let mut bytes = [0u8; 64];

        // Poseidon reference impl interprets the bits as a repr in MSB order, because
        // it's easy to do that in Python. Additionally, it does not use rejection
        // sampling in cases where the constants don't specifically need to be uniformly
        // random for security. We do not provide APIs that take a field-element-sized
        // array and reduce it modulo the field order, because those are unsafe APIs to
        // offer generally (accidentally using them can lead to divergence in consensus
        // systems due to not rejecting canonical forms).
        //
        // Given that we don't want to diverge from the reference implementation, we hack
        // around this restriction by serializing the bits into a 64-byte array and then
        // calling F::from_bytes_wide. 

        let view = bytes.as_mut();
        for (i, bit) in self.take(F::NUM_BITS as usize).enumerate() {
            // If we diverged from the reference impl and interpreted the bits in LSB
            // order, we would remove this line.
            let i = F::NUM_BITS as usize - 1 - i;

            view[i / 8] |= if bit { 1 << (i % 8) } else { 0 };
        }

        F::from_bytes_wide(&bytes)
    }
}

impl<F:FieldExt> Iterator for Grain<F> {
    type Item = bool;

     fn next(&mut self) -> Option<Self::Item> {
        while !self.get_next_bit() {
            self.get_next_bit();
        }
        Some(self.get_next_bit())
    }
}

#[cfg(test)]
mod test {
    use pasta_curves::Fp;
    use super::{Grain, SboxType};

    #[test]
    fn grain() {
        let mut grain = Grain::<Fp>::new(SboxType::Pow,3,8,56);
        let _f = grain.next_field_element();
    }


}