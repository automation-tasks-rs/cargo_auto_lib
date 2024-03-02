// auto_encrypt_decrypt_with_ssh_mod.rs

// region: auto_md_to_doc_comments include README.md A //!
//! # cargo_auto_lib
//!
//! **Library crate for common tasks when building rust projects. Intended for use with cargo-auto - automation tasks written in Rust language.**  
//! ***version: 1.4.1 date: 2024-03-02 author: [Bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/cargo_auto_lib)***
//!
//!  ![maintained](https://img.shields.io/badge/maintained-green)
//!  ![ready-for-use](https://img.shields.io/badge/ready_for_use-green)
//!  ![rustlang](https://img.shields.io/badge/rustlang-orange)
//!  ![cargo-auto](https://img.shields.io/badge/cargo_auto-orange)
//!
//!  ![logo](https://raw.githubusercontent.com/bestia-dev/cargo-auto/main/images/logo/logo_cargo_auto.svg)
//!
//!  [![crates.io](https://img.shields.io/crates/v/cargo_auto_lib.svg)](https://crates.io/crates/cargo_auto_lib)
//!  [![Documentation](https://docs.rs/cargo_auto_lib/badge.svg)](https://docs.rs/cargo_auto_lib/)
//!  [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/cargo_auto_lib.svg)](https://web.crev.dev/rust-reviews/crate/cargo_auto_lib/)
//!  [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/cargo_auto_lib/)
//!
//!  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/cargo_auto_lib/blob/master/LICENSE)
//!  [![Rust](https://github.com/bestia-dev/cargo_auto_lib/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/bestia-dev/cargo_auto_lib/)
//!  ![cargo_auto_lib](https://bestia.dev/webpage_hit_counter/get_svg_image/276360626.svg)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-2482-green.svg)](https://github.com/bestia-dev/cargo_auto_lib/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-979-blue.svg)](https://github.com/bestia-dev/cargo_auto_lib/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-377-purple.svg)](https://github.com/bestia-dev/cargo_auto_lib/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-28-yellow.svg)](https://github.com/bestia-dev/cargo_auto_lib/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-79-orange.svg)](https://github.com/bestia-dev/cargo_auto_lib/)
//!
//! Hashtags: #rustlang #buildtool #developmenttool  
//! My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).
//!
//! ## Try it
//!
//! First, install the tool for task automation in Rust projects:
//!
//! ```bash
//! cargo install cargo-auto
//! ```
//!
//! Generate a new Rust CLI project:
//!
//! ```bash
//! cargo auto new_cli hello_world
//! ```
//!
//! Open the `hello_world` project in VSCode:
//!
//! ```bash
//! code hello_world
//! ```
//!
//! Open the generated directory `automation_tasks_rs` as an independent rust project in VSCode.
//!
//! ```bash
//! code hello_world/automation_tasks_rs
//! ```
//!
//! Now we can analyze the automation code. There is already this dependency inside `Cargo.toml` for our library:  
//!
//! ```toml
//! cargo_auto_lib="1.1.2"
//! ```
//!
//! Review the code in `automation_tasks_rs/main.rs`. The `cl::` namespace is the alias for `cargo_auto_lib`.  
//! Example:  
//!
//! ```rust ignore
//! /// cargo build --release
//! fn task_release() {
//! //    let cargo_toml = CargoToml::read();
//!     cl::auto_version_increment_semver_or_date();
//!     cl::auto_cargo_toml_to_md();
//!     cl::auto_lines_of_code("");
//!
//!     cl::run_shell_command("cargo fmt");
//!     cl::run_shell_command("cargo build --release");
//!     println!(
//!         r#"{YELLOW}
//!     After `cargo auto release`, run examples and tests
//!     if ok, then,{RESET}{GREEN}
//! cargo auto doc{RESET}{YELLOW}
//! {RESET}"#
//!     );
//!     print_examples_cmd();
//! }
//! ```
//!
//! You can see this function will increment the version in `Cargo.toml`.  
//! Then it will copy some data from `Cargo.toml` to README.md (title, description, version, author,...).  
//! It will count the lines of code and create badges in README.md.  
//! Then comes the traditional Rust part: cargo fmt and cargo build --release.  
//! Finally, it will show on the screen the instructions on how to continue developing.  
//!
//! Run (in your main rust project):
//!
//! ```bash
//! cargo auto release
//! ```
//!
//! Now open the README.md and you will see the data that this automation task copied from other places. Therefore you change this data only in one place, the automation task copies them wherever needed.
//!
//! ## Caveats
//!
//! This crate will attempt to edit `Cargo.toml`. Unfortunately, there's no great robust way right now to edit TOML file preserving formatting and comments and such, so right now I use just regex to do this.  
//! If you find that the heuristics don't work for you though please let me know and I'll try to check in a fix!
//!
//! ## Development details
//!
//! Read the development details in a separate md file:  
//! [DEVELOPMENT.md](https://github.com/bestia-dev/cargo_auto_lib/blob/main/DEVELOPMENT.md)
//!
//! ## Releases changelog
//!
//! Read the release changelog in a separate md file:  
//! [RELEASES.md](https://github.com/bestia-dev/cargo_auto_lib/blob/main/RELEASES.md)
//!
//! ## TODO
//!
//! Nothing big for now.
//!
//! ## Open-source and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
//! [//bestia.dev](https://bestia.dev)  
//! [//github.com/bestia-dev](https://github.com/bestia-dev)  
//! [//bestiadev.substack.com](https://bestiadev.substack.com)  
//! [//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
//!
// endregion: auto_md_to_doc_comments include README.md A //!

