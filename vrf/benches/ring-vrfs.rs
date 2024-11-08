use criterion::{criterion_group, criterion_main, Criterion};
use utils::run_bench;

mod ark_ec_vrfs_bandersnatch_ed {
    use ark_ec_vrfs::ring::{Prover, Verifier};
    use ark_ec_vrfs::suites::bandersnatch::edwards::*;

    struct TestContext {
        ctx: RingContext,
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
            let ctx = RingContext::from_seed(ring_size, [0; 32]);
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

mod ark_ec_vrfs_bandersnatch_ws {
    use ark_ec_vrfs::ring::{Prover, Verifier};
    use ark_ec_vrfs::suites::bandersnatch::weierstrass::*;

    struct TestContext {
        ctx: RingContext,
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
            let ctx = RingContext::from_seed(ring_size, [0; 32]);
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

mod bandersnatch_vrfs {
    use bandersnatch_vrfs::{ring::KZG, IntoVrfInput, Message, PublicKey, SecretKey, Transcript};

    // Domain size from ring size
    fn domain_size(ring_size: usize) -> usize {
        const RING_DOMAIN_OVERHEAD: usize = 257;
        1 << ark_std::log2(ring_size + RING_DOMAIN_OVERHEAD)
    }

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
        pub fn new(ring_size: usize) -> Self {
            let domain_size = domain_size(ring_size);
            let kzg = KZG::testing_kzg_setup([0; 32], domain_size as u32);
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

    pub fn make_prover_key(ring_size: usize) -> impl Fn() {
        let ctx = TestContext::new(ring_size);
        let pks: Vec<_> = ctx.pks.iter().map(|pk| pk.0).collect();

        move || {
            let _prover_key = ctx.kzg.prover_key(pks.clone());
        }
    }

    pub fn make_prover(ring_size: usize) -> impl Fn() {
        let ctx = TestContext::new(ring_size);
        let pks: Vec<_> = ctx.pks.iter().map(|pk| pk.0).collect();

        move || {
            let prover_key = ctx.kzg.prover_key(pks.clone());
            let _prover = ctx.kzg.init_ring_prover(prover_key, ctx.sk_idx);
        }
    }

    pub fn prove(ring_size: usize) -> impl Fn() {
        let ctx = TestContext::new(ring_size);

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

        move || {
            let prover_wrap = bandersnatch_vrfs::RingProver {
                ring_prover: &prover,
                secret: &ctx.sk,
            };
            let _sig = prover_wrap.sign_ring_vrf(transcript.clone(), &[inout]);
        }
    }

    pub fn make_verifier_key(ring_size: usize) -> impl Fn() {
        let ctx = TestContext::new(ring_size);
        let pks: Vec<_> = ctx.pks.iter().map(|pk| pk.0).collect();

        move || {
            let _verifier_key = ctx.kzg.verifier_key(pks.clone());
        }
    }

    pub fn make_verifier(ring_size: usize) -> impl Fn() {
        let ctx = TestContext::new(ring_size);
        let pks: Vec<_> = ctx.pks.iter().map(|pk| pk.0).collect();
        let verifier_key = ctx.kzg.verifier_key(pks.clone());

        move || {
            let _verifier = ctx.kzg.init_ring_verifier(verifier_key.clone());
        }
    }

    pub fn verify(ring_size: usize) -> impl Fn() {
        let ctx = TestContext::new(ring_size);

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
        let sig = prover_wrap.sign_ring_vrf(transcript.clone(), &[inout]);

        let verifier_key = ctx.kzg.verifier_key(pks.clone());
        let verifier = ctx.kzg.init_ring_verifier(verifier_key);

        let inputs = core::iter::once(input);
        move || {
            let _inout = bandersnatch_vrfs::RingVerifier(&verifier)
                .verify_ring_vrf(transcript.clone(), inputs.clone(), &sig)
                .unwrap();
        }
    }
}

fn vrfs(c: &mut Criterion) {
    const RING_SIZE: usize = 1023;

    {
        let mut group = c.benchmark_group("make-prover-key");
        run_bench(
            "bandersnatch-vrfs",
            &mut group,
            bandersnatch_vrfs::make_prover_key(RING_SIZE),
        );
        run_bench(
            "ark-ec-vrfs-bandersnatch-ed",
            &mut group,
            ark_ec_vrfs_bandersnatch_ed::make_prover_key(RING_SIZE),
        );
        run_bench(
            "ark-ec-vrfs-bandersnatch-ws",
            &mut group,
            ark_ec_vrfs_bandersnatch_ws::make_prover_key(RING_SIZE),
        );
    }

    {
        let mut group = c.benchmark_group("make-prover");
        run_bench(
            "bandersnatch-vrfs",
            &mut group,
            bandersnatch_vrfs::make_prover(RING_SIZE),
        );
        run_bench(
            "ark-ec-vrfs-bandersnatch-ed",
            &mut group,
            ark_ec_vrfs_bandersnatch_ed::make_prover(RING_SIZE),
        );
        run_bench(
            "ark-ec-vrfs-bandersnatch-ws",
            &mut group,
            ark_ec_vrfs_bandersnatch_ws::make_prover(RING_SIZE),
        );
    }

    {
        let mut group = c.benchmark_group("prove");
        run_bench(
            "bandersnatch-vrfs",
            &mut group,
            bandersnatch_vrfs::prove(RING_SIZE),
        );
        run_bench(
            "ark-ec-vrfs-bandersnatch-ed",
            &mut group,
            ark_ec_vrfs_bandersnatch_ed::prove(RING_SIZE),
        );
        run_bench(
            "ark-ec-vrfs-bandersnatch-ws",
            &mut group,
            ark_ec_vrfs_bandersnatch_ws::prove(RING_SIZE),
        );
    }

    {
        let mut group = c.benchmark_group("make-verifier-key");
        run_bench(
            "bandersnatch-vrfs",
            &mut group,
            bandersnatch_vrfs::make_verifier_key(RING_SIZE),
        );
        run_bench(
            "ark-ec-vrfs-bandersnatch-ed",
            &mut group,
            ark_ec_vrfs_bandersnatch_ed::make_verifier_key(RING_SIZE),
        );
        run_bench(
            "ark-ec-vrfs-bandersnatch-ws",
            &mut group,
            ark_ec_vrfs_bandersnatch_ws::make_verifier_key(RING_SIZE),
        );
    }

    {
        let mut group = c.benchmark_group("make-verifier");
        run_bench(
            "bandersnatch-vrfs",
            &mut group,
            bandersnatch_vrfs::make_verifier(RING_SIZE),
        );
        run_bench(
            "ark-ec-vrfs-bandersnatch-ed",
            &mut group,
            ark_ec_vrfs_bandersnatch_ed::make_verifier(RING_SIZE),
        );
        run_bench(
            "ark-ec-vrfs-bandersnatch-ws",
            &mut group,
            ark_ec_vrfs_bandersnatch_ws::make_verifier(RING_SIZE),
        );
    }

    {
        let mut group = c.benchmark_group("verify");
        run_bench(
            "bandersnatch-vrfs",
            &mut group,
            bandersnatch_vrfs::verify(RING_SIZE),
        );
        run_bench(
            "ark-ec-vrfs-bandersnatch-ed",
            &mut group,
            ark_ec_vrfs_bandersnatch_ed::verify(RING_SIZE),
        );
        run_bench(
            "ark-ec-vrfs-bandersnatch-ws",
            &mut group,
            ark_ec_vrfs_bandersnatch_ws::verify(RING_SIZE),
        );
    }
}

criterion_group!(benches, vrfs);
criterion_main!(benches);
