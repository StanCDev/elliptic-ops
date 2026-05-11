extern crate ff;
extern crate num_bigint;

use self::ff::{PrimeField};
use self::num_bigint::BigUint;

use crate::fp::stark::FStark;
use crate::fp::nist::Fp;

pub trait CurveConfig: PrimeField {
    fn a() -> Self;
    fn b() -> Self;
    fn g_x() -> Self;
    fn g_y() -> Self;
    fn n() -> BigUint;
    fn power_of_two_table() -> Vec<Point<Self>> {
        let mut table = Vec::with_capacity(256);
        let mut current = Point::generator();
        for _ in 0..256 {
            table.push(current);
            current = current.double();
        }
        table
    }
}

// Implement it for FStark
impl CurveConfig for FStark {
    fn a() -> Self { Self::a() }
    fn b() -> Self { Self::b() }
    fn g_x() -> Self { Self::g_x() }
    fn g_y() -> Self { Self::g_y() }
    fn n() -> BigUint { Self::n() }
}

// Implement it for Fp
impl CurveConfig for Fp {
    fn a() -> Self { Self::a() }
    fn b() -> Self { Self::b() }
    fn g_x() -> Self { Self::g_x() }
    fn g_y() -> Self { Self::g_y() }
    fn n() -> BigUint { Self::n() }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Point<F: CurveConfig> {
    Affine { x: F, y: F },
    Infinity,
}

impl<F: CurveConfig> Point<F> {
    pub fn generator() -> Self {
        Point::Affine { x: F::g_x(), y: F::g_y() }
    }

