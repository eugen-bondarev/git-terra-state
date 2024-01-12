/*
 * Interfaces
 */
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
