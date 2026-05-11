pub mod stark {
    extern crate ff;
    extern crate num_bigint;

    use self::ff::PrimeField;
    use self::num_bigint::BigUint;
    // STARK252, https://docs.starknet.io/learn/protocol/cryptography
    #[derive(PrimeField)]
    #[PrimeFieldModulus = "3618502788666131213697322783095070105623107215331596699973092056135872020481"]
    #[PrimeFieldGenerator = "7"] // Actually not used in this implementation
    #[PrimeFieldReprEndianness = "little"]
    pub struct FStark([u64; 4]);

    impl FStark {
        pub fn a() -> Self {
            FStark::from_str_vartime("1").unwrap()
        }
        pub fn b() -> Self {
            FStark::from_str_vartime("3141592653589793238462643383279502884197169399375105820974944592307816406665").unwrap()
        }
        pub fn g_x() -> Self {
            FStark::from_str_vartime("874739451078007766457464989774322083649278607533249481151382481072868806602").unwrap()
        }
        pub fn g_y() -> Self {
            FStark::from_str_vartime("152666792071518830868575557812948353041420400780739481342941381225525861407").unwrap()
        }
        pub fn n() -> BigUint {
            // base 16 (hex) - note: remove "0x" prefix if present
            BigUint::parse_bytes(b"800000000000010ffffffffffffffffb781126dcae7b2321e66a241adc64d2f", 16).unwrap()
        }
    }
}

pub mod nist {
    extern crate ff;
    extern crate num_bigint;

    use self::ff::PrimeField;
    use self::num_bigint::BigUint;
    // Fp 256, https://std.neuromancer.sk/nist/P-256#
    #[derive(PrimeField)]
    #[PrimeFieldModulus = "115792089210356248762697446949407573530086143415290314195533631308867097853951"]
    #[PrimeFieldGenerator = "7"] // Actually not used in this implementation
    #[PrimeFieldReprEndianness = "little"]
    pub struct Fp([u64; 5]);

    impl Fp {
        pub fn a() -> Self {
            Fp::from_str_vartime("115792089210356248762697446949407573530086143415290314195533631308867097853948").unwrap()
        }
        pub fn b() -> Self {
            Fp::from_str_vartime("41058363725152142129326129780047268409114441015993725554835256314039467401291").unwrap()
        }
        pub fn g_x() -> Self {
            Fp::from_str_vartime("48439561293906451759052585252797914202762949526041747995844080717082404635286").unwrap()
        }
        pub fn g_y() -> Self {
            Fp::from_str_vartime("36134250956749795798585127919587881956611106672985015071877198253568414405109").unwrap()
        }
        pub fn n() -> BigUint {
            // base 16 (hex) - note: remove "0x" prefix if present
            BigUint::parse_bytes(b"ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551", 16).unwrap()
        }
    }
}

