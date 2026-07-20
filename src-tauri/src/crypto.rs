use argon2::Argon2;
use chacha20poly1305::aead::rand_core::RngCore;
use chacha20poly1305::aead::OsRng;

pub struct KeyManager;

impl KeyManager {
    pub fn generate_salt() -> [u8; 16] {
        let mut salt = [0u8; 16];
        OsRng.fill_bytes(&mut salt);
        salt
    }

    pub fn derive_key(password: &[u8], salt: &[u8]) -> Result<[u8; 32], String> {
        let mut key = [0u8; 32];
        Argon2::default()
            .hash_password_into(password, salt, &mut key)
            .map_err(|e| e.to_string())?;
        Ok(key)
    }
}