// region: bring traits in scope
use aes_gcm::aead::Aead;
use aes_gcm::AeadCore;
use aes_gcm::KeyInit;
use base64ct::Encoding;
use rsa::rand_core::RngCore;
use zeroize::Zeroize;

// endregion: bring traits in scope

use crate::auto_ssh_mod::EncryptedString;
use crate::SecretString;
// use crate::GREEN;
use crate::RED;
use crate::RESET;
use crate::YELLOW;

// region: new structs or newtype
pub struct SecretBytes<'a>(pub &'a mut [u8]);
// endregion: new structs or newtype

/// encrypt a token with the chosen ssh_identity and save as json encoded in Base64
///
/// use ssh-add to put ssh identity into ssh-agent
/// ```rust ignore
/// cl::encrypt_with_ssh_interactive_save_json("~/.ssh/github_com_ssh_1", "output1.ssh");
/// ```
pub(crate) fn encrypt_with_ssh_interactive_save_file(identity_file_path: &str, output_file_path: &str) {
    // internal function Generate a random password
    fn random_byte_password() -> [u8; 32] {
        let mut password = [0_u8; 32];
        aes_gcm::aead::OsRng.fill_bytes(&mut password);
        password
    }

    // internal function Encrypts token with secret_password_bytes
    fn encrypt_symmetric(token_is_a_secret: &SecretString, secret_password_bytes: &SecretBytes) -> Option<EncryptedString> {
        let data = token_is_a_secret.0.as_bytes();
        //only first 32 bytes
        let mut secret_password_32bytes = [0u8; 32];
        secret_password_32bytes.copy_from_slice(&secret_password_bytes.0[0..32]);

        let cipher = aes_gcm::Aes256Gcm::new(&secret_password_32bytes.into());

        let nonce = aes_gcm::Aes256Gcm::generate_nonce(&mut aes_gcm::aead::OsRng);
        match cipher.encrypt(&nonce, data) {
            Err(_err) => None,
            Ok(ciphertext) => {
                let mut encrypted_data = nonce.to_vec();
                encrypted_data.extend_from_slice(&ciphertext);
                let encrypted_string = base64ct::Base64::encode_string(&encrypted_data);
                Some(EncryptedString(encrypted_string))
            }
        }
    }

    let identity_file_path_expanded = crate::utils_mod::file_path_home_expand(identity_file_path);
    if !crate::utils_mod::file_exists(&identity_file_path_expanded) {
        eprintln!("{RED}Error: File {identity_file_path_expanded} does not exist! Exiting...{RESET}");
        // early exit
        return;
    }

    // fingerprints are calculated from the public key and are not a secret
    // offer to the user ssh-add if needed
    let fingerprint_from_file = crate::auto_github_mod::ssh_add_if_needed(&identity_file_path_expanded).unwrap_or_else(|| panic!("{RED}Identity not found in ssh-agent!{RESET}"));
    // SHA256:af123456789y1234553hmGEnN3fPv/iw6123456789M

    let path = std::env::var("SSH_AUTH_SOCK").expect("SSH_AUTH_SOCK is not set");
    let mut client = ssh_agent_client_rs::Client::connect(camino::Utf8Path::new(&path).as_std_path()).unwrap();
    let public_key = crate::auto_ssh_mod::ssh_add_list_contains_fingerprint(&mut client, &fingerprint_from_file).unwrap_or_else(|| panic!("{RED}Identity not found in ssh-agent!{RESET}"));
    let seed_bytes_not_a_secret = random_byte_password();
    let seed_string_not_a_secret = base64ct::Base64::encode_string(&seed_bytes_not_a_secret);

    let signature_is_the_new_secret_password = client.sign(&public_key, &seed_bytes_not_a_secret).unwrap();
    // only the data part of the signature goes into as_bytes.
    let mut secret_password_bytes = signature_is_the_new_secret_password.as_bytes().to_owned();
    let secret_password_bytes = SecretBytes(&mut secret_password_bytes);
    // signature_is_the_new_secret_password.zeroize;

    // input the token interactively
    let mut token_is_a_secret = SecretString(inquire::Password::new("Enter the token to encrypt:").without_confirmation().prompt().unwrap());

    // use this signed as password for symmetric encryption
    let encrypted_text = encrypt_symmetric(&token_is_a_secret, &secret_password_bytes).unwrap();
    token_is_a_secret.0.zeroize();
    secret_password_bytes.0.zeroize();
    // make json with 3 fields: fingerprint, seed, encrypted
    let json_value = serde_json::json!(
    {
        "identity": identity_file_path,
        "seed": seed_string_not_a_secret,
        "encrypted":encrypted_text.0,
    });
    let file_text = serde_json::to_string_pretty(&json_value).unwrap();
    let file_text = base64ct::Base64::encode_string(file_text.as_bytes());

    let output_file_path = crate::utils_mod::file_path_home_expand(output_file_path);
    let encrypted_file = camino::Utf8Path::new(&output_file_path);
    std::fs::write(encrypted_file, file_text).unwrap();
    println!("    {YELLOW}Encrypted text saved in file for future use.{RESET}")
}

