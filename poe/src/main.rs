#![allow(unused_imports)]

use circuit::bigint::biguint::{CircuitBuilderBiguint, WitnessBigUint};
use circuit::circuit_logger::CircuitBuilderLogging;
use circuit::nonnative::{CircuitBuilderNonNative, NonNativeTarget};
use circuit::poseidon2::hash::Poseidon2Hash;
use circuit::types::config::{Builder, C, CIRCUIT_CONFIG};
use env_logger::{try_init_from_env, Env, DEFAULT_FILTER_ENV};
use log::Level;
use plonky2::field::types::Field;
use plonky2::iop::witness::PartialWitness;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::timed;
use plonky2::util::timing::TimingTree;
use poe::blob_polynomial::BlobPolynomial;
use poe::bls12_381_scalar_field::{BLS12381Scalar, BLS12_381_SCALAR_LIMBS};
use poe::fiat_shamir::fiat_shamir_for_proof_of_commitment_equivalence;
use poe::file_utils::{read_blob, read_bls1_381_scalar, read_kzg_commitment_in_goldilocks};

pub const KZG_COMMITMENT_LIMBS: usize = 12;

fn main() {
    let _ = try_init_from_env(Env::default().filter_or(DEFAULT_FILTER_ENV, "debug"));

    ////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////

    let mut builder = Builder::new(CircuitConfig {
        num_wires: 136,
        ..CIRCUIT_CONFIG
    });

    let kzg_commitment = builder.add_virtual_biguint_target(KZG_COMMITMENT_LIMBS);
    builder.register_public_input_biguint(&kzg_commitment);

    let blob_polynomial = BlobPolynomial::new(&mut builder);
    // blob_polynomial
    //     .iter()
    //     .for_each(|coeff| builder.register_public_input_biguint(&coeff.value));

    let circuit_commitment = blob_polynomial.commit::<Poseidon2Hash>(&mut builder);
    let evaluation_point = fiat_shamir_for_proof_of_commitment_equivalence::<Poseidon2Hash>(
        &mut builder,
        &circuit_commitment,
        &kzg_commitment,
    );
    let evaluation_result = blob_polynomial.eval_at(&mut builder, &evaluation_point);
    builder.register_public_input_biguint(&evaluation_point.value);
    builder.register_public_input_biguint(&evaluation_result.value);

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
    // pw.set_biguint_target(&x.value, &read_bls1_381_scalar("x"));
    // pw.set_biguint_target(&y_from_file.value, &read_bls1_381_scalar("y"));

    let mut timing = TimingTree::new("prove", Level::Debug);
    let proof = timed!(timing, "prove", { data.prove(pw) });
    timing.print();

    ////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////

    data.verify(proof.unwrap()).unwrap();
}
