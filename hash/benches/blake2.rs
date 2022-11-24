use criterion::{criterion_group, criterion_main, Criterion};
use utils::{run_bench, DUMMY_BUF_DATA};

mod rust_crypto_benches {
    use super::*;
    use blake2::{
        digest::typenum::{U32, U64},
        Blake2b, Digest,
    };

    pub fn hash_256() -> impl Fn() {
        move || {
            let mut ctx = Blake2b::<U32>::new();
            ctx.update(&DUMMY_BUF_DATA);
            ctx.update(&DUMMY_BUF_DATA);
            ctx.update(&DUMMY_BUF_DATA);
            let _ = ctx.finalize();
        }
    }

    pub fn hash_512() -> impl Fn() {
        move || {
            let mut ctx = Blake2b::<U64>::new();
            ctx.update(&DUMMY_BUF_DATA);
            ctx.update(&DUMMY_BUF_DATA);
            ctx.update(&DUMMY_BUF_DATA);
            let _ = ctx.finalize();
        }
    }
}

mod blake2_rfc_benches {
    use super::*;
    use blake2_rfc::blake2b::Blake2b;

    pub fn hash_256() -> impl Fn() {
        move || {
            let mut ctx = Blake2b::new(32);
            ctx.update(&DUMMY_BUF_DATA);
            ctx.update(&DUMMY_BUF_DATA);
            ctx.update(&DUMMY_BUF_DATA);
            let _ = ctx.finalize();
        }
    }

    pub fn hash_512() -> impl Fn() {
        move || {
            let mut ctx = Blake2b::new(64);
            ctx.update(&DUMMY_BUF_DATA);
            ctx.update(&DUMMY_BUF_DATA);
            ctx.update(&DUMMY_BUF_DATA);
            let _ = ctx.finalize();
        }
    }
}

fn blake2(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("blake2-256");
        run_bench("blake2-rfc", &mut group, blake2_rfc_benches::hash_256());
        run_bench("rust-crypto", &mut group, rust_crypto_benches::hash_256());
    }
    {
        let mut group = c.benchmark_group("blake2-512");
        run_bench("blake2-rfc", &mut group, blake2_rfc_benches::hash_512());
        run_bench("rust-crypto", &mut group, rust_crypto_benches::hash_512());
    }
}

criterion_group!(benches, blake2);
criterion_main!(benches);
