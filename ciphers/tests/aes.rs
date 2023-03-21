mod rust_crypto_aes {
    use aes::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};
    use aes::{Aes256, Block};

    #[test]
    fn decrypt() {
        let key = b"\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01";
        let c = b"\x53\x9B\x33\x3B\x39\x70\x6D\x14\x90\x28\xCF\xE1\xD9\xD4\xA4\x07";

        let ctx = Aes256::new_from_slice(key).unwrap();
        let mut blk = Block::clone_from_slice(c);

        ctx.decrypt_block(&mut blk);

        let m = blk.to_vec();
        println!("{:x?}", m);
    }

    #[test]
    fn encrypt() {
        let key = b"\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01";
        let m = b"\x29\x6C\x93\xFD\xF4\x99\xAA\xEB\x41\x94\xBA\xBC\x2E\x63\x56\x1D";

        let ctx = Aes256::new_from_slice(key).unwrap();
        let mut blk = Block::clone_from_slice(m);

        ctx.encrypt_block(&mut blk);

        let c = blk.to_vec();
        println!("{:x?}", c);
    }

    #[test]
    fn decrypt_encrypt() {
        let key = b"\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01";
        let m = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

        let ctx = Aes256::new_from_slice(key).unwrap();
        let mut blk = Block::clone_from_slice(m);

        ctx.decrypt_block(&mut blk);
        let m = blk.to_vec();
        println!("{:x?}", m);

        ctx.encrypt_block(&mut blk);
        let c = blk.to_vec();
        println!("{:x?}", c);
    }
}
