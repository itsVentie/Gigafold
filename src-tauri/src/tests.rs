use crate::crypto::KeyManager;

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