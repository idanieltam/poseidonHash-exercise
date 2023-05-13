use halo2_proofs::circuit::Value;
use halo2_proofs::arithmetic::FieldExt;
use halo2_proofs::plonk::{ConstraintSystem,Expression,VirtualCells};

pub fn map_array<IN,OUT,FN>(array: &[IN;3], mut f:FN) -> [OUT;3]
where 
    FN: FnMut(&IN) -> OUT,
    {
        let a = f(&array[0]);
        let b = f(&array[1]);
        let c = f(&array[2]);
        [a,b,c]
    }

// Helper to make queries to a ConstraintSystem, Escape the "create_gate" closures
pub fn query<T, F: FieldExt>(
    cs: &mut ConstraintSystem<F>,
    f: impl FnOnce(&mut VirtualCells<'_,F>) -> T
) -> T {
    let mut queries:Option<F> = None;
    cs.create_gate("query", |meta| {
        queries = Some(f(meta));
        [Expression::Constant(F::zero())]
    });
    queries.unwrap()
}

pub fn join_values<F:FieldExt>(values: [Value<F>;3]) -> Value<[F;3]> {
    values[0]
        .zip(values[1])
        .zip(values[2])
        .map(|((v0,v1),v2)| [v0,v1,v2])
}

pub fn split_value<F:FieldExt>(value: Value<[F;3]>) -> [Value<F>;3] {
    [
        value.map(|v| v[0]),
        value.map(|v| v[1]),
        value.map(|v| v[2]),
    ]
}

pub mod pow_5 {
    use super::FieldExt;
    use halo2_proofs::plonk::Expression;

    pub fn expr<F:FieldExt>(v:Expression<F>) -> Expression<F> {
        let v2 = v.clone() * v.clone();
        v2.clone() * v2 * v
    }

    pub fn value<F:FieldExt>(v:F) -> F {
        let v2 = v * v;
        v2 * v2 * v
    }
}

pub mod matmul {
    use super::super::params::Mds;
    use super::FieldExt;
    use halo2_proofs::plonk::{Expression};
    use std::convert::TryInto;

    // Multiply a vector of expressions by a constant matrix
    pub fn expr<F:FieldExt>(matrix: &Mds<F>, vector: [Expression<F>;3]) -> [Expression<F>;3] {
        (0..3)
            .map(|next_idx|{
                (0..3)
                    .map(|idx| vector[idx].clone() * matrix[next_idx][idx])
                    .reduce(|acc,term| acc + term)
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    // Multiply a vector of values by a constant matrix.
    pub fn value<F: FieldExt>(matrix: &Mds<F>, vector: [F; 3]) -> [F; 3] {
        (0..3)
            .map(|next_idx| {
                (0..3)
                    .map(|idx| vector[idx] * matrix[next_idx][idx])
                    .reduce(|acc, term| acc + term)
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
}

pub mod select {
    use halo2_proofs::{arithmetic::FieldExt, plonk::Expression};

    pub fn expr<F:FieldExt>(
        selector: Expression<F>,
        when_true:Expression<F>,
        when_false:Expression<F>,
    ) -> Expression<F> {
        let one = Expression::Constant(F::one());
        selector * when_true + (F::one() - selector) * when_false
    }
}

pub mod or {
    use halo2_proofs::{arithmetic::FieldExt, plonk::Expression};

    pub fn expr<F:FieldExt>(
        a: Expression<F>,
        b: Expression<F>,
    ) -> Expression<F> {
        let one = Expression::Constant(F::one());
        one.clone() - ((one.clone() - a) * (one - b))
    }

    pub fn value<F:FieldExt>(
        a:F,
        b:F,
    ) -> F {
        let one = F::one();
        one - ((one - a) * (one - b))
    }
}