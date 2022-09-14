use criterion::{criterion_group, criterion_main, Criterion};
use utils::{run_bench, DUMMY_BUF_DATA};

mod cry_benches {
    use super::*;
    use cry_rs::sha256::Sha256;

    pub fn hash() -> impl Fn() {
        move || {
            let mut ctx = Sha256::new();
            ctx.clear();
            ctx.update(&DUMMY_BUF_DATA);
            let _ = ctx.digest();
        }
    }
}

mod rust_crypto_benches {
    use super::*;
    use sha2::{Digest, Sha256};

    pub fn hash() -> impl Fn() {
        move || {
            let mut ctx = Sha256::new();
            ctx.update(&DUMMY_BUF_DATA);
            let _ = ctx.finalize();
        }
    }
}

mod ring_benches {
    use super::*;
    use ring::digest::{self, Context};

    pub fn hash() -> impl Fn() {
        move || {
            let mut ctx = Context::new(&digest::SHA256);
            ctx.update(&DUMMY_BUF_DATA);
            let _ = ctx.finish();
        }
    }
}

fn sha256(c: &mut Criterion) {
    let mut group = c.benchmark_group("sha256");

    run_bench("rust-crypto", &mut group, rust_crypto_benches::hash());
    run_bench("ring", &mut group, ring_benches::hash());
    run_bench("cry", &mut group, cry_benches::hash());
}

criterion_group!(benches, sha256);
criterion_main!(benches);
