use std::fs;

use crate::{
    impl_aead_encryptor::AeadEncryptor,
    model::{Encryptor, FileEncryptor},
};

pub struct DefaultFileEncryptor {
    encryptor: Box<dyn Encryptor>,
}

impl DefaultFileEncryptor {
    pub fn new(encryptor: Box<dyn Encryptor>) -> Self {
        Self { encryptor }
    }
}

impl FileEncryptor for DefaultFileEncryptor {
    fn decrypt_file(&self, from: String, to: String) {
        let file_content_encrypted = fs::read(from).unwrap();
        let file_content = self.encryptor.decrypt(file_content_encrypted);
        fs::write(to, file_content).unwrap();
    }

    fn encrypt_file(&self, from: String, to: String) {
        let file_content = fs::read(from).unwrap();
        let file_content_encrypted = self.encryptor.encrypt(file_content);
        fs::write(to, file_content_encrypted).unwrap();
    }
}
