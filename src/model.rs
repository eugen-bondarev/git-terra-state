/*
 * Interfaces
 */
pub trait Encryptor {
    fn encrypt(&self, content: Vec<u8>) -> Vec<u8>;
    fn decrypt(&self, content: Vec<u8>) -> Vec<u8>;
}

pub trait FileEncryptor {
    fn encrypt_file(&self, from: String, to: String);
    fn decrypt_file(&self, from: String, to: String);
}

pub trait CryptoManager {
    fn decrypt(&self);
    fn encrypt(&self);
}

pub trait FileManager {
    fn push(&self);
    fn pull(&self);
}

/*
 * TODO:
 *      Create a struct that has file_service and crypto_service as members
 */
pub trait StateManager: CryptoManager + FileManager {}
