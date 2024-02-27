// auto_encrypt_decrypt_with_ssh_mod.rs

//! How to save a GitHub TOKEN securely inside a file?

//! GitHub TOKEN is used by GitHub API to gain access (authentication and authorization) to your GitHub.
//! A TOKEN is a secret just like a password and it must be protected.
//! If the TOKEN is leaked, a mal-intentioned can gain access to everything, using the API.
//! Never store TOKENS in plain text anywhere!

//! The TOKEN must be encrypted before storing it.
//! We already use SSH keys to connect to GitHub. And we use ssh-agent for easy work with SSH identities.
//! I want to use the ssh key inside ssh-agent to encrypt the TOKEN and save it in a file.

//! Inspired by https://!github.com/ssh-vault/ssh-vault

//! The easy solution: encrypt with the Public key and then decrypt with the Private key.
//! Problem: ssh-agent can only sign a message with the private key. Nothing more.
//! It cannot decrypt with private key, because it would be a security risk.

//! The security is based on the assumption that only the owner of the ssh private key can sign the message.
//! The user already uses the ssh private key and it uses ssh-agent to connect over ssh to GitHub.
//! So the user already knows how important are ssh private keys and to keep them secure.

//! This could work also for other TOKENS, not just GitHub.

//! Encryption
//! 1. ssh-agent must contain the ssh identity. Use ssh-add for this.
//! 2. Create a random message that will be used only by this code. It is not a secret.
//! 3. Sign this message. This will become the password for symmetric encryption. It is a secret.
//! 4. Input the token as a hidden input. It is a secret.
//! 5. Use the password to symmetric encrypt the token.
//! 6. Zeroize the token and password.
//! 7. Save the message and the encrypted token in a json file.

//! Decryption
//! 1. ssh-agent must contain the ssh identity. Use ssh-add for this.
//! 2. Read the json file, get the message and the encrypted token.
//! 3. Sign this message. This will become the password for symmetric decryption. It is a secret.
//! 4. Use this password to symmetric decrypt the token.
//! 5. Get the token.
//! 6. Zeroize the token and password.

use aes_gcm::aead::Aead;
use aes_gcm::AeadCore;
use aes_gcm::KeyInit;
use base64ct::Encoding;
use sha2::digest::generic_array::GenericArray;
use tokio_util::bytes::Buf;
use zeroize::Zeroize;

// use crate::GREEN;
use crate::RED;
use crate::RESET;
// use crate::YELLOW;

// region: new structs or newtype
pub struct SecretString(pub String);
// exactly 32 bytes?
pub struct Secret32Bytes([u8; 32]);

// endregion: new structs or newtype

// bring trait in scope
// use zeroize::Zeroize;

/// encrypt and save json file
pub fn encrypt_with_ssh_and_save_json(identity_file_path: &str, mut token_is_a_secret: SecretString, output_file_path: &str) {
    println!("encrypt and save json file");

    let identity_file_path = identity_file_path.replace("~", crate::home_dir().to_string_lossy().as_ref());

    // check if the file exists
    if !std::path::Path::new(&identity_file_path).exists() {
        eprintln!("{RED}File {identity_file_path} does not exist! Exiting.{RESET}");
        // early exit
        return;
    }
    // fingerprints are calculated from the public key and are not a secret
    // ssh-add only if needed
    let fingerprint_from_file = crate::auto_github_mod::ssh_add_if_needed(identity_file_path.to_string()).unwrap();
    // SHA256:af123456789y1234553hmGEnN3fPv/iw6123456789M

    let path = std::env::var("SSH_AUTH_SOCK").expect("SSH_AUTH_SOCK is not set");
    let mut client = ssh_agent_client_rs::Client::connect(std::path::Path::new(&path)).unwrap();

    let vec_public_key = client.list_identities().unwrap();
    let mut public_key: Option<&ssh_key::public::PublicKey> = None;
    for k in vec_public_key.iter() {
        let fingerprint_from_agent = k.key_data().fingerprint(Default::default()).to_string();
        if fingerprint_from_agent == fingerprint_from_file {
            public_key = Some(k);
            break;
        }
    }
    let Some(public_key) = public_key else {
        // early exit
        panic!("ssh-agent does not have the identity file.");
    };

    let seed_for_password_not_a_secret = "random value";
    let seed_bytes = seed_for_password_not_a_secret.as_bytes();

    let signature_is_the_new_secret_password = client.sign(public_key, seed_bytes).unwrap();
    // only the data part of the signature goes into as_bytes.
    let mut secret_password_bytes = signature_is_the_new_secret_password.as_bytes().to_owned();
    // signature_is_the_new_secret_password.zeroize;

    // use this signed as password for symmetric encryption
    let encrypted_text = encrypt_symmetric(&token_is_a_secret, &secret_password_bytes);
    token_is_a_secret.0.zeroize();
    secret_password_bytes.zeroize();
    // make json with 3 fields: fingerprint, seed, encrypted
    let json_value = serde_json::json!(
    {
        "identity": identity_file_path,
        "seed": seed_for_password_not_a_secret,
        "encrypted":encrypted_text
    });
    let file_text = serde_json::to_string_pretty(&json_value).unwrap();

    let encrypted_file = std::path::Path::new(output_file_path);
    std::fs::write(encrypted_file, file_text).unwrap();
    println!("Encrypted text saved in Json file.")
}

