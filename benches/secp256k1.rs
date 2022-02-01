use criterion::{criterion_group, criterion_main, Criterion};
use secp256k1::{rand::rngs::OsRng, Message, Secp256k1};

fn sign_verify(c: &mut Criterion) {
    let mut group = c.benchmark_group("secp256k1");

    let secp = Secp256k1::new();
    let hash = Message::from_slice(&[0; 32]).unwrap();

    let mut rng = OsRng::new().unwrap();
    let (secret_key, public_key) = secp.generate_keypair(&mut rng);

    group.bench_function("sign", |b| {
        b.iter(|| {
            let _sig = secp.sign_ecdsa(&hash, &secret_key);
        })
    });

    let sig = secp.sign_ecdsa(&hash, &secret_key);
    group.bench_function("verify", |b| {
        b.iter(|| {
            secp.verify_ecdsa(&hash, &sig, &public_key).unwrap();
        })
    });
}

criterion_group!(benches, sign_verify);
criterion_main!(benches);
