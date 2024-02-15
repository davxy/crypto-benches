mod schnorrkel {
    use merlin::Transcript;
    use rand::rngs::OsRng;
    use schnorrkel::Keypair;

    #[test]
    fn sign_verify_vrf() {
        let secret = Keypair::generate_with(OsRng);
        let public = secret.public;
        let transcript = Transcript::new(b"label");

        let (inout, signature, _) = secret.vrf_sign(transcript.clone());
        let pre_output = inout.to_preout();

        assert!(public
            .vrf_verify(transcript, &pre_output, &signature)
            .is_ok());
    }

    #[test]
    fn vrf_bytes() {
        let secret = Keypair::generate_with(OsRng);
        let transcript = Transcript::new(b"label");
        let inout = secret.vrf_create_hash(transcript);

        let _bytes = inout.make_bytes::<[u8; 32]>(b"out");
    }
}

mod bandersnatch {
    use bandersnatch_vrfs::{IntoVrfInput, Message, SecretKey, ThinVrfSignature, Transcript};

    #[test]
    fn sign_verify() {
        let secret = SecretKey::ephemeral();
        let public = secret.to_public();
        let transcript = Transcript::new_labeled(b"label");
        let input = Message {
            domain: b"domain",
            message: b"message",
        }
        .into_vrf_input();
        let io = secret.vrf_inout(input);

        let signature: ThinVrfSignature<1> = secret.sign_thin_vrf(transcript.clone(), &[io]);

        assert!(public
            .verify_thin_vrf(transcript, core::iter::once(input), &signature)
            .is_ok());
    }

    #[test]
    fn vrf_bytes() {
        let secret = SecretKey::ephemeral();
        let transcript = Transcript::new_labeled(b"label");
        let input = Message {
            domain: b"domain",
            message: b"message",
        }
        .into_vrf_input();
        let inout = secret.vrf_inout(input);

        let _bytes = inout.vrf_output_bytes::<32>(transcript);
    }
}
