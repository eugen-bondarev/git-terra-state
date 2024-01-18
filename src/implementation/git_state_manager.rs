use std::{env, fs, path::Path};

use crate::{
    cmd::run_command,
    crypto,
    git::Git,
    model::{CryptoManager, FileManager, StateManager},
};

pub struct GitStateManager {
    workspace: String,
    tmp: String,
    git: Git,
}

impl GitStateManager {
    pub fn new(workspace: &str, tmp: &str, git: Git) -> GitStateManager {
        GitStateManager {
            workspace: String::from(workspace),
            tmp: String::from(tmp),
            git,
        }
    }

    fn get_in_workspace(&self, file: &str) -> String {
        format!("{}/{}", self.workspace, file)
    }

    fn get_in_tmp_dir(&self, file: &str) -> String {
        format!("{}/{}", self.tmp, file)
    }

    fn get_key(&self) -> String {
        env::var("KEY").expect("$KEY is unset")
    }

    fn get_repo(&self) -> String {
        env::var("REPO").expect("$REPO is unset")
    }

    fn delete_repo(&self) {
        let wd = self.get_in_tmp_dir("");
        run_command(format!("rm -rf {}", wd));
    }

    fn clone_repo(&self) {
        let wd = self.get_in_tmp_dir("");
        let repo = self.get_repo();
        self.git.clone(repo, wd);
    }

    fn clean_clone_repo(&self) {
        self.delete_repo();
        self.clone_repo();
    }

    fn push_to_repo(&self) {
        let wd = self.get_in_tmp_dir("");
        let remote = self.get_repo();
        self.git.push(remote, wd);
    }

    fn ensure_dir_exists(dir: String) {
        if Path::new(dir.as_str()).exists() {
            return;
        }
        fs::create_dir(dir).unwrap();
    }

    fn copy(from: String, to: String) {
        fs::copy(from, to).unwrap();
    }

    fn move_file(from: String, to: String) {
        fs::rename(from, to).unwrap();
    }
}

impl CryptoManager for GitStateManager {
    fn decrypt(&self) {
        let src = self.get_in_workspace("terraform.tfstate.encrypted");
        let dst = self.get_in_workspace("terraform.tfstate");
        crypto::decrypt_file(src.clone(), dst.clone(), self.get_key());
        run_command(format!("rm -rf {}", src));
        run_command(format!("chmod 777 {}", dst));
    }

    fn encrypt(&self) {
        let src = self.get_in_workspace("terraform.tfstate");
        let dst = self.get_in_workspace("terraform.tfstate.encrypted");
        crypto::encrypt_file(src, dst, self.get_key());
    }
}

impl FileManager for GitStateManager {
    fn pull(&self) {
        self.clean_clone_repo();

        let encrypted_state_file_src = self.get_in_tmp_dir("terraform.tfstate");
        let encrypted_state_file_dst = self.get_in_workspace("terraform.tfstate.encrypted");

        Self::ensure_dir_exists(self.get_in_workspace(""));
        Self::copy(encrypted_state_file_src, encrypted_state_file_dst);
    }

    fn push(&self) {
        let encrypted_state_file_src = self.get_in_workspace("terraform.tfstate.encrypted");
        let encrypted_state_file_dst = self.get_in_tmp_dir("terraform.tfstate");

        Self::ensure_dir_exists(self.get_in_tmp_dir(""));
        Self::move_file(encrypted_state_file_src, encrypted_state_file_dst);

        self.push_to_repo();
    }
}

impl StateManager for GitStateManager {}
