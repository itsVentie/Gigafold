use crate::crypto::KeyManager;
use crate::storage::ChunkEncryptor;

#[test]
fn test_salt_generation() {
    let salt1 = KeyManager::generate_salt();
    let salt2 = KeyManager::generate_salt();
    assert_ne!(salt1, salt2);
}

#[test]
fn test_key_derivation() {
    let password = b"secure_password";
    let salt = KeyManager::generate_salt();

    let key1 = KeyManager::derive_key(password, &salt).unwrap();
    let key2 = KeyManager::derive_key(password, &salt).unwrap();
    assert_eq!(key1, key2);

    let wrong_salt = KeyManager::generate_salt();
    let key3 = KeyManager::derive_key(password, &wrong_salt).unwrap();
    assert_ne!(key1, key3);
}

#[test]
fn test_stream_encryption_decryption() {
    let key = [0u8; 32];
    let nonce = [1u8; 19];
    let plaintext = b"Gigafold project data for issue 4 pipeline testing. Splitting into chunks!";

    let mut ciphertext = Vec::new();
    let mut reader = &plaintext[..];
    ChunkEncryptor::encrypt_stream(
        &mut reader,
        &mut ciphertext,
        &key,
        &nonce
    ).unwrap();

    assert_ne!(plaintext.to_vec(), ciphertext);

    let mut decrypted = Vec::new();
    let mut cipher_reader = &ciphertext[..];
    ChunkEncryptor::decrypt_stream(
        &mut cipher_reader,
        &mut decrypted,
        &key,
        &nonce
    ).unwrap();

    assert_eq!(plaintext.to_vec(), decrypted);
}