use criterion::{criterion_group, criterion_main, Criterion};
use utils::run_bench;

const NBLOCKS: usize = 1024;

mod rust_crypto_aes {
    use super::*;
    use aes::cipher::KeyInit;
    use aes::cipher::{BlockDecrypt, BlockEncrypt};
    use aes::{Aes128, Block};

    pub fn encrypt() -> impl Fn() {
        let key = [0; 16];
        let ctx = Aes128::new_from_slice(&key).unwrap();

        let mut blocks = Vec::with_capacity(NBLOCKS);
        for _ in 0..NBLOCKS {
            blocks.push(Block::default());
        }

        move || {
            ctx.encrypt_blocks(&mut blocks.clone());
        }
    }

    pub fn decrypt() -> impl Fn() {
        let key = [0; 16];
        let ctx = Aes128::new_from_slice(&key).unwrap();

        let mut blocks = Vec::with_capacity(NBLOCKS);
        for _ in 0..NBLOCKS {
            blocks.push(Block::default());
        }

        move || {
            ctx.decrypt_blocks(&mut blocks.clone());
        }
    }
}

mod cry_aes {
    use super::*;
    use cry_rs::aes::Aes128;

    pub fn encrypt() -> impl FnMut() {
        let buf = [0; 16 * NBLOCKS];
        let key = [0; 16];
        let mut ctx = Aes128::new(&key).unwrap();

        move || {
            ctx.encrypt_inplace(&mut buf.clone());
        }
    }

    pub fn decrypt() -> impl FnMut() {
        let buf = [0; 16 * NBLOCKS];
        let key = [0; 16];
        let mut ctx = Aes128::new(&key).unwrap();

        move || {
            ctx.decrypt_inplace(&mut buf.clone());
        }
    }
}

fn aes128(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("aes-128-encrypt");
        run_bench("rust-crypto", &mut group, rust_crypto_aes::encrypt());
        run_bench("cry-rs", &mut group, cry_aes::encrypt());
    }

    {
        let mut group = c.benchmark_group("aes-128-decrypt");
        run_bench("rust-crypto", &mut group, rust_crypto_aes::decrypt());
        run_bench("cry-rs", &mut group, cry_aes::decrypt());
    }
}

criterion_group!(benches, aes128);
criterion_main!(benches);
