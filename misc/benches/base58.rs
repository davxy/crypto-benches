use criterion::{criterion_group, criterion_main, Criterion};
use utils::run_bench;

const BYTES: &[u8; 126] = &[
    99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52, 98, 108,
    51, 125, 99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52,
    98, 108, 51, 125, 99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110,
    116, 52, 98, 108, 51, 125, 99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114,
    49, 110, 116, 52, 98, 108, 51, 125, 99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95,
    112, 114, 49, 110, 116, 52, 98, 108, 51, 125, 99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73,
];

const STRING: &str = "XJEUKbjEAPUNWcrnuZAoriZjDaQ4ZR2ZUR4vjZrHde61uVeUvMy1uu83NDpbixDrAPfAjmJNFvfwrmPJm4aaBsXEF5v4D7tAA6vE7adYDB6cBpuV6ECNbqoSWj5HmGqy9vmr6pmUXhnMc8SHGDPwFDr6FYuRUTV9stwP1UW5UgBr";

mod base58_crate {
    use super::*;
    use base58::{FromBase58, ToBase58};

    pub fn base58_encode() -> impl Fn() {
        eprintln!("{}", BYTES.to_base58());
        move || {
            let _ = BYTES.to_base58();
        }
    }

    pub fn base58_decode() -> impl Fn() {
        move || {
            let _ = STRING.from_base58().unwrap();
        }
    }
}

mod bs58_crate {
    use super::*;

    pub fn base58_encode() -> impl Fn() {
        move || {
            let _ = bs58::encode(BYTES).into_string();
        }
    }

    pub fn base58_decode() -> impl Fn() {
        move || {
            let _ = bs58::decode(STRING).into_vec().unwrap();
        }
    }
}

fn base64(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("base58-encode");
        run_bench("base58-crate", &mut group, base58_crate::base58_encode());
        run_bench("bs58-crate", &mut group, bs58_crate::base58_encode());
    }
    {
        let mut group = c.benchmark_group("base58-decode");
        run_bench("base58-crate", &mut group, base58_crate::base58_decode());
        run_bench("bs58-crate", &mut group, bs58_crate::base58_decode());
    }
}

criterion_group!(benches, base64);
criterion_main!(benches);
