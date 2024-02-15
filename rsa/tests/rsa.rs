use utils::{hex, lazy_static, rng::OsRng};

const KEY_BITS: usize = 2048;

mod rust_crypto {
    use super::*;
    use rsa::{Pkcs1v15Encrypt, Pkcs1v15Sign, RsaPrivateKey, RsaPublicKey};

    lazy_static::lazy_static! {
        static ref PRIVATE_KEY: RsaPrivateKey = RsaPrivateKey::new(&mut OsRng, KEY_BITS).unwrap();
        static ref PUBLIC_KEY: RsaPublicKey = RsaPublicKey::from(&*PRIVATE_KEY);
    }

    #[test]
    fn sign_verify() {
        let private_key: &RsaPrivateKey = &PRIVATE_KEY;
        let public_key = &PUBLIC_KEY;
        let dummy_digest = [0; 32];

        let padding = Pkcs1v15Sign::new_unprefixed();
        let signature = private_key.sign(padding.clone(), &dummy_digest).unwrap();

        println!("Signature: {}", hex::encode(&signature));

        let res = public_key.verify(padding, &dummy_digest, &signature);

        assert!(res.is_ok());
    }

    #[test]
    fn encrypt_decrypt() {
        let private_key: &RsaPrivateKey = &PRIVATE_KEY;
        let public_key = &PUBLIC_KEY;
        // 1960 bits (=245 bytes) is the max allowed length with 2028 bit key
        let msg = [0_u8; 245];

        let ciphertext = public_key
            .encrypt(&mut OsRng, Pkcs1v15Encrypt, &msg)
            .unwrap();

        println!("Ciphertext: {}", hex::encode(&ciphertext));

        let msg_dec = private_key.decrypt(Pkcs1v15Encrypt, &ciphertext).unwrap();

        assert_eq!(msg.as_slice(), msg_dec);
    }
}
