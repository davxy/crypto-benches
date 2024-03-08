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

    pub fn vrf_bytes() -> impl Fn() {
        let secret = Keypair::generate_with(OsRng);
        let transcript = Transcript::new(b"label");
        let inout = secret.vrf_create_hash(transcript.clone());

        move || {
            let _transcript = transcript.clone();
            let _bytes = inout.make_bytes::<[u8; 32]>(b"out");
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

    pub fn vrf_bytes() -> impl Fn() {
        let secret = SecretKey::ephemeral();
        let transcript = Transcript::new_labeled(b"label");
        let input = Message {
            domain: b"domain",
            message: b"message",
        }
        .into_vrf_input();
        let inout = secret.vrf_inout(input);

        move || {
            let _bytes = inout.vrf_output_bytes::<32>(transcript.clone());
        }
    }
}

fn vrfs(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("sign");
        run_bench("schnorrkel", &mut group, schnorrkel::sign());
        run_bench("bandersnatch", &mut group, bandersnatch::sign());
    }
    {
        let mut group = c.benchmark_group("verify");
        run_bench("schnorrkel", &mut group, schnorrkel::verify());
        run_bench("bandersnatch", &mut group, bandersnatch::verify());
    }
    {
        let mut group = c.benchmark_group("vrf-bytes");
        run_bench("schnorrkel", &mut group, schnorrkel::vrf_bytes());
        run_bench("bandersnatch", &mut group, bandersnatch::vrf_bytes());
    }
}

criterion_group!(benches, vrfs);
criterion_main!(benches);
