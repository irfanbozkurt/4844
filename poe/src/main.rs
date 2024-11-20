#![allow(unused_imports)]

use circuit::bigint::biguint::{CircuitBuilderBiguint, WitnessBigUint};
use circuit::circuit_logger::CircuitBuilderLogging;
use circuit::nonnative::{CircuitBuilderNonNative, NonNativeTarget};
use circuit::poseidon2::hash::Poseidon2Hash;
use circuit::types::config::{Builder, C, CIRCUIT_CONFIG};
use env_logger::{try_init_from_env, Env, DEFAULT_FILTER_ENV};
use plonky2::iop::witness::PartialWitness;
use poe::blob_polynomial::{BlobPolynomial, BLS12_381_SCALAR_LIMBS};
use poe::bls12_381_scalar_field::BLS12381Scalar;
use poe::fiat_shamir::fiat_shamir_for_proof_of_commitment_equivalence;
use poe::file_utils::{read_blob, read_bls1_381_scalar, read_kzg_commitment_in_goldilocks};

const KZG_COMMITMENT_LIMBS: usize = 12;

fn main() {
    let _ = try_init_from_env(Env::default().filter_or(DEFAULT_FILTER_ENV, "debug"));

    ////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////

    let mut builder = Builder::new(CIRCUIT_CONFIG);

    let blob_polynomial = BlobPolynomial::new(&mut builder);
    let kzg_commitment = builder.add_virtual_biguint_target(KZG_COMMITMENT_LIMBS);

    blob_polynomial
        .iter()
        .for_each(|coeff| builder.register_public_input_biguint(&coeff.value));
    builder.register_public_input_biguint(&kzg_commitment);

    let x: NonNativeTarget<BLS12381Scalar> =
        builder.add_virtual_nonnative_target_sized(BLS12_381_SCALAR_LIMBS);

    let y = blob_polynomial.eval_at(&x);
    builder.println_biguint(&y.value, "y");

    // let y: NonNativeTarget<BLS12381Scalar> =
    //     builder.add_virtual_nonnative_target_sized(BLS12_381_SCALAR_LIMBS);

    // // Get the fiat-shamir challenge point hashing together the plonky2 commitment
    // // and the EVM commitment to the same polynomial
    // let circuit_commitment = blob_polynomial_coefficients.commit::<Poseidon2Hash>(&mut builder);
    // let fiat_shamir_for_proof_of_commitment_equivalence =
    //     fiat_shamir_for_proof_of_commitment_equivalence::<Poseidon2Hash>(
    //         &mut builder,
    //         &circuit_commitment,
    //         &kzg_commitment,
    //     );
    // let challenge_point = fiat_shamir_for_proof_of_commitment_equivalence;

    let data = builder.build::<C>();

    ////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////

    let mut pw = PartialWitness::new();
    read_blob()
        .iter()
        .zip(blob_polynomial.iter())
        .for_each(|(coeff, coeff_target)| {
            pw.set_biguint_target(&coeff_target.value, coeff);
        });
    pw.set_biguint_target(&kzg_commitment, &read_kzg_commitment_in_goldilocks());
    pw.set_biguint_target(&x.value, &read_bls1_381_scalar("x"));

    let proof = data.prove(pw).unwrap();

    ////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////

    data.verify(proof).unwrap();
}
