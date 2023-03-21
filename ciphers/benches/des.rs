use criterion::{criterion_group, criterion_main, Criterion};
use utils::run_bench;

const NBLOCKS: usize = 1024;

mod rust_crypto_des {
    use super::*;
    use aes::cipher::generic_array::GenericArray;
    use des::{
        cipher::{consts::U8, BlockDecrypt, BlockEncrypt, KeyInit},
        Des,
    };

    type Block = GenericArray<u8, U8>;

    pub fn encrypt() -> impl Fn() {
        let key = [0; 8];
        let ctx = Des::new_from_slice(&key).unwrap();

        let mut blocks = Vec::with_capacity(NBLOCKS);
        for _ in 0..NBLOCKS {
            blocks.push(Block::default());
        }

        move || {
            ctx.encrypt_blocks(&mut blocks.clone());
        }
    }

    pub fn decrypt() -> impl Fn() {
        let key = [0; 8];
        let ctx = Des::new_from_slice(&key).unwrap();

        let mut blocks = Vec::with_capacity(NBLOCKS);
        for _ in 0..NBLOCKS {
            blocks.push(Block::default());
        }

        move || {
            ctx.decrypt_blocks(&mut blocks.clone());
        }
    }
}

mod cry_des {
    use super::*;
    use cry_rs::des::Des;

    pub fn encrypt() -> impl FnMut() {
        let buf = [0; 8 * NBLOCKS];
        let key = [0; 8];
        let mut ctx = Des::new(&key).unwrap();

        move || {
            ctx.encrypt_inplace(&mut buf.clone());
        }
    }

    pub fn decrypt() -> impl FnMut() {
        let buf = [0; 8 * NBLOCKS];
        let key = [0; 8];
        let mut ctx = Des::new(&key).unwrap();

        move || {
            ctx.decrypt_inplace(&mut buf.clone());
        }
    }
}

fn des(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("des-encrypt");
        run_bench("rust-crypto", &mut group, rust_crypto_des::encrypt());
        run_bench("cry-rs", &mut group, cry_des::encrypt());
    }

    {
        let mut group = c.benchmark_group("des-decrypt");
        run_bench("rust-crypto", &mut group, rust_crypto_des::decrypt());
        run_bench("cry-rs", &mut group, cry_des::decrypt());
    }
}

criterion_group!(benches, des);
criterion_main!(benches);
