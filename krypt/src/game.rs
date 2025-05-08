//! Encryption related to the game server

use rand::Rng;

/// Hard coded in the client binary
const STATIC_KEY: [u8; 64] =
    *b"nKO/WctQ0AVLbpzfBkS6NevDYT8ourG5CRlmdjyJ72aswx4EPq1UgZhFMXH?3iI9";

/// Hard coded in binary
const SUB_SEED: u32 = 0x3FF2CC87;
/// Hard coded in binary
const XOR_SEED_32: u32 = 0xCD92E451;
/// Hard coded in binary
const XOR_SEED_64: u64 = 0x87546CA100000000;

pub fn gen_xor_seed() -> u32 {
    rand::thread_rng().gen()
}

/// After a random number is generated using [gen_seed] the client and server
/// do an additional sum with two hardcoded values in the binary [SUB_SEED] and
/// [XOR_SEED]
pub fn gen_xor_key(mut seed: u32) -> u64 {
    seed = seed.wrapping_sub(SUB_SEED);
    seed ^= XOR_SEED_32;

    (seed as u64) ^ XOR_SEED_64
}

/// Encrypt game packet using a shared key that is incremented by the data length
/// after every call
/// The encryption is just a xor algorithm that uses a hardcoded key and a
/// preshared key that is agreed at runtime
pub fn encrypt(key: &mut [u8; 8], data: &mut [u8]) {
    data[0] ^= key[0];

    for i in 1..data.len() {
        let key_ptr = i & key.len() - 1;
        let static_key_ptr = i & STATIC_KEY.len() - 1;

        // XOR with the shared key
        data[i] ^= key[key_ptr];

        // XOR with current static key element
        data[i] ^= STATIC_KEY[static_key_ptr];

        // XOR the previous element
        data[i] ^= data[i - 1];
    }

    // Update the key with the length of the data
    let mut new_key = u64::from_le_bytes(*key);
    new_key = new_key.checked_add(data.len() as u64).unwrap();
    key.copy_from_slice(&new_key.to_le_bytes());
}

pub fn decrypt(key: &mut [u8; 8], data: &mut [u8]) {
    let old_key_first_byte = key[0];

    for i in (1..data.len()).rev() {
        let key_ptr = i & key.len() - 1;
        let static_key_ptr = i & STATIC_KEY.len() - 1;

        // XOR with the shared key
        data[i] ^= key[key_ptr];

        // XOR with current static key element
        data[i] ^= STATIC_KEY[static_key_ptr];

        // XOR the previous element
        data[i] ^= data[i - 1];
    }
    // Update the key with the length of the data
    let mut new_key = u64::from_le_bytes(*key);
    new_key = new_key.checked_add(data.len() as u64).unwrap();
    key.copy_from_slice(&new_key.to_le_bytes());

    data[0] ^= old_key_first_byte;
}

#[inline(always)]
pub fn decrypt_client_opcode(opcode: u16) -> u16 {
    (opcode ^ 0x20).wrapping_sub(0xB4) ^ 0x1E
}

#[inline(always)]
pub fn encrypt_server_opcode(opcode: u16) -> u16 {
    (opcode + 0xC0) ^ 0xCF
}

