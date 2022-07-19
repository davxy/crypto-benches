use secp256k1::{rand::rngs::OsRng, Message, Secp256k1};

#[test]
fn secp256k1_sign_verify_prehash() {
    let secp = Secp256k1::new();
    let hash = Message::from_slice(&[0; 32]).unwrap();

    let mut rng = OsRng::new().unwrap();
    let (secret_key, public_key) = secp.generate_keypair(&mut rng);

    let sig = secp.sign_ecdsa(&hash, &secret_key);

    assert!(secp.verify_ecdsa(&hash, &sig, &public_key).is_ok());
}
