use criterion::{criterion_group, criterion_main, Criterion};
use utils::run_bench;

const A_HEX: &str = "e832fcffa681dec7a22795ba8e528dd5e9af5d197f50fce4340707efec68e1f9";
const B_HEX: &str = "6ba218432ef91f921a3dfe1b19725e61274cf66f76b5297c6ccd95c0d3d20ac1";
const C_HEX: &str = "4ca6087f5d5f081fbe7a305def082458ae2c5020b2b6116253dc62a8beab09ac";

mod parity_uint {
    use super::*;
    use primitive_types::U512;

    pub fn add() -> impl Fn() {
        let a = U512::from_str_radix(A_HEX, 16).unwrap();
        let b = U512::from_str_radix(B_HEX, 16).unwrap();

        move || {
            let _ = a.clone().overflowing_add(b.clone());
        }
    }
}

mod rust_crypto_bigint {
    use super::*;
    use crypto_bigint::{Wrapping, U512};

    fn from_hex(s: &str) -> Wrapping<U512> {
        let pad = 128 - s.len();
        let s = (0..pad).map(|_| '0').chain(s.chars()).collect::<String>();
        Wrapping(U512::from_be_hex(&s))
    }

    pub fn add() -> impl Fn() {
        let a = from_hex(A_HEX);
        let b = from_hex(B_HEX);

        move || {
            let _ = a.clone() + b.clone();
        }
    }
}

mod num_bigint {
    use super::*;
    use num::bigint::BigUint;

    fn from_hex(s: &str) -> BigUint {
        let bytes = s.as_bytes();
        BigUint::parse_bytes(bytes, 16).unwrap()
    }

    pub fn add() -> impl Fn() {
        let a = from_hex(A_HEX);
        let b = from_hex(B_HEX);

        move || {
            let _ = a.clone() + b.clone();
        }
    }

    pub fn modexp() -> impl Fn() {
        let a = from_hex(A_HEX);
        let b = from_hex(B_HEX);
        let c = from_hex(C_HEX);

        move || {
            let _ = a.modpow(&b, &c);
        }
    }
}

mod cry_bigint {
    use super::*;
    use cry_rs::mpi::Mpi;

    pub fn add() -> impl Fn() {
        let a = Mpi::from_hex(A_HEX).unwrap();
        let b = Mpi::from_hex(B_HEX).unwrap();

        move || {
            let _ = a.clone() + b.clone();
        }
    }

    pub fn modexp() -> impl Fn() {
        let a = Mpi::from_hex(A_HEX).unwrap();
        let b = Mpi::from_hex(B_HEX).unwrap();
        let c = Mpi::from_hex(C_HEX).unwrap();
        move || {
            let _ = a.mod_exp(&b, &c);
        }
    }
}

fn arithmetic(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("add");
        run_bench("parity-uint", &mut group, parity_uint::add());
        run_bench("rust-crypto", &mut group, rust_crypto_bigint::add());
        run_bench("num-bigint", &mut group, num_bigint::add());
        run_bench("cry-bigint", &mut group, cry_bigint::add());
    }
    {
        let mut group = c.benchmark_group("modexp");
        run_bench("num-bigint", &mut group, num_bigint::modexp());
        run_bench("cry-bigint", &mut group, cry_bigint::modexp());
    }
}

criterion_group!(benches, arithmetic);
criterion_main!(benches);
