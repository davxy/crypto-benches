mod ark_ec_ring_vrf {
    use ark_vrf::{
        ring::{Prover, Verifier},
        suites::bandersnatch::{Input, Public, RingSetup, Secret},
    };

    const RING_SIZE: usize = 100;

    struct TestContext {
        params: RingSetup,
        pks: Vec<Public>,
        sk: Secret,
        sk_idx: usize,
    }

    fn secret_from_u32(value: u32) -> Secret {
        let mut seed = [0; 32];
        seed[0..4].copy_from_slice(&value.to_le_bytes());
        Secret::from_seed(seed)
    }

    impl TestContext {
        pub fn new() -> Self {
            let params = RingSetup::from_seed_insecure(RING_SIZE, [0; 32]);
            let pks: Vec<_> = (0..RING_SIZE)
                .map(|i| secret_from_u32(i as u32).public())
                .collect();
            let sk = secret_from_u32(3);
            Self {
                params,
                pks,
                sk,
                sk_idx: 3,
            }
        }
    }

    fn vrf_input_point(data: &[u8]) -> Input {
        Input::new(data).unwrap()
    }

    #[test]
    fn sign_verify() {
        let ctx = TestContext::new();

        let input = vrf_input_point(b"foo");
        let io = ctx.sk.vrf_io(input);

        // Backend currently requires the wrapped type (plain affine points)
        let pts: Vec<_> = ctx.pks.iter().map(|pk| pk.point()).collect();

        // Proof construction
        let prover_key = ctx.params.prover_key(&pts).unwrap();
        let ring_ctx = ctx.params.ring_context();
        let prover = ring_ctx.ring_prover(prover_key, ctx.sk_idx);
        let proof = ctx.sk.prove(io, b"aux", &prover);

        let verifier_key = ctx.params.verifier_key(&pts).unwrap();
        let verifier = ring_ctx.ring_verifier(verifier_key);

        assert!(Public::verify(io, b"aux", &proof, &verifier).is_ok());
    }
}
