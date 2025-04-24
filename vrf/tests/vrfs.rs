mod w3f_schnorrkel {
    use merlin::Transcript;
    use rand::rngs::OsRng;
    use schnorrkel::Keypair;

    #[test]
    fn sign_verify() {
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
