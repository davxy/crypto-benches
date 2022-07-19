use criterion::{criterion_group, criterion_main, Criterion};
use utils::run_bench;

const DATA_SIZE: usize = 4096;

mod cry {
    use super::*;
    use cry_rs::sha256::Sha256;

    pub fn hash() -> impl Fn() {
        let buf = vec![0; DATA_SIZE];

        move || {
            let mut ctx = Sha256::new();
            ctx.clear();
            ctx.update(&buf);
            let _ = ctx.digest();
        }
    }
}

mod rustcrypto {
    use super::*;
    use sha2::{Digest, Sha256};

    pub fn hash() -> impl Fn() {
        let buf = vec![0; DATA_SIZE];

        move || {
            let mut ctx = Sha256::new();
            ctx.update(&buf);
            let _ = ctx.finalize();
        }
    }
}

mod crate_ring {
    use super::*;
    use ring::digest::{self, Context};

    pub fn hash() -> impl Fn() {
        let buf = vec![0; DATA_SIZE];

        move || {
            let mut ctx = Context::new(&digest::SHA256);
            ctx.update(&buf);
            let _ = ctx.finish();
        }
    }
}

fn sha256(c: &mut Criterion) {
    let mut group = c.benchmark_group("sha256");
    run_bench("rust-crypto", &mut group, rustcrypto::hash());
    run_bench("ring", &mut group, crate_ring::hash());
    run_bench("cry", &mut group, cry::hash());
}

criterion_group!(benches, sha256);
criterion_main!(benches);
