use criterion::{criterion_group, criterion_main, Criterion};
use utils::run_bench;

mod rustcrypto_k256 {
    use ecdsa::signature::{DigestVerifier, PrehashSignature as PrehashSignatureT};
    use k256::ecdsa::{
        digest::Digest, signature::DigestSigner, Signature, SigningKey, VerifyingKey,
    };
    use rand::rngs::OsRng;

    pub fn sign() -> impl Fn() {
        let digest = <Signature as PrehashSignatureT>::Digest::new();
        let digest = digest.chain(b"HelloWorld");
        let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
                                                          //
        move || {
            let _: Signature = signing_key.sign_digest(digest.clone());
        }
    }

    pub fn verify() -> impl Fn() {
        let digest = <Signature as PrehashSignatureT>::Digest::new();
        let digest = digest.chain(b"HelloWorld");
        let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
        let sig: Signature = signing_key.sign_digest(digest.clone());
        let verify_key = VerifyingKey::from(&signing_key); // Serialize with `::to_encoded_point()`
                                                           //
        move || {
            let _ = verify_key.verify_digest(digest.clone(), &sig).is_ok();
        }
    }
}

mod libsecp256k1 {
    use libsecp256k1::{Message, PublicKey, SecretKey};
    use rand::rngs::OsRng;

    pub fn sign() -> impl Fn() {
        let hash_raw: [u8; 32] = [0; 32];
        let message = Message::parse_slice(&hash_raw).unwrap();
        let secret_key = SecretKey::random(&mut OsRng);

        move || {
            let _ = libsecp256k1::sign(&message, &secret_key).0;
        }
    }

    pub fn verify() -> impl Fn() {
        let hash_raw: [u8; 32] = [0; 32];
        let message = Message::parse_slice(&hash_raw).unwrap();
        let secret_key = SecretKey::random(&mut OsRng);
        let public_key = PublicKey::from_secret_key(&secret_key);
        let sig = libsecp256k1::sign(&message, &secret_key).0;

        move || {
            let _ = libsecp256k1::verify(&message, &sig, &public_key);
        }
    }
}

mod secp256k1 {
    use secp256k1::{rand::rngs::OsRng, Message, Secp256k1};

    pub fn sign() -> impl Fn() {
        let secp = Secp256k1::new();
        let hash = Message::from_slice(&[0; 32]).unwrap();
        let mut rng = OsRng::new().unwrap();
        let (secret_key, _) = secp.generate_keypair(&mut rng);

        move || {
            let _ = secp.sign_ecdsa(&hash, &secret_key);
        }
    }

    pub fn verify() -> impl Fn() {
        let secp = Secp256k1::new();
        let hash = Message::from_slice(&[0; 32]).unwrap();
        let mut rng = OsRng::new().unwrap();
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);
        let sig = secp.sign_ecdsa(&hash, &secret_key);

        move || {
            let _ = secp.verify_ecdsa(&hash, &sig, &public_key).unwrap();
        }
    }
}

fn k256(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("k256-sign");
        run_bench("rust-crypto", &mut group, rustcrypto_k256::sign());
        run_bench("libsecp256k1", &mut group, libsecp256k1::sign());
        run_bench("secp256k1", &mut group, secp256k1::sign());
    }
    {
        let mut group = c.benchmark_group("k256-verify");
        run_bench("rust-crypto", &mut group, rustcrypto_k256::verify());
        run_bench("libsecp256k1", &mut group, libsecp256k1::verify());
        run_bench("secp256k1", &mut group, secp256k1::verify());
    }
}

criterion_group!(benches, k256);
criterion_main!(benches);
