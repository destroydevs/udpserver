extern crate crypto;
extern crate rand;

use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::symmetriccipher::SymmetricCipherError;
use crypto::{aes, blockmodes, buffer};
use rand::Rng;

pub struct Crypt {
    key: [u8; 32],
    iv: [u8; 16],
}

impl Crypt {
    pub fn new() -> Crypt {
        // generating AES key and IV
        let mut rng = rand::rng();
        let mut key = [0u8; 32];
        let mut iv = [0u8; 16];
        rng.fill(&mut key);
        rng.fill(&mut iv);

        Crypt { key, iv }
    }

    fn get_key(&self) -> [u8; 32] {
        self.key
    }

    fn get_iv(&self) -> [u8; 16] {
        self.iv
    }

    // Decryption method
    // for more info https://docs.rs/rust-crypto/latest/crypto/aes/index.html#functions
    pub fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, SymmetricCipherError> {
        let mut decryptor = aes::cbc_decryptor(
            aes::KeySize::KeySize256,
            &self.key,
            &self.iv,
            blockmodes::PkcsPadding);

        let mut final_result = Vec::<u8>::new();
        let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
        let mut buffer = [0; 4096];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

        loop {
            let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
            final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().copied());
            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => {}
            }
        }

        Ok(final_result)
    }

    // Encryption method
    // for more info https://docs.rs/rust-crypto/latest/crypto/aes/index.html#functions
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, SymmetricCipherError> {
        let mut encryptor = aes::cbc_encryptor(
            aes::KeySize::KeySize256,
            &self.key,
            &self.iv,
            blockmodes::PkcsPadding);

        let mut final_result = Vec::<u8>::new();
        let mut read_buffer = buffer::RefReadBuffer::new(data);
        let mut buffer = [0; 4096];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

        loop {
            let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;
            final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().copied());
            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => {}
            }
        }

        Ok(final_result)
    }
}
