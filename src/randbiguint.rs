extern crate ff;
extern crate num_bigint;
extern crate rand;

use self::rand::Rng;
use self::num_bigint::BigUint;

// Extension trait to make generating BigUint in range easier
pub trait RandBigUint {
    fn gen_b_range(&mut self, low: &BigUint, high: &BigUint) -> BigUint;
}

impl<R: Rng> RandBigUint for R {
    fn gen_b_range(&mut self, low: &BigUint, high: &BigUint) -> BigUint {
        let range = high - low;
        let bytes = range.to_bytes_be();
        let mut random_bytes = vec![0u8; bytes.len()];
        self.fill_bytes(&mut random_bytes);
        let mut val = BigUint::from_bytes_be(&random_bytes);
        val %= range;
        val + low
    }
}