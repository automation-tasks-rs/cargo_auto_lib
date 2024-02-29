// auto_encrypt_decrypt_with_ssh_mod.rs

//! How to save a GitHub TOKEN securely inside a file?

//! GitHub TOKEN is used by GitHub API to gain access (authentication and authorization) to your GitHub.
//! A TOKEN is a secret just like a password and it must be protected.
//! If the TOKEN is leaked, a mal-intentioned can gain access to everything, using the API.
//! Never store TOKENS in plain text anywhere!

//! The TOKEN must be encrypted before storing it.
//! We already use SSH keys to connect to GitHub. And we use ssh-agent for easy work with SSH identities.
//! I want to use the ssh key inside ssh-agent to encrypt the TOKEN and save it in a file.

//! The easy solution: encrypt with the Public key and then decrypt with the Private key.
//! Problem: ssh-agent can only sign a message with the private key. Nothing more.
//! It cannot decrypt with private key, because it would be a security risk.

//! The security is based on the assumption that only the owner of the ssh private key can sign the message.
//! The user already uses the ssh private key and it uses ssh-agent to connect over ssh to GitHub.
//! So the user already knows how important are ssh private keys and to keep them secure.

//! This could work also for other TOKENS, not just GitHub.

//! Encryption
//! 1. ssh-agent must contain the chosen ssh identity. Use ssh-add for this.
//! 2. Create a random message that will be used only by this code. It is not a secret.
//! 3. Sign this message. This will become the password for symmetric encryption. It is a secret.
//! 4. Input the token interactively in hidden input. It is a secret.
//! 5. Use the password to symmetric encrypt the token.
//! 6. Zeroize the token and password.
//! 7. Save the message and the encrypted token in a json file.

//! Decryption
//! 1. ssh-agent must contain the ssh identity. Use ssh-add for this.
//! 2. Read the json file, get the ssh_identity_file_path, message and the encrypted token.
//! 3. Find the right identity inside ssh-agent.
//! 4. Sign the message. This will become the password for symmetric decryption. It is a secret.
//! 5. Use this password to symmetric decrypt the token.
//! 6. Get the token.
//! 7. Zeroize the token and password.

// region: bring traits in scope
use aes_gcm::aead::Aead;
use aes_gcm::AeadCore;
use aes_gcm::KeyInit;
use base64ct::Encoding;
use rsa::rand_core::RngCore;
use zeroize::Zeroize;

// endregion: bring traits in scope

// use crate::GREEN;
use crate::RED;
use crate::RESET;
// use crate::YELLOW;

// region: new structs or newtype
pub struct SecretString(pub String);
pub struct EncryptedString(pub String);
pub struct SecretBytes<'a>(pub &'a mut [u8]);

// endregion: new structs or newtype

/// encrypt a token with the chosen ssh_identity and save as json file
/// use ssh-add to put ssh identity into ssh-agent
/// ```rust ignore
/// cl::encrypt_with_ssh_interactive_save_json("~/.ssh/github_com_ssh_1", "output1.json");
/// ```
pub(crate) fn encrypt_with_ssh_interactive_save_json(identity_file_path: &str, output_file_path: &str) {
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

    println!("Encrypt and save json file");

    let identity_file_path_expanded = crate::utils_mod::file_path_home_expand(identity_file_path);
    if !crate::utils_mod::file_exists(&identity_file_path_expanded) {
        eprintln!("{RED}File {identity_file_path_expanded} does not exist! Exiting.{RESET}");
        // early exit
        return;
    }

    // fingerprints are calculated from the public key and are not a secret
    // offer to the user ssh-add if needed
    let fingerprint_from_file = crate::auto_github_mod::ssh_add_if_needed(&identity_file_path_expanded).unwrap_or_else(|| panic!("Identity not found in ssh-agent!"));
    // SHA256:af123456789y1234553hmGEnN3fPv/iw6123456789M

    let path = std::env::var("SSH_AUTH_SOCK").expect("SSH_AUTH_SOCK is not set");
    let mut client = ssh_agent_client_rs::Client::connect(std::path::Path::new(&path)).unwrap();
    let public_key = crate::auto_ssh_mod::ssh_add_list_contains_fingerprint(&mut client, &fingerprint_from_file).unwrap_or_else(|| panic!("Identity not found in ssh-agent!"));
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

    let output_file_path = crate::utils_mod::file_path_home_expand(output_file_path);
    let encrypted_file = std::path::Path::new(&output_file_path);
    std::fs::write(encrypted_file, file_text).unwrap();
    println!("Encrypted text saved in Json file.")
}

/// decrypt from json file with ssh_identity
/// use ssh-add to put ssh identity into ssh-agent
/// ```rust ignore
/// let output = cl::decrypt_with_ssh_from_json("output1.json").unwrap();
/// dbg!(output);
/// ```
pub(crate) fn decrypt_with_ssh_from_json(json_file_path: &str) -> Option<SecretString> {
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
    println!("decrypt file from json");
    let json_file_path = crate::utils_mod::file_path_home_expand(json_file_path);
    let file_text = std::fs::read_to_string(json_file_path).unwrap();
    let json_value: serde_json::Value = serde_json::from_str(&file_text).unwrap();
    let identity_file_path: &str = json_value.get("identity").unwrap().as_str().unwrap();
    let seed_for_password_not_a_secret: &str = json_value.get("seed").unwrap().as_str().unwrap();
    let encrypted_text: &str = json_value.get("encrypted").unwrap().as_str().unwrap();

    let identity_file_path_expanded = crate::utils_mod::file_path_home_expand(identity_file_path);
    if !crate::utils_mod::file_exists(&identity_file_path_expanded) {
        eprintln!("{RED}File {identity_file_path_expanded} does not exist! Exiting.{RESET}");
        // early exit
        return None;
    }

    // fingerprints are calculated from the public key and are not a secret
    // ssh-add only if needed
    let fingerprint_from_file = crate::auto_github_mod::ssh_add_if_needed(&identity_file_path_expanded).unwrap_or_else(|| panic!("Identity not found in ssh-agent!"));
    // SHA256:af123456789y1234553hmGEnN3fPv/iw6123456789M

    let path = std::env::var("SSH_AUTH_SOCK").expect("SSH_AUTH_SOCK is not set");
    let mut client = ssh_agent_client_rs::Client::connect(std::path::Path::new(&path)).unwrap();

    let public_key = crate::auto_ssh_mod::ssh_add_list_contains_fingerprint(&mut client, &fingerprint_from_file).unwrap_or_else(|| panic!("Identity not found in ssh-agent!"));

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
