A simple, fast, lightweight and autonomous tool to securely store Terraform state in a git repository.

**Please note: currently, it's only a prototype (v0.1.0). There are several problems that I will address in future releases.**

**Please note: this tool doesn't aim to be a terraform backend.**

# How it works

Underneath the hood git-terra-state is a Rust program that applies four functions on your Terraform state file:

1. Encrypt
2. Push to git
3. Pull from git
4. Decrypt

# Quick start

First time use:

1. Create a Terraform project or open an existing one
2. .gitignore your .tfstate file. Here are my .gitignore entries for example: `.terraform`, `.terraform.lock.hcl`, `*.tfstate`, `*.tfstate.backup`, `.env`
3. Create a `.env` file containing the following variables:
   1. `KEY`: a key you want to use to encrypt the state file, example: `xcmqiqwemqweoiqasdkj`
   2. `REPO`: a repository in which you want to store the state file, example: `git@github.com:lorem/ipsum.git`
   3. `EMAIL`: an email that should be used for the commits
   4. `SSH_KEY`: a base64-encoded one-liner containing a private SSH key with permission to push to the repository `REPO`
4. `terraform apply`
5. `docker run --env-file=.env -v ./:/workspace eugbondarev/git-terra-state push`

Subsequent use:

1. `docker run --env-file=.env -v ./:/workspace eugbondarev/git-terra-state pull`
2. `terraform apply`
3. `docker run --env-file=.env -v ./:/workspace eugbondarev/git-terra-state push`

### Demo

Here is a small demo project consisting of two repos:

- https://github.com/eugen-bondarev/git-terra-state-test: a terraform project that uses git-terra-state in CI/CD.
- https://github.com/eugen-bondarev/git-terra-state-test-state: repo containing the state file.

# Motivation

I wanted to create a straightforward solution for people who don't want to use some cloud storage for their Terraform state. I want to make Git sufficient for this use case.

# Goals

// Coming soon

# Roadmap

- Implement proper encryption
- Minimize the number of uses of run_command, where Linux commands are executed, and eventually get rid of them altogether
- Make the implementation "git host agnostic" (at this point, only GitHub is supported)
