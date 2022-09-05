use criterion::{criterion_group, criterion_main, Criterion};
use utils::run_bench;

const BYTES: &[u8; 23] = &[
    99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52, 98, 108,
    51, 125,
];

const STRING: &str = "Y3J5cHRve0FTQ0lJX3ByMW50NGJsM30=";

mod cry {
    use super::*;
    use cry_rs::base64;

    pub fn base64_encode() -> impl Fn() {
        move || {
            let _ = base64::encode(BYTES);
        }
    }

    pub fn base64_decode() -> impl Fn() {
        move || {
            let _ = base64::decode(STRING);
        }
    }
}

mod smile {
    use super::*;
    use smile_utils::base64;

    pub fn base64_encode() -> impl Fn() {
        move || {
            let _ = base64::encode(BYTES);
        }
    }

    pub fn base64_decode() -> impl Fn() {
        move || {
            let _ = base64::decode(STRING);
        }
    }
}

mod base64_crate {
    use super::*;
    use base64 as base64_crate;

    pub fn base64_encode() -> impl Fn() {
        move || {
            let _ = base64_crate::encode(BYTES);
        }
    }

    pub fn base64_decode() -> impl Fn() {
        move || {
            let _ = base64_crate::decode(STRING);
        }
    }
}

fn base64(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("base64-encode");
        run_bench("base64-crate", &mut group, base64_crate::base64_encode());
        run_bench("cry", &mut group, cry::base64_encode());
        run_bench("smile", &mut group, smile::base64_encode());
    }
    {
        let mut group = c.benchmark_group("base64-decode");
        run_bench("base64-crate", &mut group, base64_crate::base64_decode());
        run_bench("cry", &mut group, cry::base64_decode());
        run_bench("smile", &mut group, smile::base64_decode());
    }
}

criterion_group!(benches, base64);
criterion_main!(benches);
