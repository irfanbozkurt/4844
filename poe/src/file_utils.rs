use std::fs;
use std::io::Read;
use std::path::Path;

use itertools::Itertools;
use num::{BigUint, Num};

use crate::blob_polynomial::BLOB_WIDTH;

pub const DIR_PATH: &str = "../files";

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
