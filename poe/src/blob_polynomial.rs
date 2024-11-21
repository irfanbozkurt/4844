use circuit::bigint::biguint::CircuitBuilderBiguint;
use circuit::nonnative::{CircuitBuilderNonNative, NonNativeTarget};
use circuit::types::config::{Builder, F};
use num::BigUint;
use plonky2::hash::hash_types::HashOutTarget;
use plonky2::plonk::config::AlgebraicHasher;

use crate::blob_domain::get_brp_roots_of_unity_as_constant;
use crate::bls12_381_scalar_field::{BLS12381Scalar, BLS12_381_SCALAR_LIMBS};

pub const BLOB_WIDTH: usize = 4096;
pub const BLOB_WIDTH_BITS: usize = 12;

// Represents evaluations of polynomial P at points w_0, w_1, ..., w_4096
// where w_i is the i'th 4096'th root of unity in bls12-381 scalar field.
pub struct BlobPolynomial([NonNativeTarget<BLS12381Scalar>; BLOB_WIDTH]);

impl BlobPolynomial {
    pub fn new(builder: &mut Builder) -> Self {
        BlobPolynomial(
            (0..BLOB_WIDTH)
                .map(|_| builder.add_virtual_nonnative_target_sized(BLS12_381_SCALAR_LIMBS))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }

    pub fn iter(&self) -> impl Iterator<Item = &NonNativeTarget<BLS12381Scalar>> {
        self.0.iter()
    }

    pub fn commit<H: AlgebraicHasher<F>>(&self, builder: &mut Builder) -> HashOutTarget {
        builder.hash_n_to_hash_no_pad::<H>(
            self.0
                .iter()
                .flat_map(|coeff| coeff.value.limbs.iter())
                .map(|u32_target| u32_target.0)
                .collect::<Vec<_>>(),
        )
    }

    /// Evaluate a polynomial (in evaluation form) at an arbitrary point ``z``.
    /// - When ``z`` is in the domain, the evaluation can be found by indexing the polynomial at the
    /// position that ``z`` is in the domain.
    /// - When ``z`` is not in the domain, the barycentric formula is used:
    ///    f(z) = (z**WIDTH - 1) / WIDTH  *  sum_(i=0)^WIDTH  (f(DOMAIN[i]) * DOMAIN[i]) / (z - DOMAIN[i])
    ///
    /// In our case:
    /// - ``z`` is the challenge point in Fp
    /// - ``WIDTH`` is BLOB_WIDTH
    /// - ``DOMAIN`` is the bit_reversal_permutation roots of unity
    /// - ``f(DOMAIN[i])`` is the blob[i]
    ///
    pub fn eval_at(
        &self,
        builder: &mut Builder,
        x: &NonNativeTarget<BLS12381Scalar>,
    ) -> NonNativeTarget<BLS12381Scalar> {
        let zero_big = builder.zero_biguint();
        let one_big = builder.one_biguint();
        let one_nonnative = builder.biguint_to_nonnative(&one_big);

        let roots_of_unity_brp = get_brp_roots_of_unity_as_constant(builder);

        let mut result = builder.zero_nonnative();
        let mut cp_is_not_root_of_unity = builder._true();
        let mut barycentric_evaluation = builder.zero_nonnative();
        for i in 0..BLOB_WIDTH {
            let nominator_i = builder.mul_nonnative(&roots_of_unity_brp[i], &self.0[i]);

            // avoid division by zero
            // safe_denominator_i = denominator_i       (denominator_i != 0)
            // safe_denominator_i = 1                   (denominator_i == 0)
            let denominator_i = builder.sub_nonnative(x, &roots_of_unity_brp[i]);
            let is_zero_denominator_i = builder.is_zero_biguint(&denominator_i.value);
            let safe_denominator_i =
                builder.select_biguint(is_zero_denominator_i, &one_big, &denominator_i.value);
            let safe_denominator_i = builder.biguint_to_nonnative(&safe_denominator_i);

            // update `cp_is_not_root_of_unity`
            // cp_is_not_root_of_unity = 1          (initialize)
            // cp_is_not_root_of_unity = 0          (denominator_i == 0)
            let non_zero_denominator_i = builder.not(is_zero_denominator_i);
            cp_is_not_root_of_unity = builder.and(cp_is_not_root_of_unity, non_zero_denominator_i);

            // update `result`
            // result = blob[i]     (challenge_point = roots_of_unity_brp[i])
            let select_blob_i_big =
                builder.select_biguint(is_zero_denominator_i, &self.0[i].value, &zero_big);
            let select_blob_i = builder.biguint_to_nonnative(&select_blob_i_big);
            result = builder.add_nonnative(&result, &select_blob_i);

            let term_i = BLS12381Scalar::divide(builder, &nominator_i, &safe_denominator_i);
            barycentric_evaluation = builder.add_nonnative(&barycentric_evaluation, &term_i);
        }

        let cp_to_the_width = BLS12381Scalar::pow_to_const(builder, &x, BLOB_WIDTH);
        let cp_to_the_width_minus_one = builder.sub_nonnative(&cp_to_the_width, &one_nonnative);
        let width_big = builder.constant_biguint(&BigUint::from(BLOB_WIDTH));
        let width = builder.biguint_to_nonnative(&width_big);
        let factor = BLS12381Scalar::divide(builder, &cp_to_the_width_minus_one, &width);
        barycentric_evaluation = builder.mul_nonnative(&barycentric_evaluation, &factor);

        // if challenge_point is a root of unity, then result = blob[i], else result = barycentric_evaluation
        let select_evaluation_big = builder.select_biguint(
            cp_is_not_root_of_unity,
            &barycentric_evaluation.value,
            &zero_big,
        );
        let select_evaluation = builder.biguint_to_nonnative(&select_evaluation_big);

        builder.add_nonnative(&result, &select_evaluation)
    }
}
