use criterion::{criterion_group, criterion_main, Criterion};
use utils::run_bench;

mod bandersnatch {
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

    pub fn make_prover_key(domain_size: u32) -> impl Fn() {
        let ctx = TestContext::new(domain_size);

        let pks: Vec<_> = ctx.pks.iter().map(|pk| pk.0).collect();
        move || {
            // TODO: change to take a reference to `pks`.
            // Internally change to take a reference to `pcs_params`
            let _prover_key = ctx.kzg.prover_key(pks.clone());
        }
    }

    pub fn make_prover(domain_size: u32) -> impl Fn() {
        let ctx = TestContext::new(domain_size);

        let pks: Vec<_> = ctx.pks.iter().map(|pk| pk.0).collect();
        move || {
            let prover_key = ctx.kzg.prover_key(pks.clone());
            let _prover = ctx.kzg.init_ring_prover(prover_key, ctx.sk_idx);
        }
    }

    pub fn sign(domain_size: u32) -> impl Fn() {
        let ctx = TestContext::new(domain_size);

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

    pub fn make_verifier_key(domain_size: u32) -> impl Fn() {
        let ctx = TestContext::new(domain_size);

        let pks: Vec<_> = ctx.pks.iter().map(|pk| pk.0).collect();
        move || {
            // TODO: change to take a reference to `pks`.
            // Internally change to take a reference to `pcs_params`
            let _verifier_key = ctx.kzg.verifier_key(pks.clone());
        }
    }

    pub fn make_verifier(domain_size: u32) -> impl Fn() {
        let ctx = TestContext::new(domain_size);
        let pks: Vec<_> = ctx.pks.iter().map(|pk| pk.0).collect();
        move || {
            let verifier_key = ctx.kzg.verifier_key(pks.clone());
            let _verifier = ctx.kzg.init_ring_verifier(verifier_key);
        }
    }

    pub fn verify(domain_size: u32) -> impl Fn() {
        let ctx = TestContext::new(domain_size);

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
    {
        const DOMAIN_SIZE: u32 = 512;
        let mut group = c.benchmark_group("bandersnatch-ring-vrf-512");
        run_bench(
            "make-prover-key",
            &mut group,
            bandersnatch::make_prover_key(DOMAIN_SIZE),
        );
        run_bench(
            "make-prover",
            &mut group,
            bandersnatch::make_prover(DOMAIN_SIZE),
        );
        run_bench("sign", &mut group, bandersnatch::sign(DOMAIN_SIZE));

        run_bench(
            "make-verifier-key",
            &mut group,
            bandersnatch::make_verifier_key(DOMAIN_SIZE),
        );
        run_bench(
            "make-verifier",
            &mut group,
            bandersnatch::make_verifier(DOMAIN_SIZE),
        );
        run_bench("verify", &mut group, bandersnatch::verify(DOMAIN_SIZE));
    }
    {
        const DOMAIN_SIZE: u32 = 1024;
        let mut group = c.benchmark_group("bandersnatch-ring-vrf-1024");
        run_bench(
            "make-prover-key",
            &mut group,
            bandersnatch::make_prover_key(DOMAIN_SIZE),
        );
        run_bench(
            "make-prover",
            &mut group,
            bandersnatch::make_prover(DOMAIN_SIZE),
        );
        run_bench("sign", &mut group, bandersnatch::sign(DOMAIN_SIZE));

        run_bench(
            "make-verifier-key",
            &mut group,
            bandersnatch::make_verifier_key(DOMAIN_SIZE),
        );
        run_bench(
            "make-verifier",
            &mut group,
            bandersnatch::make_verifier(DOMAIN_SIZE),
        );
        run_bench("verify", &mut group, bandersnatch::verify(DOMAIN_SIZE));
    }
    {
        const DOMAIN_SIZE: u32 = 2048;
        let mut group = c.benchmark_group("bandersnatch-ring-vrf-2048");
        run_bench(
            "make-prover-key",
            &mut group,
            bandersnatch::make_prover_key(DOMAIN_SIZE),
        );
        run_bench(
            "make-prover",
            &mut group,
            bandersnatch::make_prover(DOMAIN_SIZE),
        );
        run_bench("sign", &mut group, bandersnatch::sign(DOMAIN_SIZE));

        run_bench(
            "make-verifier-key",
            &mut group,
            bandersnatch::make_verifier_key(DOMAIN_SIZE),
        );
        run_bench(
            "make-verifier",
            &mut group,
            bandersnatch::make_verifier(DOMAIN_SIZE),
        );
        run_bench("verify", &mut group, bandersnatch::verify(DOMAIN_SIZE));
    }
}

criterion_group!(benches, vrfs);
criterion_main!(benches);
