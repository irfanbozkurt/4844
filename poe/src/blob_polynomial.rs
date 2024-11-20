use std::fs;
use std::io::Read;
use std::path::Path;

use circuit::nonnative::{CircuitBuilderNonNative, NonNativeTarget};
use circuit::types::config::{Builder, F};
use itertools::Itertools;
use num::{BigUint, Num};
use plonky2::hash::hash_types::HashOutTarget;
use plonky2::plonk::config::AlgebraicHasher;

use crate::bls12_381_scalar_field::BLS12381Scalar;

pub const DIR_PATH: &str = "../files";
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

    // Barycentric evaluation
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

pub fn read_blob() -> [BigUint; BLOB_WIDTH] {
    let mut file = fs::File::open(Path::new(DIR_PATH).join("blob")).unwrap();
    let mut blob_hex_string = String::new();
    file.read_to_string(&mut blob_hex_string).unwrap();
    assert_eq!(blob_hex_string.len(), BLOB_WIDTH * 32 * 2);

    blob_hex_string
        .trim()
        .chars()
        .chunks(64)
        .into_iter()
        .map(|chunk| BigUint::from_str_radix(&chunk.collect::<String>(), 16).unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

pub fn read_kzg_commitment_in_goldilocks() -> BigUint {
    let mut file = fs::File::open(Path::new(DIR_PATH).join("commitment")).unwrap();
    let mut kzg_commitment_hex_string = String::new();
    file.read_to_string(&mut kzg_commitment_hex_string).unwrap();
    assert_eq!(kzg_commitment_hex_string.len(), 96);

    BigUint::from_str_radix(&kzg_commitment_hex_string, 16).unwrap()
}

pub fn read_bls1_381_scalar(path: &str) -> BigUint {
    let mut file = fs::File::open(Path::new(DIR_PATH).join(path)).unwrap();
    let mut bls1_381_scalar_hex_string = String::new();
    file.read_to_string(&mut bls1_381_scalar_hex_string)
        .unwrap();
    assert_eq!(bls1_381_scalar_hex_string.len(), 64);

    BigUint::from_str_radix(&bls1_381_scalar_hex_string, 16).unwrap()
}
