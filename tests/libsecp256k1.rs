use libsecp256k1::{Message, PublicKey, SecretKey};
use rand::rngs::OsRng;

#[test]
fn secp256k1_sign_verify_prehash() {
    let hash_raw: [u8; 32] = [0; 32];
    let message = Message::parse_slice(&hash_raw).unwrap();

    let secret_key = SecretKey::random(&mut OsRng);
    let public_key = PublicKey::from_secret_key(&secret_key);

    let sig = libsecp256k1::sign(&message, &secret_key).0;

    assert!(libsecp256k1::verify(&message, &sig, &public_key));
}
