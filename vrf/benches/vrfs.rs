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
    use ark_ec_vrfs::{
        ietf::{Prover, Verifier},
        suites::ed25519::*,
    };
    use ark_std::UniformRand;

    const SEED: &[u8] = b"test";

    fn dummy_input() -> Input {
        let mut rng = ark_std::test_rng();
        let p = AffinePoint::rand(&mut rng);
        Input::from(p)
    }

    pub fn prove() -> impl Fn() {
        let secret = Secret::from_seed(SEED);
        let input = dummy_input();
        let output = secret.output(input);

        move || {
            let _sig = secret.prove(input, output, b"ad");
        }
    }

    pub fn verify() -> impl Fn() {
        let secret = Secret::from_seed(SEED);
        let public = secret.public();
        let input = dummy_input();
        let output = secret.output(input);
        let proof = secret.prove(input, output, b"ad");

        move || {
            let _ = public.verify(input, output, b"ad", &proof);
        }
    }
}

mod ark_ec_vrf_bandersnatch_sha512_ws {
    use ark_ec_vrfs::{
        ietf::{Prover, Verifier},
        suites::bandersnatch::weierstrass::*,
    };
    use ark_std::UniformRand;

    const SEED: &[u8] = b"test";

    fn dummy_input() -> Input {
        let mut rng = ark_std::test_rng();
        let p = AffinePoint::rand(&mut rng);
        Input::from(p)
    }

    pub fn prove() -> impl Fn() {
        let secret = Secret::from_seed(SEED);
        let input = dummy_input();
        let output = secret.output(input);

        move || {
            let _proof = secret.prove(input, output, b"ad");
        }
    }

    pub fn verify() -> impl Fn() {
        let secret = Secret::from_seed(SEED);
        let public = secret.public();
        let input = dummy_input();
        let output = secret.output(input);
        let proof = secret.prove(input, output, b"ad");

        move || {
            let _ = public.verify(input, output, b"ad", &proof);
        }
    }
}

mod ark_ec_vrf_bandersnatch_sha512_ed {
    use ark_ec_vrfs::{
        ietf::{Prover, Verifier},
        suites::bandersnatch::edwards::*,
    };
    use ark_std::UniformRand;

    const SEED: &[u8] = b"test";

    fn dummy_input() -> Input {
        let mut rng = ark_std::test_rng();
        let p = AffinePoint::rand(&mut rng);
        Input::from(p)
    }

    pub fn prove() -> impl Fn() {
        let secret = Secret::from_seed(SEED);
        let input = dummy_input();
        let output = secret.output(input);

        move || {
            let _proof = secret.prove(input, output, b"ad");
        }
    }

    pub fn verify() -> impl Fn() {
        let secret = Secret::from_seed(SEED);
        let public = secret.public();
        let input = dummy_input();
        let output = secret.output(input);
        let proof = secret.prove(input, output, b"ad");

        move || {
            let _ = public.verify(input, output, b"ad", &proof);
        }
    }
}

mod ark_ec_vrf_bandersnatch_blake2_ed {
    use ark_ec_vrfs::{
        ietf::{Prover, Verifier},
        suite_types, Suite,
    };
    use ark_std::UniformRand;

    #[derive(Debug, Clone, Copy)]
    struct BandersnatchBlake2b512;

    impl Suite for BandersnatchBlake2b512 {
        const SUITE_ID: &'static [u8] = &[0x00];
        const CHALLENGE_LEN: usize = 32;

        type Affine = ark_ed_on_bls12_381_bandersnatch::SWAffine;
        type Hasher = blake2::Blake2b512;
        type Codec = ark_ec_vrfs::codec::ArkworksCodec;
    }

    suite_types!(BandersnatchBlake2b512);

    const SEED: &[u8] = b"test";

    fn dummy_input() -> Input {
        let mut rng = ark_std::test_rng();
        let p = AffinePoint::rand(&mut rng);
        Input::from(p)
    }

    pub fn prove() -> impl Fn() {
        let secret = Secret::from_seed(SEED);
        let input = dummy_input();
        let output = secret.output(input);

        move || {
            let _proof = secret.prove(input, output, b"ad");
        }
    }

    pub fn verify() -> impl Fn() {
        let secret = Secret::from_seed(SEED);
        let public = secret.public();
        let input = dummy_input();
        let output = secret.output(input);
        let signature = secret.prove(input, output, b"ad");

        move || {
            let _ = public.verify(input, output, b"ad", &signature);
        }
    }
}

mod bandersnatch_vrfs {
    use bandersnatch_vrfs::{IntoVrfInput, Message, SecretKey, ThinVrfSignature, Transcript};

    pub fn prove() -> impl Fn() {
        let secret = SecretKey::ephemeral();
        let transcript = Transcript::new_labeled(b"label");
        let input = Message {
            domain: b"domain",
            message: b"message",
        }
        .into_vrf_input();
        let io = secret.vrf_inout(input);

        move || {
            let _proof: ThinVrfSignature<1> = secret.sign_thin_vrf(transcript.clone(), &[io]);
        }
    }

    pub fn verify() -> impl Fn() {
        let secret = SecretKey::ephemeral();
        let public = secret.to_public();
        let transcript = Transcript::new_labeled(b"label");
        let input = Message {
            domain: b"domain",
            message: b"message",
        }
        .into_vrf_input();
        let io = secret.vrf_inout(input);
        let proof: ThinVrfSignature<1> = secret.sign_thin_vrf(transcript.clone(), &[io]);

        move || {
            let _res = public.verify_thin_vrf(transcript.clone(), core::iter::once(input), &proof);
        }
    }
}

fn vrfs(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("prove");
        run_bench("schnorrkel", &mut group, schnorrkel::prove());
        run_bench(
            "ark-ec-vrf-ed25519",
            &mut group,
            ark_ec_vrf_ed25519::prove(),
        );
        run_bench(
            "ark-ec-vrf-bandersnatch-sha512-ws",
            &mut group,
            ark_ec_vrf_bandersnatch_sha512_ws::prove(),
        );
        run_bench(
            "ark-ec-vrf-bandersnatch-sha512-ed",
            &mut group,
            ark_ec_vrf_bandersnatch_sha512_ed::prove(),
        );
        run_bench(
            "ark-ec-vrf-bandersnatch-blake2-ed",
            &mut group,
            ark_ec_vrf_bandersnatch_blake2_ed::prove(),
        );
        run_bench("bandersnatch-vrfs", &mut group, bandersnatch_vrfs::prove());
    }
    {
        let mut group = c.benchmark_group("verify");
        run_bench("schnorrkel", &mut group, schnorrkel::verify());
        run_bench(
            "ark-ec-vrf-ed25519",
            &mut group,
            ark_ec_vrf_ed25519::verify(),
        );
        run_bench(
            "ark-ec-vrf-bandersnatch-sha512-ws",
            &mut group,
            ark_ec_vrf_bandersnatch_sha512_ws::verify(),
        );
        run_bench(
            "ark-ec-vrf-bandersnatch-sha512-ed",
            &mut group,
            ark_ec_vrf_bandersnatch_sha512_ed::verify(),
        );
        run_bench(
            "ark-ec-vrf-bandersnatch-blake2-ed",
            &mut group,
            ark_ec_vrf_bandersnatch_blake2_ed::verify(),
        );
        run_bench("bandersnatch-vrfs", &mut group, bandersnatch_vrfs::verify());
    }
}

criterion_group!(benches, vrfs);
criterion_main!(benches);
