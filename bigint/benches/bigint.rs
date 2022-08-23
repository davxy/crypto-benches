use criterion::{criterion_group, criterion_main, Criterion};
use utils::run_bench;

const A_HEX: &str = "ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551";
const B_HEX: &str = "ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551";

mod parity_uint {
    use super::*;
    use primitive_types::U512;

    pub fn add() -> impl Fn() {
        let a = U512::from_str_radix(A_HEX, 16).unwrap();
        let b = U512::from_str_radix(B_HEX, 16).unwrap();

        move || {
            let _ = a
                .overflowing_mul(b)
                .0
                .overflowing_mul(a)
                .0
                .overflowing_mul(b)
                .0;
        }
    }
}

mod rust_crypto_bigint {
    use super::*;
    use crypto_bigint::{Wrapping, U512};

    pub fn add() -> impl Fn() {
        let a = Wrapping(U512::from_be_hex(A_HEX));
        let b = Wrapping(U512::from_be_hex(B_HEX));

        move || {
            let _ = a * b * a * b;
        }
    }
}

fn arithmetic(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("add");
        run_bench("parity-uint", &mut group, parity_uint::add());
        run_bench("rust-crypto", &mut group, rust_crypto_bigint::add());
    }
}

criterion_group!(benches, arithmetic);
criterion_main!(benches);
