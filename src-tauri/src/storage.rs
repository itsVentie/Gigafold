use chacha20poly1305::{
    aead::stream::{DecryptorBE32, EncryptorBE32},
    XChaCha20Poly1305, KeyInit
};
use std::io::{Read, Write};

pub struct ChunkEncryptor;

impl ChunkEncryptor {
    pub fn encrypt_stream<W: Write>(
        source: &mut dyn Read,
        mut dest: W,
        key: &[u8; 32],
        nonce: &[u8; 19],
    ) -> Result<(), String> {
        let aead = XChaCha20Poly1305::new(key.into());
        let mut encryptor = EncryptorBE32::from_aead(aead, nonce.into());
        let mut buffer = vec![0u8; 64 * 1024];
      
        let mut current_source: Box<dyn Read> = Box::new(source);

        loop {
            let read_bytes = current_source.read(&mut buffer).map_err(|e| e.to_string())?;
            if read_bytes == 0 {
                break;
            }

            let chunk = &buffer[..read_bytes];
            let mut next_buffer = vec![0u8; 1];
            
            let is_last = match current_source.read(&mut next_buffer) {
                Ok(0) => true,
                Ok(n) => {
                    let leftover = std::io::Cursor::new(next_buffer[..n].to_vec());
                    current_source = Box::new(leftover.chain(current_source));
                    false
                }
                Err(e) => return Err(e.to_string()),
            };

            if is_last {
                let ciphertext = encryptor
                    .encrypt_last(chunk)
                    .map_err(|e| format!("Encryption failure: {}", e))?;
                dest.write_all(&ciphertext).map_err(|e| e.to_string())?;
                break;
            } else {
                let ciphertext = encryptor
                    .encrypt_next(chunk)
                    .map_err(|e| format!("Encryption failure: {}", e))?;
                dest.write_all(&ciphertext).map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    pub fn decrypt_stream<W: Write>(
        source: &mut dyn Read,
        mut dest: W,
        key: &[u8; 32],
        nonce: &[u8; 19],
    ) -> Result<(), String> {
        let aead = XChaCha20Poly1305::new(key.into());
        let mut decryptor = DecryptorBE32::from_aead(aead, nonce.into());
        let mut buffer = vec![0u8; (64 * 1024) + 16];
        
        let mut current_source: Box<dyn Read> = Box::new(source);

        loop {
            let read_bytes = current_source.read(&mut buffer).map_err(|e| e.to_string())?;
            if read_bytes == 0 {
                break;
            }

            let chunk = &buffer[..read_bytes];
            let mut next_buffer = vec![0u8; 1];

            let is_last = match current_source.read(&mut next_buffer) {
                Ok(0) => true,
                Ok(n) => {
                    let leftover = std::io::Cursor::new(next_buffer[..n].to_vec());
                    current_source = Box::new(leftover.chain(current_source));
                    false
                }
                Err(e) => return Err(e.to_string()),
            };

            if is_last {
                let plaintext = decryptor
                    .decrypt_last(chunk)
                    .map_err(|e| format!("Decryption failure: {}", e))?;
                dest.write_all(&plaintext).map_err(|e| e.to_string())?;
                break;
            } else {
                let plaintext = decryptor
                    .decrypt_next(chunk)
                    .map_err(|e| format!("Decryption failure: {}", e))?;
                dest.write_all(&plaintext).map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
}