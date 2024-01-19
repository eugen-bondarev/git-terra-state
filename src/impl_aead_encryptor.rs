use chacha20poly1305::{
    aead::{generic_array::GenericArray, Aead},
    consts::U12,
    ChaCha20Poly1305, KeyInit,
};

use crate::model::Encryptor;

fn vec_to_arr_12<T>(v: Vec<T>) -> [T; 12] {
    v.try_into().unwrap_or_else(|v: Vec<T>| {
        panic!("Expected a Vec of length {} but it was {}", 12, v.len())
    })
}

fn str_to_vec(string: String) -> Vec<u8> {
    string.into_bytes()
}

fn key_from_string(string: String) -> Vec<u8> {
    str_to_vec(string)
}

fn nonce_from_string(string: String) -> GenericArray<u8, U12> {
    vec_to_arr_12(str_to_vec(string)).into()
}

pub struct AeadEncryptor {
    nonce: String,
    key: String,
}

impl AeadEncryptor {
    pub fn new(key: String, nonce: String) -> AeadEncryptor {
        return AeadEncryptor { key, nonce };
    }
}

impl Encryptor for AeadEncryptor {
    fn encrypt(&self, content: Vec<u8>) -> Vec<u8> {
        let cipher =
            ChaCha20Poly1305::new_from_slice(&key_from_string(self.key.clone()).as_slice())
                .unwrap();
        let nonce = nonce_from_string(self.nonce.clone());
        return cipher.encrypt(&nonce, content.as_ref()).unwrap();
    }

    fn decrypt(&self, content: Vec<u8>) -> Vec<u8> {
        let cipher =
            ChaCha20Poly1305::new_from_slice(&key_from_string(self.key.clone()).as_slice())
                .unwrap();
        let nonce = nonce_from_string(self.nonce.clone());
        return cipher.decrypt(&nonce, content.as_ref()).unwrap();
    }
}
