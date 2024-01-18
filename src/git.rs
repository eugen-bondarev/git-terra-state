use std::path::Path;

use crate::cmd::run_command;

pub struct Git {
    ssh_key: String,
    email: String,
}

impl Git {
    pub fn new(ssh_key: String, email: String) -> Git {
        Git { ssh_key, email }
    }

    fn get_ssh_dir_path() -> String {
        format!("{}/.ssh", simple_home_dir::home_dir().unwrap().display())
    }

    fn ssh_dir_exists() -> bool {
        Path::new(&Self::get_ssh_dir_path()[..]).exists()
    }

    fn running_in_docker() -> bool {
        !Self::ssh_dir_exists()
    }

    fn should_prepare_ssh(&self) -> bool {
        return self.ssh_key.len() > 0;
    }

    fn prepare_ssh(&self) {
        if !Self::running_in_docker() {
            panic!(
                "you should only run this program inside a docker container as it modifies your ~/.ssh dir"
            );
        }

        /*
         * TODO: replace these three commands with rust code
         */
        run_command(format!("mkdir ~/.ssh"));
        run_command(format!("echo \"{}\" > ~/.ssh/id_rsa", self.ssh_key.trim()));
        run_command(format!("chmod 400 ~/.ssh/id_rsa"));

        run_command(format!(
            "ssh-keyscan -t rsa github.com >> ~/.ssh/known_hosts"
        ));
    }

    pub fn clone(&self, repo: String, dst: String) {
        if self.should_prepare_ssh() {
            self.prepare_ssh();
        }

        /*
         * TODO: replace this call with rust code
         */
        run_command(format!("mkdir {}", dst));
        run_command(format!("git clone {} {}", repo, dst));
    }

    pub fn push(&self, repo: String, repo_location: String) {
        if self.should_prepare_ssh() {
            self.prepare_ssh();
        }

        run_command(format!(
            "cd {} && git config user.email {}",
            repo_location, self.email
        ));
        run_command(format!("cd {} && git init", repo_location));
        run_command(format!(
            "cd {} && git remote add origin {}",
            repo_location, repo
        ));
        run_command(format!("cd {} && git add .", repo_location));
        run_command(format!("cd {} && git commit -m \"test\"", repo_location));
        run_command(format!("cd {} && git switch -c main", repo_location));

        run_command(format!(
            "cd {} && git push -f -u origin main",
            repo_location
        ));
    }
}
