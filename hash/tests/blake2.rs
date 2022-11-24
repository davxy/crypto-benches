use utils::{hex, DUMMY_BUF_DATA};

const KEYLEN: usize = 64;
const KEY: [u8; KEYLEN] = [0; KEYLEN];

mod rust_crypto_crate {
    use super::*;
    use blake2::{digest::typenum::U32, Blake2b, Digest};

    #[test]
    pub fn hash_256_incremental() {
        let mut ctx = Blake2b::<U32>::new();
        ctx.update(&DUMMY_BUF_DATA);
        let hash = ctx.finalize().to_vec();

        println!("{}", hex::encode(&hash));
    }

    #[test]
    pub fn hash_256_oneshot() {
        let hash = blake2::Blake2b::<U32>::digest(&DUMMY_BUF_DATA).to_vec();
        println!("{}", hex::encode(&hash));
    }

    #[test]
    pub fn hash_256_keyed() {
        use blake2::{
            digest::{FixedOutput, Update},
            Blake2bMac,
        };

        let mut ctx = Blake2bMac::<U32>::new_with_salt_and_personal(&KEY, &[], &[]).unwrap();
        ctx.update(&DUMMY_BUF_DATA);
        let hash = ctx.finalize_fixed().to_vec();

        println!("MAC: {}", hex::encode(&hash));
    }
}

mod blake2_rfc_crate {
    use super::*;
    use blake2_rfc::blake2b::Blake2b;

    #[test]
    pub fn hash_256_incremental() {
        let mut ctx = Blake2b::new(32);
        ctx.update(&DUMMY_BUF_DATA);
        let hash = ctx.finalize().as_bytes().to_vec();

        println!("{}", hex::encode(&hash));
    }

    #[test]
    pub fn hash_256_oneshot() {
        let hash = blake2_rfc::blake2b::blake2b(32, &[], &DUMMY_BUF_DATA);
        println!("{}", hex::encode(&hash));
    }

    #[test]
    pub fn hash_256_keyed() {
        let hash = blake2_rfc::blake2b::blake2b(32, &KEY, &DUMMY_BUF_DATA);
        println!("MAC: {}", hex::encode(&hash));
    }
}
