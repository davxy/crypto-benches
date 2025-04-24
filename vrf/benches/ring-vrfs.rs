use ark_vrf::reexports::ark_serialize::Compress;
use criterion::{criterion_group, criterion_main, Criterion};
use utils::run_bench;

mod ark_vrf_bandersnatch_ed {
    use ark_vrf::reexports::ark_serialize::{
        CanonicalDeserialize, CanonicalSerialize, Compress, Validate,
    };
    use ark_vrf::ring::{Prover, Verifier};
    use ark_vrf::suites::bandersnatch::*;

    struct TestContext {
        params: RingProofParams,
        pks: Vec<Public>,
        sk: Secret,
        sk_idx: usize,
    }

    fn secret_from_u32(value: u32) -> Secret {
        let mut seed = [0; 32];
        seed[0..4].copy_from_slice(&value.to_le_bytes());
        Secret::from_seed(&seed)
    }

    impl TestContext {
        pub fn new(ring_size: usize) -> Self {
            let ctx = RingProofParams::from_seed(ring_size, [0; 32]);
            let ring_size = ctx.max_ring_size();
            let pks: Vec<_> = (0..ring_size)
                .map(|i| secret_from_u32(i as u32).public())
                .collect();
            let sk = secret_from_u32(3);
            Self {
                params: ctx,
                pks,
                sk,
                sk_idx: 3,
            }
        }
    }

    pub fn deserialize_params(compress: Compress, ring_size: usize) -> impl Fn() {
        println!(
            "ring_size = {}, pcs_size = {}",
            ring_size,
            ark_vrf::ring::pcs_domain_size::<BandersnatchSha512Ell2>(ring_size)
        );

        let ctx = TestContext::new(ring_size);
        let mut buf = vec![];
        ctx.params.serialize_with_mode(&mut buf, compress).unwrap();
        move || {
            let _params =
                RingProofParams::deserialize_with_mode(&mut &buf[..], compress, Validate::No)
                    .unwrap();
        }
    }

    pub fn make_prover_key(ring_size: usize) -> impl Fn() {
        let ctx = TestContext::new(ring_size);

        let pks: Vec<_> = ctx.pks.iter().map(|pk| pk.0).collect();
        move || {
            let _prover_key = ctx.params.prover_key(&pks);
        }
    }

    pub fn make_prover(ring_size: usize) -> impl Fn() {
        let ctx = TestContext::new(ring_size);
        let pks: Vec<_> = ctx.pks.iter().map(|pk| pk.0).collect();

        move || {
            let prover_key = ctx.params.prover_key(&pks);
            let _prover = ctx.params.prover(prover_key, ctx.sk_idx);
        }
    }

    pub fn prove(ring_size: usize) -> impl Fn() {
        let ctx = TestContext::new(ring_size);

        let input = Input::new(b"hello").unwrap();
        let output = ctx.sk.output(input);

        let pks: Vec<_> = ctx.pks.iter().map(|pk| pk.0).collect();
        let prover_key = ctx.params.prover_key(&pks);
        let prover = ctx.params.prover(prover_key, ctx.sk_idx);

        move || {
            let _proof = ctx.sk.prove(input, output, b"foo", &prover);
        }
    }

    pub fn make_verifier_key(ring_size: usize) -> impl Fn() {
        let ctx = TestContext::new(ring_size);
        let pks: Vec<_> = ctx.pks.iter().map(|pk| pk.0).collect();

        move || {
            let _verifier_key = ctx.params.verifier_key(&pks);
        }
    }

    pub fn make_verifier(ring_size: usize) -> impl Fn() {
        let ctx = TestContext::new(ring_size);
        let pks: Vec<_> = ctx.pks.iter().map(|pk| pk.0).collect();
        let verifier_key = ctx.params.verifier_key(&pks);

        move || {
            let _verifier = ctx.params.verifier(verifier_key.clone());
        }
    }

    pub fn verify(ring_size: usize) -> impl Fn() {
        let ctx = TestContext::new(ring_size);

        let input = Input::new(b"hello").unwrap();
        let output = ctx.sk.output(input);

        let pks: Vec<_> = ctx.pks.iter().map(|pk| pk.0).collect();

        let prover_key = ctx.params.prover_key(&pks);
        let prover = ctx.params.prover(prover_key, ctx.sk_idx);
        let proof = ctx.sk.prove(input, output, b"foo", &prover);

        let verifier_key = ctx.params.verifier_key(&pks);
        let verifier = ctx.params.verifier(verifier_key);

        move || {
            let _result = Public::verify(input, output, b"foo", &proof, &verifier).unwrap();
        }
    }
}

mod ark_vrf_bandersnatch_ws {
    use ark_vrf::ring::{Prover, Verifier};
    use ark_vrf::suites::bandersnatch_sw::*;

    struct TestContext {
        ctx: RingProofParams,
        pks: Vec<Public>,
        sk: Secret,
        sk_idx: usize,
    }

    fn secret_from_u32(value: u32) -> Secret {
        let mut seed = [0; 32];
        seed[0..4].copy_from_slice(&value.to_le_bytes());
        Secret::from_seed(&seed)
    }

    impl TestContext {
        pub fn new(ring_size: usize) -> Self {
            let ctx = RingProofParams::from_seed(ring_size, [0; 32]);
            let ring_size = ctx.max_ring_size();
            let pks: Vec<_> = (0..ring_size)
                .map(|i| secret_from_u32(i as u32).public())
                .collect();
            let sk = secret_from_u32(3);
            Self {
                ctx,
                pks,
                sk,
                sk_idx: 3,
            }
        }
    }

