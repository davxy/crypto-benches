use criterion::{criterion_group, criterion_main, Criterion};
use utils::run_bench;

mod schnorrkel {
    use merlin::Transcript;
    use rand::rngs::OsRng;
    use schnorrkel::Keypair;

    pub fn prove() -> impl Fn() {
        let secret = Keypair::generate_with(OsRng);
        let transcript = Transcript::new(b"label");

        move || {
            let (_inout, _sig, _) = secret.vrf_sign(transcript.clone());
        }
    }

    pub fn verify() -> impl Fn() {
        let secret = Keypair::generate_with(OsRng);
        let public = secret.public;
        let transcript = Transcript::new(b"label");
        let (inout, signature, _) = secret.vrf_sign(transcript.clone());
        let pre_output = inout.to_preout();

        move || {
            let _res = public.vrf_verify(transcript.clone(), &pre_output, &signature);
        }
    }
}

mod ark_ec_vrf_ed25519 {
    use ark_std::UniformRand;
    use ark_vrf::{
        suites::ed25519::*,
        tiny::{Prover, Verifier},
    };

    const SEED: [u8; 32] = [0; 32];

    fn dummy_input() -> Input {
        let mut rng = ark_std::test_rng();
        let p = AffinePoint::rand(&mut rng);
        Input::from_affine_unchecked(p)
    }

    pub fn prove() -> impl Fn() {
        let secret = Secret::from_seed(SEED);
        let io = secret.vrf_io(dummy_input());

        move || {
            let _sig = secret.prove(io, b"ad");
        }
    }

    pub fn verify() -> impl Fn() {
        let secret = Secret::from_seed(SEED);
        let public = secret.public();
        let io = secret.vrf_io(dummy_input());
        let proof = secret.prove(io, b"ad");

        move || {
            let _ = public.verify(io, b"ad", &proof);
        }
    }
}

mod ark_ec_vrf_bandersnatch_sha512_ws {
    use ark_std::UniformRand;
    use ark_vrf::{
        suites::bandersnatch_sw::*,
        tiny::{Prover, Verifier},
    };

    const SEED: [u8; 32] = [0; 32];

    fn dummy_input() -> Input {
        let mut rng = ark_std::test_rng();
        let p = AffinePoint::rand(&mut rng);
        Input::from_affine_unchecked(p)
    }

    pub fn prove() -> impl Fn() {
        let secret = Secret::from_seed(SEED);
        let io = secret.vrf_io(dummy_input());

        move || {
            let _proof = secret.prove(io, b"ad");
        }
    }

    pub fn verify() -> impl Fn() {
        let secret = Secret::from_seed(SEED);
        let public = secret.public();
        let io = secret.vrf_io(dummy_input());
        let proof = secret.prove(io, b"ad");

        move || {
            let _ = public.verify(io, b"ad", &proof);
        }
    }
}

mod ark_ec_vrf_bandersnatch_sha512_ed {
    use ark_std::UniformRand;
    use ark_vrf::{
        suites::bandersnatch::*,
        tiny::{Prover, Verifier},
    };

    const SEED: [u8; 32] = [0; 32];

    fn dummy_input() -> Input {
        let mut rng = ark_std::test_rng();
        let p = AffinePoint::rand(&mut rng);
        Input::from_affine_unchecked(p)
    }

    pub fn prove() -> impl Fn() {
        let secret = Secret::from_seed(SEED);
        let io = secret.vrf_io(dummy_input());

        move || {
            let _proof = secret.prove(io, b"ad");
        }
    }

    pub fn verify() -> impl Fn() {
        let secret = Secret::from_seed(SEED);
        let public = secret.public();
        let io = secret.vrf_io(dummy_input());
        let proof = secret.prove(io, b"ad");

        move || {
            let _ = public.verify(io, b"ad", &proof);
        }
    }
}

mod ark_ec_vrf_bandersnatch_blake2_ed {
    use ark_std::UniformRand;
    use ark_vrf::{
        suite_types,
        tiny::{Prover, Verifier},
        utils::HashTranscript,
        Suite,
    };

    #[derive(Debug, Clone, Copy)]
    struct BandersnatchBlake2b512;

    impl Suite for BandersnatchBlake2b512 {
        const SUITE_ID: &'static [u8] = b"Bandersnatch-SW-BLAKE2-TAI-bench";

        type Affine = ark_ed_on_bls12_381_bandersnatch::SWAffine;
        type Transcript = HashTranscript<blake2::Blake2b512>;
    }

    suite_types!(BandersnatchBlake2b512);

    const SEED: [u8; 32] = [0; 32];

    fn dummy_input() -> Input {
        let mut rng = ark_std::test_rng();
        let p = AffinePoint::rand(&mut rng);
        Input::from_affine_unchecked(p)
    }

    pub fn prove() -> impl Fn() {
        let secret = Secret::from_seed(SEED);
        let io = secret.vrf_io(dummy_input());

        move || {
            let _proof = secret.prove(io, b"ad");
        }
    }

    pub fn verify() -> impl Fn() {
        let secret = Secret::from_seed(SEED);
        let public = secret.public();
        let io = secret.vrf_io(dummy_input());
        let signature = secret.prove(io, b"ad");

        move || {
            let _ = public.verify(io, b"ad", &signature);
        }
    }
}

fn vrfs(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("prove");
        run_bench("schnorrkel", &mut group, schnorrkel::prove());
        run_bench("ark-vrf-ed25519", &mut group, ark_ec_vrf_ed25519::prove());
        run_bench(
            "ark-vrf-bandersnatch-sha512-ws",
            &mut group,
            ark_ec_vrf_bandersnatch_sha512_ws::prove(),
        );
        run_bench(
            "ark-vrf-bandersnatch-sha512-ed",
            &mut group,
            ark_ec_vrf_bandersnatch_sha512_ed::prove(),
        );
        run_bench(
            "ark-vrf-bandersnatch-blake2-ed",
            &mut group,
            ark_ec_vrf_bandersnatch_blake2_ed::prove(),
        );
    }
    {
        let mut group = c.benchmark_group("verify");
        run_bench("schnorrkel", &mut group, schnorrkel::verify());
        run_bench("ark-vrf-ed25519", &mut group, ark_ec_vrf_ed25519::verify());
        run_bench(
            "ark-vrf-bandersnatch-sha512-ws",
            &mut group,
            ark_ec_vrf_bandersnatch_sha512_ws::verify(),
        );
        run_bench(
            "ark-vrf-bandersnatch-sha512-ed",
            &mut group,
            ark_ec_vrf_bandersnatch_sha512_ed::verify(),
        );
        run_bench(
            "ark-vrf-bandersnatch-blake2-ed",
            &mut group,
            ark_ec_vrf_bandersnatch_blake2_ed::verify(),
        );
    }
}

criterion_group!(benches, vrfs);
criterion_main!(benches);
