use criterion::{criterion_group, criterion_main, Criterion};
use utils::run_bench;

mod schnorrkel {
    use merlin::Transcript;
    use rand::rngs::OsRng;
    use schnorrkel::Keypair;

    pub fn sign() -> impl Fn() {
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

mod ietf_ecvrf {
    use ark_ec::{AffineRepr, CurveGroup};
    use ark_ed_on_bls12_381_bandersnatch as curve;
    use ark_ff::PrimeField;

    type P = curve::EdwardsAffine;
    type Secret = ark_ecvrf::Secret<P>;

    fn make_dummy_point(s: u32) -> P {
        let s = <P as AffineRepr>::ScalarField::from_be_bytes_mod_order(&s.to_be_bytes()[..]);
        (P::generator() * s).into_affine()
    }

    pub fn sign() -> impl Fn() {
        let input = make_dummy_point(3);
        let secret = Secret::from_seed([0u8; 32]);

        move || {
            let _sig = secret.sign(input.into(), b"ad");
        }
    }

    pub fn verify() -> impl Fn() {
        let input = make_dummy_point(3).into();
        let secret = Secret::from_seed([0u8; 32]);
        let public = secret.public();
        let signature = secret.sign(input, b"ad");

        move || {
            let _ = public.verify(input, b"ad", &signature);
        }
    }
}

mod bandersnatch {
    use bandersnatch_vrfs::{IntoVrfInput, Message, SecretKey, ThinVrfSignature, Transcript};

    pub fn sign() -> impl Fn() {
        let secret = SecretKey::ephemeral();
        let transcript = Transcript::new_labeled(b"label");
        let input = Message {
            domain: b"domain",
            message: b"message",
        }
        .into_vrf_input();
        let io = secret.vrf_inout(input);

        move || {
            let _sig: ThinVrfSignature<1> = secret.sign_thin_vrf(transcript.clone(), &[io]);
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
        let signature: ThinVrfSignature<1> = secret.sign_thin_vrf(transcript.clone(), &[io]);

        move || {
            let _res =
                public.verify_thin_vrf(transcript.clone(), core::iter::once(input), &signature);
        }
    }
}

fn vrfs(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("sign");
        run_bench("schnorrkel", &mut group, schnorrkel::sign());
        run_bench("ietf_ecvrf", &mut group, ietf_ecvrf::sign());
        run_bench("bandersnatch", &mut group, bandersnatch::sign());
    }
    {
        let mut group = c.benchmark_group("verify");
        run_bench("schnorrkel", &mut group, schnorrkel::verify());
        run_bench("ietf_ecvrf", &mut group, ietf_ecvrf::verify());
        run_bench("bandersnatch", &mut group, bandersnatch::verify());
    }
}

criterion_group!(benches, vrfs);
criterion_main!(benches);
