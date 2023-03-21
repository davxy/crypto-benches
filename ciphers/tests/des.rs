mod rust_crypto_des {
    // Note: the library accept an 8 bytes slice (i.e. 64 bits) as the key while
    // the AES key is 56 bits.
    // The least significant bit of each byte in the key is discarded.
    // That is 0x0000000000000000 == 0x0101010101010101
    use des::{
        cipher::{consts::U8, generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit},
        Des,
    };

    pub type Block = GenericArray<u8, U8>;

    #[test]
    fn encrypt() {
        let key = b"\x80\x00\x00\x00\x00\x00\x00\x00";
        let m = b"\x53\x9B\x33\x3B\x39\x70\x6D\x14";

        let ctx = Des::new_from_slice(key).unwrap();
        let mut blk = Block::clone_from_slice(m);

        ctx.encrypt_block(&mut blk);

        println!("{:x?}", blk.to_vec());
    }

    #[test]
    fn decrypt() {
        let key = b"\x80\x00\x00\x00\x00\x00\x00\x00";
        let c = b"\x53\x9B\x33\x3B\x39\x70\x6D\x14";

        let ctx = Des::new_from_slice(key).unwrap();
        let mut blk = Block::clone_from_slice(c);

        ctx.decrypt_block(&mut blk);

        println!("{:x?}", blk.to_vec());
    }

    // 0x00..00 is a weak key in DES
    // Tha is E[0, m] = D[0, m]
    #[test]
    fn weak_key() {
        let key = b"\x00\x00\x00\x00\x00\x00\x00\x00";
        let m = b"\x53\x9B\x33\x3B\x39\x70\x6D\x14";

        let ctx = Des::new_from_slice(key).unwrap();

        let mut blk = Block::clone_from_slice(m);
        ctx.encrypt_block(&mut blk);
        let e = blk.to_vec();

        let mut blk = Block::clone_from_slice(m);
        ctx.decrypt_block(&mut blk);
        let d = blk.to_vec();

        assert_eq!(e, d);
    }

    // E[!k, !p] = !E[k, p]
    #[test]
    fn complementation_property() {
        let key: u64 = 0x8000000000000000;
        let m: u64 = 0x539B333B39706D14;

        let ctx = Des::new_from_slice(&key.to_be_bytes()).unwrap();
        let mut blk = Block::clone_from_slice(&m.to_be_bytes());
        ctx.encrypt_block(&mut blk);
        let c1 = u64::from_be_bytes(blk.as_slice().try_into().unwrap());
        println!("!E[0x{key:x}, 0x{m:x}] = !0x{c1:x} = 0x{:x}", !c1);

        let ctx = Des::new_from_slice(&(!key).to_be_bytes()).unwrap();
        let mut blk = Block::clone_from_slice(&(!m).to_be_bytes());
        ctx.encrypt_block(&mut blk);
        let c2 = u64::from_be_bytes(blk.as_slice().try_into().unwrap());
        println!(
            "E[!0x{key:x}, !0x{m:x}] = E[0x{:x}, 0x{:x}] = 0x{c2:x}",
            !key, !m
        );

        assert_eq!(!c1, c2);
    }
}