    pub fn make_prover_key(ring_size: usize) -> impl Fn() {
        let ctx = TestContext::new(ring_size);
        let pks: Vec<_> = ctx.pks.iter().map(|pk| pk.0).collect();

        move || {
            let _prover_key = ctx.ctx.prover_key(&pks);
        }
    }

    pub fn make_prover(ring_size: usize) -> impl Fn() {
        let ctx = TestContext::new(ring_size);
        let pks: Vec<_> = ctx.pks.iter().map(|pk| pk.0).collect();

        move || {
            let prover_key = ctx.ctx.prover_key(&pks);
            let _prover = ctx.ctx.prover(prover_key, ctx.sk_idx);
        }
    }

    pub fn prove(ring_size: usize) -> impl Fn() {
        let ctx = TestContext::new(ring_size);

        let input = Input::new(b"hello").unwrap();
        let output = ctx.sk.output(input);

        let pks: Vec<_> = ctx.pks.iter().map(|pk| pk.0).collect();
        let prover_key = ctx.ctx.prover_key(&pks);
        let prover = ctx.ctx.prover(prover_key, ctx.sk_idx);

        move || {
            let _proof = ctx.sk.prove(input, output, b"foo", &prover);
        }
    }

    pub fn make_verifier_key(ring_size: usize) -> impl Fn() {
        let ctx = TestContext::new(ring_size);
        let pks: Vec<_> = ctx.pks.iter().map(|pk| pk.0).collect();

        move || {
            let _verifier_key = ctx.ctx.verifier_key(&pks);
        }
    }

    pub fn make_verifier(ring_size: usize) -> impl Fn() {
        let ctx = TestContext::new(ring_size);
        let pks: Vec<_> = ctx.pks.iter().map(|pk| pk.0).collect();
        let verifier_key = ctx.ctx.verifier_key(&pks);

        move || {
            let _verifier = ctx.ctx.verifier(verifier_key.clone());
        }
    }

    pub fn verify(ring_size: usize) -> impl Fn() {
        let ctx = TestContext::new(ring_size);

        let input = Input::new(b"hello").unwrap();
        let output = ctx.sk.output(input);

        let pks: Vec<_> = ctx.pks.iter().map(|pk| pk.0).collect();

        let prover_key = ctx.ctx.prover_key(&pks);
        let prover = ctx.ctx.prover(prover_key, ctx.sk_idx);
        let proof = ctx.sk.prove(input, output, b"foo", &prover);

        let verifier_key = ctx.ctx.verifier_key(&pks);
        let verifier = ctx.ctx.verifier(verifier_key);

        move || {
            let _result = Public::verify(input, output, b"foo", &proof, &verifier).unwrap();
        }
    }
}

fn vrfs(c: &mut Criterion) {
    const RING_SIZE: usize = 1023;

    {
        let mut group = c.benchmark_group("deserialize-params");
        run_bench(
            "ark-vrf-bandersnatch-ed-uncompressed",
            &mut group,
            ark_vrf_bandersnatch_ed::deserialize_params(Compress::No, RING_SIZE),
        );
        run_bench(
            "ark-vrf-bandersnatch-ed-compressed",
            &mut group,
            ark_vrf_bandersnatch_ed::deserialize_params(Compress::Yes, RING_SIZE),
        );
    }

    {
        let mut group = c.benchmark_group("make-prover-key");
        run_bench(
            "ark-vrf-bandersnatch-ed",
            &mut group,
            ark_vrf_bandersnatch_ed::make_prover_key(RING_SIZE),
        );
        run_bench(
            "ark-vrf-bandersnatch-ws",
            &mut group,
            ark_vrf_bandersnatch_ws::make_prover_key(RING_SIZE),
        );
    }

    {
        let mut group = c.benchmark_group("make-prover");
        run_bench(
            "ark-vrf-bandersnatch-ed",
            &mut group,
            ark_vrf_bandersnatch_ed::make_prover(RING_SIZE),
        );
        run_bench(
            "ark-vrf-bandersnatch-ws",
            &mut group,
            ark_vrf_bandersnatch_ws::make_prover(RING_SIZE),
        );
    }

    {
        let mut group = c.benchmark_group("prove");
        run_bench(
            "ark-vrf-bandersnatch-ed",
            &mut group,
            ark_vrf_bandersnatch_ed::prove(RING_SIZE),
        );
        run_bench(
            "ark-vrf-bandersnatch-ws",
            &mut group,
            ark_vrf_bandersnatch_ws::prove(RING_SIZE),
        );
    }

    {
        let mut group = c.benchmark_group("make-verifier-key");
        run_bench(
            "ark-vrf-bandersnatch-ed",
            &mut group,
            ark_vrf_bandersnatch_ed::make_verifier_key(RING_SIZE),
        );
        run_bench(
            "ark-vrf-bandersnatch-ws",
            &mut group,
            ark_vrf_bandersnatch_ws::make_verifier_key(RING_SIZE),
        );
    }

    {
        let mut group = c.benchmark_group("make-verifier");
        run_bench(
            "ark-vrf-bandersnatch-ed",
            &mut group,
            ark_vrf_bandersnatch_ed::make_verifier(RING_SIZE),
        );
        run_bench(
            "ark-vrf-bandersnatch-ws",
            &mut group,
            ark_vrf_bandersnatch_ws::make_verifier(RING_SIZE),
        );
    }

    {
        let mut group = c.benchmark_group("verify");
        run_bench(
            "ark-vrf-bandersnatch-ed",
            &mut group,
            ark_vrf_bandersnatch_ed::verify(RING_SIZE),
        );
        run_bench(
            "ark-vrf-bandersnatch-ws",
            &mut group,
            ark_vrf_bandersnatch_ws::verify(RING_SIZE),
        );
    }
}

criterion_group!(benches, vrfs);
criterion_main!(benches);
