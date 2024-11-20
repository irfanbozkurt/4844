use circuit::bigint::biguint::BigUintTarget;
use circuit::nonnative::{CircuitBuilderNonNative, NonNativeTarget};
use circuit::types::config::{Builder, F};
use circuit::u32::gadgets::arithmetic_u32::CircuitBuilderU32;
use plonky2::hash::hash_types::HashOutTarget;
use plonky2::plonk::config::AlgebraicHasher;

use crate::bls12_381_scalar_field::BLS12381Scalar;

pub fn fiat_shamir_for_proof_of_commitment_equivalence<H: AlgebraicHasher<F>>(
    builder: &mut Builder,
    circuit_commitment: &HashOutTarget,
    kzg_commitment: &BigUintTarget,
) -> NonNativeTarget<BLS12381Scalar> {
    let challenge_point_biguint = BigUintTarget {
        limbs: builder
            .hash_n_to_hash_no_pad::<H>(
                circuit_commitment
                    .elements
                    .iter()
                    .chain(kzg_commitment.limbs.iter().map(|u32_elem| &u32_elem.0))
                    .map(|reference| *reference)
                    .collect(),
            )
            .elements
            .map(|elem| builder.split_u64_to_u32s_le(elem))
            .iter()
            .flat_map(|e| *e)
            .collect::<Vec<_>>(),
    };

    builder.biguint_to_nonnative(&challenge_point_biguint)
}
