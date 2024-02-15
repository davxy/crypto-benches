use criterion::{criterion_group, criterion_main, Criterion};
use utils::{lazy_static::lazy_static, rng::OsRng, run_bench};

const KEY_BITS: usize = 2048;

mod rust_crypto_rsa {
    use super::*;
    use rsa::{Pkcs1v15Encrypt, Pkcs1v15Sign, RsaPrivateKey, RsaPublicKey};

    lazy_static! {
        static ref PRIVATE_KEY: RsaPrivateKey = RsaPrivateKey::new(&mut OsRng, KEY_BITS).unwrap();
        static ref PUBLIC_KEY: RsaPublicKey = RsaPublicKey::from(&*PRIVATE_KEY);
    }

    pub fn sign() -> impl Fn() {
        let private_key: &RsaPrivateKey = &PRIVATE_KEY;
        let dummy_digest = [0; 32];
        let padding = Pkcs1v15Sign::new_unprefixed();

        move || {
            private_key.sign(padding.clone(), &dummy_digest).unwrap();
        }
    }

    pub fn verify() -> impl Fn() {
        let private_key: &RsaPrivateKey = &PRIVATE_KEY;
        let public_key: &RsaPublicKey = &PUBLIC_KEY;
        let dummy_digest = [0; 32];
        let padding = Pkcs1v15Sign::new_unprefixed();

        let signature = private_key.sign(padding.clone(), &dummy_digest).unwrap();

        move || {
            public_key
                .verify(padding.clone(), &dummy_digest, &signature.clone())
                .unwrap();
        }
    }

    pub fn encrypt() -> impl Fn() {
        let public_key: &RsaPublicKey = &PUBLIC_KEY;
        let dummy_data = [0; 32];

        move || {
            public_key
                .encrypt(&mut OsRng, Pkcs1v15Encrypt, &dummy_data)
                .unwrap();
        }
    }

    pub fn decrypt() -> impl Fn() {
        let private_key: &RsaPrivateKey = &PRIVATE_KEY;
        let public_key: &RsaPublicKey = &PUBLIC_KEY;
        let dummy_data = [0; 32];

        let ciphertext = public_key
            .encrypt(&mut OsRng, Pkcs1v15Encrypt, &dummy_data)
            .unwrap();

        move || {
            private_key.decrypt(Pkcs1v15Encrypt, &ciphertext).unwrap();
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
