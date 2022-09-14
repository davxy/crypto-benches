const A_HEX: &str = "e832fcffa681dec7a22795ba8e528dd5e9af5d197f50fce4340707efec68e1f9";
const B_HEX: &str = "6ba218432ef91f921a3dfe1b19725e61274cf66f76b5297c6ccd95c0d3d20ac1";
const C_HEX: &str = "4ca6087f5d5f081fbe7a305def082458ae2c5020b2b6116253dc62a8beab09ac";

mod rust_crypto_bigint {
    use super::*;
    use crypto_bigint::{Wrapping, U512};

    fn from_hex(s: &str) -> Wrapping<U512> {
        let pad = 128 - s.len();
        let s = (0..pad).map(|_| '0').chain(s.chars()).collect::<String>();
        println!("{}", s);
        Wrapping(U512::from_be_hex(&s))
    }

    #[test]
    pub fn add() {
        let a = from_hex(A_HEX);
        let b = from_hex(B_HEX);

        let _ = a + b;
    }
}

mod num_bigint {
    use super::*;
    use num::bigint::BigUint;

    fn from_hex(s: &str) -> BigUint {
        let bytes = s.as_bytes();
        BigUint::parse_bytes(bytes, 16).unwrap()
    }

    #[test]
    pub fn modpow_works() {
        let a = from_hex(A_HEX);
        let b = from_hex(B_HEX);
        let c = from_hex(C_HEX);
        let d = a.modpow(&b, &c);
        println!("{:x}", d);
    }
}

mod cry_bigint {
    use super::*;
    use cry_rs::mpi::Mpi;

    #[test]
    pub fn modpow_works() {
        let a = Mpi::from_hex(A_HEX).unwrap();
        let b = Mpi::from_hex(B_HEX).unwrap();
        let c = Mpi::from_hex(C_HEX).unwrap();
        let d = a.mod_exp(&b, &c);
        println!("{}", d);
    }
}
