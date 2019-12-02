use bip39;
use bitstream_io::{BigEndian, BitWriter};
use openssl::sha::sha256;

const BITS_PER_WORD: usize = 11;

pub fn solve_checkwords(words: &[&str], mnemonic_type: bip39::MnemonicType) -> Vec<&'static str> {
    assert_eq!(words.len(), mnemonic_type.word_count() - 1);
    let wordmap = bip39::Language::English.wordmap();
    let wordlist = bip39::Language::English.wordlist();
    let check_bits = mnemonic_type.checksum_bits() as usize;
    let check_shift = 8 - check_bits;
    let check_mask = (((1u16 << (check_bits + 1)) - 1) << check_shift) as u8;
    let nonce_bits = BITS_PER_WORD - check_bits;
    let max_nonce = 1u16 << nonce_bits;
    (0u16..max_nonce)
        .map(|nonce| {
            let mut bytes: Vec<u8> = Vec::new();
            {
                let mut writer =
                    words
                        .iter()
                        .fold(BitWriter::endian(&mut bytes, BigEndian), |mut acc, word| {
                            let bits: u16 = wordmap.get_bits(word).unwrap().into();
                            acc.write(BITS_PER_WORD as u32, bits).unwrap();
                            acc
                        });
                writer.write(nonce_bits as u32, nonce).unwrap();
                assert!(writer.byte_aligned());
            }
            assert_eq!(bytes.len(), mnemonic_type.entropy_bits() / 8);
            let hash = sha256(&bytes);
            let checksum: u16 = ((hash[0] & check_mask) >> check_shift) as u16;
            let idx = (nonce << check_bits) | checksum;
            wordlist.get_word(idx.into())
        })
        .collect()
}
