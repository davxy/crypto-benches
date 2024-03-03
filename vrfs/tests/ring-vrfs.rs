mod bandersnatch_ring_vrf {
    use bandersnatch_vrfs::{ring::KZG, IntoVrfInput, Message, PublicKey, SecretKey, Transcript};

    struct TestContext {
        kzg: KZG,
        pks: Vec<PublicKey>,
        sk: SecretKey,
        sk_idx: usize,
    }

    fn secret_from_u32(value: u32) -> SecretKey {
        let mut seed = [0; 32];
        seed[0..4].copy_from_slice(&value.to_le_bytes());
        SecretKey::from_seed(&seed)
    }

    impl TestContext {
        pub fn new(domain_size: u32) -> Self {
            let kzg = KZG::testing_kzg_setup([0; 32], domain_size);
            let ring_size = kzg.max_keyset_size();
            let pks: Vec<_> = (0..ring_size)
                .map(|i| secret_from_u32(i as u32).to_public())
                .collect();
            let sk = secret_from_u32(3);
            Self {
                kzg,
                pks,
                sk,
                sk_idx: 3,
            }
        }
    }

    #[test]
    fn sign() {
        let ctx = TestContext::new(1024);

        let transcript = Transcript::new_labeled(b"label");
        let input = Message {
            domain: b"domain",
            message: b"message",
        }
        .into_vrf_input();
        let inout = ctx.sk.vrf_inout(input);

        let pks: Vec<_> = ctx.pks.iter().map(|pk| pk.0).collect();
        let prover_key = ctx.kzg.prover_key(pks.clone());
        let prover = ctx.kzg.init_ring_prover(prover_key, ctx.sk_idx);

        let prover_wrap = bandersnatch_vrfs::RingProver {
            ring_prover: &prover,
            secret: &ctx.sk,
        };
        let _sig = prover_wrap.sign_ring_vrf(transcript.clone(), &[inout]);
    }
}
