use criterion::{criterion_group, criterion_main, Criterion};
use utils::{lazy_static::lazy_static, rng::OsRng, run_bench};

const KEY_BITS: usize = 2048;

mod rust_crypto_rsa {
    use super::*;
    use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};

    lazy_static! {
        static ref PRIVATE_KEY: RsaPrivateKey = RsaPrivateKey::new(&mut OsRng, KEY_BITS).unwrap();
        static ref PUBLIC_KEY: RsaPublicKey = RsaPublicKey::from(&*PRIVATE_KEY);
    }

    pub fn sign() -> impl Fn() {
        let private_key: &RsaPrivateKey = &PRIVATE_KEY;
        let dummy_digest = [0; 32];

        move || {
            let padding = PaddingScheme::new_pkcs1v15_sign_raw();
            private_key.sign(padding, &dummy_digest).unwrap();
        }
    }

    pub fn verify() -> impl Fn() {
        let private_key: &RsaPrivateKey = &PRIVATE_KEY;
        let public_key: &RsaPublicKey = &PUBLIC_KEY;
        let dummy_digest = [0; 32];

        let padding = PaddingScheme::new_pkcs1v15_sign_raw();
        let signature = private_key.sign(padding, &dummy_digest).unwrap();

        move || {
            let padding = PaddingScheme::new_pkcs1v15_sign_raw();
            public_key
                .verify(padding, &dummy_digest, &signature.clone())
                .unwrap();
        }
    }

    pub fn encrypt() -> impl Fn() {
        let public_key: &RsaPublicKey = &PUBLIC_KEY;
        let dummy_data = [0; 32];

        move || {
            let padding = PaddingScheme::new_pkcs1v15_encrypt();
            public_key
                .encrypt(&mut OsRng, padding, &dummy_data)
                .unwrap();
        }
    }

    pub fn decrypt() -> impl Fn() {
        let private_key: &RsaPrivateKey = &PRIVATE_KEY;
        let public_key: &RsaPublicKey = &PUBLIC_KEY;
        let dummy_data = [0; 32];

        let padding = PaddingScheme::new_pkcs1v15_encrypt();
        let ciphertext = public_key
            .encrypt(&mut OsRng, padding, &dummy_data)
            .unwrap();

        move || {
            let padding = PaddingScheme::new_pkcs1v15_encrypt();
            private_key.decrypt(padding, &ciphertext).unwrap();
        }
    }
}

fn rsa(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("rsa_pkcs1_v15_sign");
        run_bench("rust-crypto", &mut group, rust_crypto_rsa::sign());
    }
    {
        let mut group = c.benchmark_group("rsa_pkcs1_v15_verify");
        run_bench("rust-crypto", &mut group, rust_crypto_rsa::verify());
    }
    {
        let mut group = c.benchmark_group("rsa_pkcs1_v15_encrypt");
        run_bench("rust-crypto", &mut group, rust_crypto_rsa::encrypt());
    }
    {
        let mut group = c.benchmark_group("rsa_pkcs1_v15_decrypt");
        run_bench("rust-crypto", &mut group, rust_crypto_rsa::decrypt());
    }
}

criterion_group!(benches, rsa);
criterion_main!(benches);
