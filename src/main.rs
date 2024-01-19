mod cmd;
mod git;
mod impl_aead_encryptor;
mod impl_default_file_encryptor;
mod impl_git_state_manager;
mod model;

use base64::{engine::general_purpose, Engine};
use dotenv::dotenv;
use git::Git;
use impl_aead_encryptor::AeadEncryptor;
use impl_default_file_encryptor::DefaultFileEncryptor;
use impl_git_state_manager::GitStateManager;
use model::StateManager;
use std::{
    env,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

struct App {
    pub state_manager: Box<dyn StateManager>,
}

impl App {}

fn get_time_since_epoch() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
}

/*
 * Env helpers
 */
fn get_private_key() -> String {
    env::var("SSH_KEY").expect("$SSH_KEY is unset")
}

fn get_decoded_private_key() -> String {
    let key = get_private_key();
    let bytes_vec = general_purpose::STANDARD.decode(key).unwrap();
    let bytes_arr = bytes_vec.as_slice();

    match std::str::from_utf8(bytes_arr) {
        Ok(s) => String::from(s),
        Err(_) => return String::from(""),
    }
}

fn get_keys() -> String {
    env::var("KEY").expect("$KEY is unset")
}

fn get_email() -> String {
    env::var("EMAIL").expect("$EMAIL is unset")
}

fn main() {
    dotenv().ok();

    let git = Git::new(get_decoded_private_key(), get_email());
    // let git = Git::new(String::from(""), get_email());

    let keys = get_keys();
    let key: Vec<&str> = keys.split(":").collect();
    let enc = AeadEncryptor::new(String::from(key[0]), String::from(key[1]));
    let file_encryptor = DefaultFileEncryptor::new(Box::new(enc));

    let app = App {
        state_manager: Box::new(GitStateManager::new(
            // "./workspace",
            // "./tmp",
            "/workspace",
            "/tmp",
            git,
            Box::new(file_encryptor),
        )),
    };

    let args = &env::args().collect::<Vec<String>>()[1..];

    /*
     * TODO:
     *      Use a clean CLI lib
     */
    match args[0].as_str() {
        "push" => {
            {
                let start = get_time_since_epoch();
                app.state_manager.encrypt();
                let delta = get_time_since_epoch() - start;
                println!("State encrypted in {:?}", delta);
            }
            {
                let start = get_time_since_epoch();
                app.state_manager.push();
                let delta = get_time_since_epoch() - start;
                println!("State pushed in {:?}", delta);
            }
        }
        "pull" => {
            {
                let start = get_time_since_epoch();
                app.state_manager.pull();
                let delta = get_time_since_epoch() - start;
                println!("State pulled in {:?}", delta);
            }
            {
                let start = get_time_since_epoch();
                app.state_manager.decrypt();
                let delta = get_time_since_epoch() - start;
                println!("State decrypted in {:?}", delta);
            }
        }
        _ => {}
    };
}