#[inline(always)]
pub fn decrypt_server_opcode(opcode: u16) -> u16 {
    (opcode ^ 0xCF) - 0xC0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encrypt_server_opcode() {
        assert_eq!(encrypt_server_opcode(0x00), 0x0F)
    }

    #[test]
    fn test_decrypt_client_opcode() {
        assert_eq!(decrypt_client_opcode(0xF2), 0);
    }

    #[test]
    fn encrypt_client_os_version() {
        let mut key = [0x74, 0x92, 0xB2, 0x35, 0xA1, 0x6C, 0x54, 0x87];
        let mut data = [
            0xF2, 0x00, 0x6F, 0x0D, 0xFF, 0xC0, 0x00, 0x28, 0x00, 0xE4, 0x04,
            0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
        ];

        encrypt(&mut key, &mut data);

        const EXPECTED_CIPHERTEXT: [u8; 0x15] = [
            0x86, 0x5f, 0xcd, 0xda, 0xd3, 0x1c, 0x3c, 0xc2, 0x86, 0xb1, 0x51,
            0x28, 0xeb, 0xf1, 0xdf, 0x3e, 0x08, 0xf3, 0x12, 0x11, 0xfe,
        ];
        const EXPEXTED_KEY: [u8; 8] =
            [0x89, 0x92, 0xB2, 0x35, 0xA1, 0x6C, 0x54, 0x87];

        assert_eq!(data, EXPECTED_CIPHERTEXT);
        assert_eq!(key, EXPEXTED_KEY);
    }

    #[test]
    fn decrypt_client_os_version() {
        let mut key = [0x74, 0x92, 0xB2, 0x35, 0xA1, 0x6C, 0x54, 0x87];
        let mut data = [
            0x86, 0x5f, 0xcd, 0xda, 0xd3, 0x1c, 0x3c, 0xc2, 0x86, 0xb1, 0x51,
            0x28, 0xeb, 0xf1, 0xdf, 0x3e, 0x08, 0xf3, 0x12, 0x11, 0xfe,
        ];

        decrypt(&mut key, &mut data);

        const EXPECTED_PLAINTEXT: [u8; 0x15] = [
            0xF2, 0x00, 0x6F, 0x0D, 0xFF, 0xC0, 0x00, 0x28, 0x00, 0xE4, 0x04,
            0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
        ];
        const EXPEXTED_KEY: [u8; 8] =
            [0x89, 0x92, 0xB2, 0x35, 0xA1, 0x6C, 0x54, 0x87];

        assert_eq!(key, EXPEXTED_KEY);
        assert_eq!(data, EXPECTED_PLAINTEXT);
    }

    #[test]
    fn test_generate_xor_key() {
        let seed = 0x38133813;
        let key = gen_xor_key(seed);
        assert_eq!(key, 0x87546CA135B28FDD);
    }
    fn client_decrypt(key: &mut [u8; 8], data: &mut [u8]) {
        let mut tmp = data[0];
        let mut prev = data[0];
        data[0] ^= key[0];

        let mut key_ptr = 1;
        let mut static_key_ptr = 1;

        for i in 1..data.len() {
            key_ptr &= 0x07;
            static_key_ptr &= 0x3F;

            tmp = data[i];
            // XOR the previous element
            data[i] ^= prev;
            prev = tmp;

            // XOR with the shared key
            data[i] ^= key[key_ptr];

            // XOR with current static key element
            data[i] ^= STATIC_KEY[static_key_ptr];

            key_ptr += 1;
            static_key_ptr += 1;
        }
    }
    fn server_encrypt(key: &mut [u8; 8], data: &mut [u8]) {
        let mut tmp = data[0];
        let mut prev = data[0];
        data[0] ^= key[0];

        let mut key_ptr = 1;
        let mut static_key_ptr = 1;

        for i in 1..data.len() {
            key_ptr &= 0x07;
            static_key_ptr &= 0x3F;

            tmp = data[i];
            // XOR the previous element
            data[i] ^= prev;
            prev = tmp;

            // XOR with the shared key
            data[i] ^= key[key_ptr];

            // XOR with current static key element
            data[i] ^= STATIC_KEY[static_key_ptr];

            key_ptr += 1;
            static_key_ptr += 1;
        }
    }
    #[test]
    fn test_longer_packet() {
        let target_plain = [
            0x39, 0x00, 0x40, 0xC6, 0xFF, 0xD2, 0x42, 0x00, 0x2A, 0x09, 0xC2,
            0x01, 0x00, 0x02, 0x0E, 0x93, 0x01, 0x80, 0x32, 0x64, 0x00, 0x00,
            0x00, 0x00, 0x03, 0x13, 0x37, 0x13, 0x00, 0x0A, 0x00, 0x18, 0x00,
            0x00, 0x00, 0x0B, 0x00, 0x02, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x00,
        ];

        let target_cipher = [
            0xDC, 0x66, 0xBE, 0x49, 0x40, 0x9D, 0xFF, 0x29, 0xD6, 0x6F, 0x2C,
            0x7F, 0xBC, 0xA2, 0x82, 0xF0, 0x56, 0x4C, 0xFA, 0xB6, 0x59, 0x50,
            0x72, 0xB1, 0x0E, 0xB8, 0x60, 0x02, 0xD6, 0xC2, 0xD1, 0x7B, 0xDD,
            0x7E, 0xC5, 0xBD, 0x78, 0x7C, 0x51, 0x9C, 0x4E, 0x87, 0x31, 0x5C,
        ];

        let mut input = target_cipher.clone();

        let mut key = 0x87546CA11ED7F1E5_u64.to_le_bytes();
        decrypt(&mut key, &mut input);
        assert_eq!(input, target_plain);

        let mut key = 0x87546CA11ED7F1E5_u64.to_le_bytes();
        encrypt(&mut key, &mut input);
        assert_eq!(input, target_cipher);
    }
}
