use halo2_proofs::arithmetic::FieldExt;

use super::{grain::Grain,Mds};

pub(super) fn generate_mds<F:FieldExt, const T: usize>(
    grain: &Grain<F,T>,
    mut select: usize,
) ->(Mds<F,T>,Mds<F,T>) {
    let(xs,ys,mds) = loop {
        let(xs,ys) = loop {
            // Generate two [F;T] arrays of unique random field elements
        let mut vals: Vec<_> = (0..2 * T)
        .map(|_| grain.next_field_element_without_rejection())
        .collect();

        // Check if we have unique elements
        let mut unique = vals.clone();
        unique.sort_unstable();
        unique.dedup();
        if unique.len() == vals.len() {
            let rhs = vals.split_off(T);
            break (vals,rhs);
        }
    };

    if select != 0 {
        select -= 1;
        continue;
    }

    let mut mds = [[F::ZERO; T]; T];
        #[allow(clippy::needless_range_loop)]
        for i in 0..T {
            for j in 0..T {
                let sum = xs[i] + ys[j];
                // We leverage the secure MDS selection counter to also check this.
                assert!(!sum.is_zero_vartime());
                mds[i][j] = sum.invert().unwrap();
            }
        }

        break (xs, ys, mds);
    };

    let mut mds_inv = [[F::ZERO; T]; T];
    let l = |xs: &[F], j, x: F| {
        let x_j = xs[j];
        xs.iter().enumerate().fold(F::ONE, |acc, (m, x_m)| {
            if m == j {
                acc
            } else {
                // We hard-code the type, to avoid spurious "cannot infer type" rustc errors.
                let denominator: F = x_j - x_m;

                // We can invert freely; by construction, the elements of xs are distinct.
                let denominator_inverted: F = denominator.invert().unwrap();

                acc * (x - x_m) * denominator_inverted
            }
        })
    };
    let neg_ys: Vec<_> = ys.iter().map(|y| -*y).collect();
    for i in 0..T {
        for j in 0..T {
            mds_inv[i][j] = (xs[j] - neg_ys[i]) * l(&xs, j, neg_ys[i]) * l(&neg_ys, i, xs[j]);
        }
    }

    (mds, mds_inv)
}
