mod rust_crypto {
    use k256::ecdsa::{
        signature::{
            digest::{Digest, Update},
            DigestSigner, DigestVerifier, PrehashSignature as PrehashSignatureT, Signer, Verifier,
        },
        Signature, SigningKey, VerifyingKey,
    };
    use rand::rngs::OsRng;

    #[test]
    fn sign_verify() {
        let message = b"HelloWorld";

        let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
        let sig: Signature = signing_key.sign(message);

        let verify_key = VerifyingKey::from(&signing_key); // Serialize with `::to_encoded_point()`
        assert!(verify_key.verify(message, &sig).is_ok());
    }

    #[test]
    fn sign_verify_prehash() {
        let digest = <Signature as PrehashSignatureT>::Digest::new();
        let digest = digest.chain(b"HelloWorld");

        let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
        let sig: Signature = signing_key.sign_digest(digest.clone());

        let verify_key = VerifyingKey::from(&signing_key); // Serialize with `::to_encoded_point()`
        assert!(verify_key.verify_digest(digest, &sig).is_ok());
    }

    #[test]
    fn sign_verify2() {
        use k256::ecdsa::RecoveryId;
        let message = b"HelloWorld";

        let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
        let (sig, _recid) = signing_key.sign_recoverable(message).unwrap();

        println!("LEN: {:?}", sig.to_bytes().len());

        let verify_key = VerifyingKey::from(&signing_key); // Serialize with `::to_encoded_point()`

        RecoveryId::trial_recovery_from_msg(&verify_key, message, &sig).unwrap();
    }
}

mod libsecp256k1 {
    use libsecp256k1::{Message, PublicKey, SecretKey};
    use rand::rngs::OsRng;

    #[test]
    fn sign_verify_prehash() {
        let hash_raw: [u8; 32] = [0; 32];
        let message = Message::parse_slice(&hash_raw).unwrap();

        let secret_key = SecretKey::random(&mut OsRng);
        let public_key = PublicKey::from_secret_key(&secret_key);

        let sig = libsecp256k1::sign(&message, &secret_key).0;

        assert!(libsecp256k1::verify(&message, &sig, &public_key));
    }
}

mod secp256k1 {
    use secp256k1::{rand::rngs::OsRng, Message, Secp256k1};

    #[test]
    fn sign_verify_prehash() {
        let secp = Secp256k1::new();
        let hash = Message::from_digest_slice(&[0; 32]).unwrap();

        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);

        let sig = secp.sign_ecdsa(&hash, &secret_key);

        assert!(secp.verify_ecdsa(&hash, &sig, &public_key).is_ok());
    }
}
