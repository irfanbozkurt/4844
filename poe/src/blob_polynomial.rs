use circuit::nonnative::{CircuitBuilderNonNative, NonNativeTarget};
use circuit::types::config::{Builder, F};
use plonky2::hash::hash_types::HashOutTarget;
use plonky2::plonk::config::AlgebraicHasher;

use crate::bls12_381_scalar_field::BLS12381Scalar;

pub const BLS12_381_SCALAR_LIMBS: usize = 8;
pub const BLOB_WIDTH: usize = 4096;

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
    pub fn eval_at(&self, _: &NonNativeTarget<BLS12381Scalar>) -> NonNativeTarget<BLS12381Scalar> {
        // let mut result = NonNativeTarget::default();
        // for (coeff, coeff_target) in self.0.iter().zip(self.iter()) {
        //     let term = coeff_target.mul(&x);
        //     result = result.add(&term);
        // }
        // result

        todo!()
    }
}
