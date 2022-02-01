use criterion::{criterion_group, criterion_main, Criterion};
use libsecp256k1::{Message, PublicKey, SecretKey};
use rand::rngs::OsRng;

fn sign_verify(c: &mut Criterion) {
    let mut group = c.benchmark_group("libsecp256k1");

    let hash_raw: [u8; 32] = [0; 32];
    let message = Message::parse_slice(&hash_raw).unwrap();

    let secret_key = SecretKey::random(&mut OsRng);
    let public_key = PublicKey::from_secret_key(&secret_key);

    group.bench_function("sign", |b| {
        b.iter(|| {
            let _sig = libsecp256k1::sign(&message, &secret_key).0;
        })
    });

    let sig = libsecp256k1::sign(&message, &secret_key).0;
    group.bench_function("verify", |b| {
        b.iter(|| {
            libsecp256k1::verify(&message, &sig, &public_key);
        })
    });
}

criterion_group!(benches, sign_verify);
criterion_main!(benches);