/// decrypt from json file
pub fn decrypt_with_ssh_from_json(json_file_path: &str) -> Option<String> {
    // and now decrypt with private key
    println!("decrypt file from json");
    let file_text = std::fs::read_to_string(json_file_path).unwrap();
    let json_value: serde_json::Value = serde_json::from_str(&file_text).unwrap();
    let identity_file_path: &str = json_value.get("identity").unwrap().as_str().unwrap();
    let seed_for_password_not_a_secret: &str = json_value.get("seed").unwrap().as_str().unwrap();
    let encrypted_text: &str = json_value.get("encrypted").unwrap().as_str().unwrap();
    dbg!(identity_file_path);

    // fingerprints are calculated from the public key and are not a secret
    // ssh-add only if needed
    let fingerprint_from_file = crate::auto_github_mod::ssh_add_if_needed(identity_file_path.to_string()).unwrap();
    // SHA256:af123456789y1234553hmGEnN3fPv/iw6123456789M

    let path = std::env::var("SSH_AUTH_SOCK").expect("SSH_AUTH_SOCK is not set");
    let mut client = ssh_agent_client_rs::Client::connect(std::path::Path::new(&path)).unwrap();

    let vec_public_key = client.list_identities().unwrap();
    let mut public_key: Option<&ssh_key::public::PublicKey> = None;
    for k in vec_public_key.iter() {
        let fingerprint_from_agent = k.key_data().fingerprint(Default::default()).to_string();
        if fingerprint_from_agent == fingerprint_from_file {
            public_key = Some(k);
            break;
        }
    }
    let Some(public_key) = public_key else {
        // early exit
        panic!("ssh-agent does not have the identity file.");
    };

    let seed_bytes = seed_for_password_not_a_secret.as_bytes();
    let signature_is_the_new_secret_password = client.sign(public_key, seed_bytes).unwrap();
    // only the data part of the signature goes into as_bytes.
    let mut secret_password_bytes = signature_is_the_new_secret_password.as_bytes().to_owned();
    // signature_is_the_new_secret_password.zeroize;

    // use this signed as password for symmetric decryption
    let token_is_a_secret = decrypt_symmetric(&encrypted_text, &secret_password_bytes);
    // return
    token_is_a_secret
}

// Encrypts token with secret_password_bytes
fn encrypt_symmetric(token_is_a_secret: &SecretString, secret_password_bytes: &[u8]) -> Option<String> {
    let data = token_is_a_secret.0.as_bytes();
    //only first 32 bytes
    let mut secret_password_32bytes = [0u8; 32];
    secret_password_32bytes.copy_from_slice(&secret_password_bytes[0..32]);

    let cipher = aes_gcm::Aes256Gcm::new(&secret_password_32bytes.into());

    let nonce = aes_gcm::Aes256Gcm::generate_nonce(&mut aes_gcm::aead::OsRng);
    match cipher.encrypt(&nonce, data) {
        Err(_err) => None,
        Ok(ciphertext) => {
            let mut encrypted_data = nonce.to_vec();
            encrypted_data.extend_from_slice(&ciphertext);
            let encrypted_string = base64ct::Base64::encode_string(&encrypted_data);
            Some(encrypted_string)
        }
    }
}

// Decrypts data with a key and a fingerprint
fn decrypt_symmetric(encrypted_text: &str, secret_password_bytes: &[u8]) -> Option<String> {
    let encrypted_bytes = base64ct::Base64::decode_vec(encrypted_text).unwrap();
    //only first 32 bytes
    let mut secret_password_32bytes = [0u8; 32];
    secret_password_32bytes.copy_from_slice(&secret_password_bytes[0..32]);

    let cipher = aes_gcm::Aes256Gcm::new(&secret_password_32bytes.into());
    let nonce = GenericArray::from_slice(&encrypted_bytes[..12]);
    let ciphertext = &encrypted_bytes[12..];

    match cipher.decrypt(nonce, ciphertext) {
        Err(_err) => None,
        Ok(decrypted_bytes) => {
            let decrypted_string = String::from_utf8(decrypted_bytes).unwrap();
            Some(decrypted_string)
        }
    }
}
