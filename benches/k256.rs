use criterion::{criterion_group, criterion_main, Criterion};
use ecdsa::signature::{DigestVerifier, PrehashSignature as PrehashSignatureT};
use k256::ecdsa::{digest::Digest, signature::DigestSigner, Signature, SigningKey, VerifyingKey};
use rand::rngs::OsRng;

fn sign_verify(c: &mut Criterion) {
    let mut group = c.benchmark_group("k256");

    let digest = <Signature as PrehashSignatureT>::Digest::new();
    let digest = digest.chain(b"HelloWorld");

    let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
    let verify_key = VerifyingKey::from(&signing_key); // Serialize with `::to_encoded_point()`

    group.bench_function("sign", |b| {
        b.iter(|| {
            let _sig: Signature = signing_key.sign_digest(digest.clone());
        })
    });

    let sig: Signature = signing_key.sign_digest(digest.clone());
    group.bench_function("verify", |b| {
        b.iter(|| {
            let _res = verify_key.verify_digest(digest.clone(), &sig).is_ok();
        })
    });
}

criterion_group!(benches, sign_verify);
criterion_main!(benches);