    pub fn invert(&self) -> Self {
        match self {
            Point::Infinity => *self,
            Point::Affine { x: x1, y: y1 } => Point::Affine { x : *x1, y : -*y1}
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        match (self, other) {
            (Point::Infinity, p) | (p, Point::Infinity) => *p,
            (Point::Affine { x: x1, y: y1 }, Point::Affine { x: x2, y: y2 }) => {
                if x1 == x2 {
                    if y1 == y2 {
                        self.double()
                    } else {
                        Point::Infinity
                    }
                } else {
                    let num = *y2 - *y1;
                    let denom = (*x2 - *x1).invert().unwrap();
                    let lambda = num * denom;
                    let x3 = lambda.square() - *x1 - *x2;
                    let y3 = lambda * (*x1 - x3) - *y1;
                    Point::Affine { x: x3, y: y3 }
                }
            }
        }
    }

    pub fn double(&self) -> Self {
        match self {
            Point::Infinity => Point::Infinity,
            Point::Affine { x, y } => {
                // 3x^2 + a
                let x_sq = x.square();
                let num = (x_sq + x_sq + x_sq) + F::a();
                let denom = (*y + *y).invert().unwrap();
                let lambda = num * denom;
                let xt = lambda.square() - (*x + *x);
                let yt = lambda * (*x - xt) - *y;
                Point::Affine { x: xt, y: yt }
            }
        }
    }
    //subject to timing attacks
    pub fn mul_gen(k: &BigUint, table : &Vec<Point<F>>) -> Self {
        let mut res = Point::Infinity;
        let bits = k.to_radix_le(2);
        
        for (i, bit) in bits.iter().enumerate() {
            if *bit == 1 {
                res = res.add(&table[i]);
            }
        }
        res
    }

    //subject to timing attacks possibly due to cache accesses
    pub fn mul(&self, k: &BigUint) -> Self {
        // 1. Generate table of size 16.
        let mut table: Vec<Self> = Vec::with_capacity(16);
        table.push(Point::Infinity);
        for i in 1..16 {
            table.push(table[i-1].add(self));
        }
        // 2. Process 4 bits at a time
        // naive implementation: for every bit double everytime, if its a 1 add
        let mut res = Point::Infinity;
        let nibbles = k.to_radix_be(16);
        
        for idx in nibbles.iter() {
            if res != Point::Infinity {
                for _ in 0..4 {
                    res = res.double();
                }
            }
            res = res.add(&table[*idx as usize]);
        }
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use self::ff::Field; 
    use self::ff::PrimeField;
    extern crate rand;
    use crate::randbiguint::RandBigUint;

    const NBR_ITS : usize = 100;

    //// FSTARK TESTS NOW

    #[test]
    fn test_curve_is_non_singular() {
        let four = FStark::from_str_vartime("4").unwrap();
        let twenty_seven = FStark::from_str_vartime("27").unwrap();
        
        // Calculate 4a^3 + 27b^2
        let a_cubed = FStark::a().square() * FStark::a();
        let b_squared = FStark::b().square();
        
        let discriminant = (four * a_cubed) + (twenty_seven * b_squared);
        
        // Check 4a^3 + 27b^2 != 0
        assert_ne!(discriminant, FStark::ZERO, "Curve is singular.");
    }

    #[test]
    fn test_generator_on_curve() {
        let g: Point<FStark> = Point::generator();
        assert_ne!(g, Point::Infinity, "Generator is the Identity");
        // Check the Equation: Does y^2 = x^3 + ax + b \pmod p?
        assert_eq!(FStark::g_y().square(), FStark::g_x().square() * FStark::g_x() + FStark::a() * FStark::g_x() + FStark::b(), "Generator not on curve");
        // Check the Order: Does $[n]P = O?
        let table = FStark::power_of_two_table();
        assert_eq!( Point::mul_gen(&FStark::n(),&table) , Point::Infinity, "Generator not of the correct order");
        // also check that [n-1]P != O
        let n_minus_1 = FStark::n() - BigUint::from(1u128);
        assert_ne!( Point::mul_gen(&n_minus_1,&table) , Point::Infinity, "[n-1]P = O which is not supposed to happen");
    }

    #[test]
    fn test_addition_identity() {
        let g: Point<FStark> = Point::generator();
        let res = g.add(&Point::Infinity);
        assert_eq!(res, g);
        let res2 = Point::Infinity.add(&g);
        assert_eq!(res2, g);
    }

    //// FP TESTS NOW

    #[test]
    fn test_fp_curve_is_non_singular() {
        let four = Fp::from_str_vartime("4").unwrap();
        let twenty_seven = Fp::from_str_vartime("27").unwrap();
        
        // Calculate 4a^3 + 27b^2 using Fp constants
        let a = Fp::a();
        let b = Fp::b();
        let a_cubed = a.square() * a;
        let b_squared = b.square();
        
        let discriminant = (four * a_cubed) + (twenty_seven * b_squared);
        
        // Check 4a^3 + 27b^2 != 0
        assert_ne!(discriminant, Fp::ZERO, "NIST P-256 Curve is singular.");
    }

    #[test]
    fn test_fp_generator_on_curve() {
        let g: Point<Fp> = Point::generator();
        assert_ne!(g, Point::Infinity, "Fp Generator is the Identity");

        // Check the Equation: y^2 = x^3 + ax + b
        let lhs = Fp::g_y().square();
        let rhs = Fp::g_x().square() * Fp::g_x() + Fp::a() * Fp::g_x() + Fp::b();
        assert_eq!(lhs, rhs, "Fp Generator not on curve");

        // Check the Order: [n]P = O
        let table = Fp::power_of_two_table();
        assert_eq!(Point::mul_gen(&Fp::n(),&table), Point::Infinity, "Fp Generator not of the correct order");

        // Check [n-1]P != O
        let n_minus_1 = Fp::n() - BigUint::from(1u128);
        assert_ne!(Point::mul_gen(&n_minus_1,&table), Point::Infinity, "[n-1]G = O for Fp, which is incorrect");
    }

    #[test]
    fn test_fp_addition_identity() {
        let g: Point<Fp> = Point::generator();
        let res = g.add(&Point::Infinity);
        assert_eq!(res, g, "Fp + Infinity failed");
        
        let res2 = Point::Infinity.add(&g);
        assert_eq!(res2, g, "Infinity + Fp failed");
    }

    

    fn test_associativity_generic<F: CurveConfig>(iterations: usize) {
        let mut rng = rand::thread_rng();
        let n = F::n();
        let table = F::power_of_two_table();

        for _ in 0..iterations {
            // 1. Sample three random scalars in range [0, n)
            let a_raw = rng.gen_b_range(&BigUint::from(0u8), &n);
            let b_raw = rng.gen_b_range(&BigUint::from(0u8), &n);
            let c_raw = rng.gen_b_range(&BigUint::from(0u8), &n);

            // 2. Compute Points A, B, and C via scalar multiplication
            let p_a = Point::mul_gen(&a_raw,&table);
            let p_b = Point::mul_gen(&b_raw,&table);
            let p_c = Point::mul_gen(&c_raw,&table);

            // 3. Check Associativity: (A + B) + C == A + (B + C)
            let lhs = p_a.add(&p_b).add(&p_c);
            let rhs = p_a.add(&p_b.add(&p_c));

            assert_eq!(lhs, rhs, "Associativity failed for curve points");
        }
    }

    #[test]
    fn test_stark_addition_associativity() {
        test_associativity_generic::<FStark>(NBR_ITS);
    }

    #[test]
    fn test_fp_addition_associativity() {
        test_associativity_generic::<Fp>(NBR_ITS);
    }

    // MAYBE TEST INVERSE TOO
    fn test_inverse_generic<F: CurveConfig>(iterations : usize) {
        let mut rng = rand::thread_rng();
        let n = F::n();

        let table = F::power_of_two_table();

        for _ in 0..iterations {
            let a_raw = rng.gen_b_range(&BigUint::from(0u8), &n);
            let p_a = Point::mul_gen(&a_raw,&table);
            // 3. Check Associativity: A + (-A) === O
            let lhs = p_a.add(&(p_a.invert()));

            assert_eq!(lhs, Point::Infinity, "Associativity failed for curve points");
        }
    }

    #[test]
    fn test_inverse_stark() {
        test_inverse_generic::<FStark>(NBR_ITS);
    }

    #[test]
    fn test_inverse_fp() {
        test_inverse_generic::<Fp>(NBR_ITS);
    }

    fn test_mul_generic<F: CurveConfig>(iterations: usize) {
        let mut rng = rand::thread_rng();
        let n = F::n();
        let g = Point::generator();

        let table = F::power_of_two_table();

        for _ in 0..iterations {
            // 1. Sample three random scalars in range [0, n)
            let a = rng.gen_b_range(&BigUint::from(0u8), &n);

            // 2. Compute [a]G via mul_gen
            let p_a_gen = Point::mul_gen(&a,&table);

            // 3. Compute [a]G via mul
            let p_a = Point::mul(&g, &a);

            // 4. Check for equality
            assert_eq!(p_a_gen, p_a, "Both point multiplication methods should be identical");
        }
    }
    #[test]
    fn test_mul_stark() {
        test_mul_generic::<FStark>(NBR_ITS);
    }

    #[test]
    fn test_mul_fp() {
        test_mul_generic::<Fp>(NBR_ITS);
    }
}

