use crate::crypto::KeyManager;
use crate::storage::ChunkEncryptor;
use crate::index::{IndexManager, FileMetadata, FileType, ChunkInfo};

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

#[test]
fn test_index_manager_lifecycle() {
    let tmp_dir = std::env::temp_dir().join("gigafold_test_db");
    if tmp_dir.exists() {
        let _ = std::fs::remove_dir_all(&tmp_dir);
    }

    let manager = IndexManager::open(&tmp_dir).unwrap();

    let chunk = ChunkInfo {
        hash: "a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e".to_string(),
        nonce: [7u8; 19],
        size: 65536,
    };

    let meta = FileMetadata {
        name: "document.pdf".to_string(),
        file_type: FileType::File,
        size: 65536,
        created_at: 1717000000,
        updated_at: 1717000000,
        chunks: vec![chunk],
    };

    manager.save_file("/root/document.pdf", &meta).unwrap();

    let retrieved = manager.get_file("/root/document.pdf").unwrap().unwrap();
    assert_eq!(retrieved.name, "document.pdf");
    assert_eq!(retrieved.file_type, FileType::File);
    assert_eq!(retrieved.chunks[0].hash, meta.chunks[0].hash);

    manager.delete_file("/root/document.pdf").unwrap();
    let after_delete = manager.get_file("/root/document.pdf").unwrap();
    assert!(after_delete.is_none());

    let _ = std::fs::remove_dir_all(&tmp_dir);
}