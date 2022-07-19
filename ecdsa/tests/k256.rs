use ecdsa::signature::{DigestVerifier, PrehashSignature as PrehashSignatureT};
use k256::{
    ecdsa::digest::Digest,
    ecdsa::{signature::Verifier, VerifyingKey},
    ecdsa::{
        signature::{DigestSigner, Signer},
        Signature, SigningKey,
    },
};
use rand::rngs::OsRng; // requires 'getrandom' feature

#[test]
fn k256_sign_verify() {
    let message = b"HelloWorld";

    let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
    let sig: Signature = signing_key.sign(message);

    let verify_key = VerifyingKey::from(&signing_key); // Serialize with `::to_encoded_point()`
    assert!(verify_key.verify(message, &sig).is_ok());
}

#[test]
fn k256_sign_verify_prehash() {
    let digest = <Signature as PrehashSignatureT>::Digest::new();
    let digest = digest.chain(b"HelloWorld");

    let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
    let sig: Signature = signing_key.sign_digest(digest.clone());

    let verify_key = VerifyingKey::from(&signing_key); // Serialize with `::to_encoded_point()`
    assert!(verify_key.verify_digest(digest, &sig).is_ok());
}