/// decrypt from json file with ssh_identity
///
/// use ssh-add to put ssh identity into ssh-agent
/// ```rust ignore
/// let output = cl::decrypt_with_ssh_from_json("output1.json").unwrap();
/// ```
pub(crate) fn decrypt_with_ssh_from_file(json_file_path: &str) -> Option<SecretString> {
    // internal function Decrypts data with a key and a fingerprint
    fn decrypt_symmetric(encrypted_text: &EncryptedString, secret_password_bytes: SecretBytes) -> Option<SecretString> {
        let encrypted_bytes = base64ct::Base64::decode_vec(&encrypted_text.0).unwrap();
        //only first 32 bytes
        let mut secret_password_32bytes = [0u8; 32];
        secret_password_32bytes.copy_from_slice(&secret_password_bytes.0[0..32]);

        let cipher = aes_gcm::Aes256Gcm::new(&secret_password_32bytes.into());
        let nonce = sha2::digest::generic_array::GenericArray::from_slice(&encrypted_bytes[..12]);
        let ciphertext = &encrypted_bytes[12..];

        match cipher.decrypt(nonce, ciphertext) {
            Err(_err) => None,
            Ok(decrypted_bytes) => {
                let decrypted_string = String::from_utf8(decrypted_bytes).unwrap();
                Some(SecretString(decrypted_string))
            }
        }
    }

    // and now decrypt with private key
    println!("    {YELLOW}Decrypting GitHub API token from encrypted file.{RESET}");
    let json_file_path = crate::utils_mod::file_path_home_expand(json_file_path);
    let file_text = std::fs::read_to_string(json_file_path).unwrap();
    let file_text = base64ct::Base64::decode_vec(&file_text).unwrap();
    let file_text = String::from_utf8(file_text).unwrap();
    let json_value: serde_json::Value = serde_json::from_str(&file_text).unwrap();
    let identity_file_path: &str = json_value.get("identity").unwrap().as_str().unwrap();
    let seed_for_password_not_a_secret: &str = json_value.get("seed").unwrap().as_str().unwrap();
    let encrypted_text: &str = json_value.get("encrypted").unwrap().as_str().unwrap();

    let identity_file_path_expanded = crate::utils_mod::file_path_home_expand(identity_file_path);
    if !crate::utils_mod::file_exists(&identity_file_path_expanded) {
        eprintln!("{RED}Error: File {identity_file_path_expanded} does not exist! Exiting...{RESET}");
        // early exit
        return None;
    }

    // fingerprints are calculated from the public key and are not a secret
    // ssh-add only if needed
    let fingerprint_from_file = crate::auto_github_mod::ssh_add_if_needed(&identity_file_path_expanded).unwrap_or_else(|| panic!("{RED}Identity not found in ssh-agent!{RESET}"));
    // SHA256:af123456789y1234553hmGEnN3fPv/iw6123456789M

    let path = std::env::var("SSH_AUTH_SOCK").expect("SSH_AUTH_SOCK is not set");
    let mut client = ssh_agent_client_rs::Client::connect(camino::Utf8Path::new(&path).as_std_path()).unwrap();

    let public_key = crate::auto_ssh_mod::ssh_add_list_contains_fingerprint(&mut client, &fingerprint_from_file).unwrap_or_else(|| panic!("{RED}Identity not found in ssh-agent!{RESET}"));

    let seed_bytes = base64ct::Base64::decode_vec(seed_for_password_not_a_secret).unwrap();

    let signature_is_the_new_secret_password = client.sign(&public_key, &seed_bytes).unwrap();
    // only the data part of the signature goes into as_bytes.
    let mut secret_password_bytes = signature_is_the_new_secret_password.as_bytes().to_owned();
    let secret_password_bytes = SecretBytes(&mut secret_password_bytes);
    // signature_is_the_new_secret_password.zeroize;

    // use this signed as password for symmetric decryption
    let encrypted_text = EncryptedString(encrypted_text.to_string());

    let token_is_a_secret = decrypt_symmetric(&encrypted_text, secret_password_bytes);
    // return
    token_is_a_secret
}
