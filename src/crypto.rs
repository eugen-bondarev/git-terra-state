use encryptfile as ef;

pub fn encrypt_file(src: String, dst: String, key: String) {
    let mut c = ef::Config::new();

    c.input_stream(ef::InputStream::File(src))
        .output_stream(ef::OutputStream::File(dst))
        .add_output_option(ef::OutputOption::AllowOverwrite)
        .initialization_vector(ef::InitializationVector::GenerateFromRng)
        .password(ef::PasswordType::Text(key, ef::scrypt_defaults()))
        .encrypt();

    let _ = ef::process(&c).map_err(|e| panic!("error encrypting: {:?}", e));
}

pub fn decrypt_file(src: String, dst: String, key: String) {
    let mut c = ef::Config::new();
    c.input_stream(ef::InputStream::File(src))
        .output_stream(ef::OutputStream::File(dst))
        .add_output_option(ef::OutputOption::AllowOverwrite)
        .password(ef::PasswordType::Text(
            key,
            ef::PasswordKeyGenMethod::ReadFromFile,
        ))
        .decrypt();

    let _ = ef::process(&c).map_err(|e| panic!("error decrypting: {:?}", e));
}
