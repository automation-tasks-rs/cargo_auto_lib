// auto_ssh_mod.rs

//! Work with ssh keys

pub type FingerprintString = String;
pub type IdentityFilePathString = String;
pub struct SecretString(pub String);
pub struct EncryptedString(pub String);

/// The work with ssh_agent_client_rs starts with a client
pub fn crate_ssh_agent_client() -> ssh_agent_client_rs::Client {
    let path = std::env::var("SSH_AUTH_SOCK").expect("SSH_AUTH_SOCK is not set");
    let client = ssh_agent_client_rs::Client::connect(camino::Utf8Path::new(&path).as_std_path()).unwrap();
    // return
    client
}

/// returns the public_key inside ssh-add or None
pub(crate) fn ssh_add_list_contains_fingerprint(client: &mut ssh_agent_client_rs::Client, fingerprint_from_file: &str) -> Option<ssh_key::PublicKey> {
    let vec_public_key = client.list_identities().unwrap();

    for public_key in vec_public_key.iter() {
        let fingerprint_from_agent = public_key.key_data().fingerprint(Default::default()).to_string();

        if fingerprint_from_agent == fingerprint_from_file {
            return Some(public_key.to_owned());
        }
    }
    None
}

/// the parameter is the filepath of the private key
/// But the code then uses the public key to calculate the fingerprint
pub(crate) fn get_fingerprint_from_file(identity_private_file_path: &str) -> FingerprintString {
    let identity_public_file_path = format!("{identity_private_file_path}.pub");

    let public_key = ssh_key::PublicKey::read_openssh_file(crate::utils_mod::to_path(&identity_public_file_path)).unwrap();
    let fingerprint = public_key.fingerprint(Default::default());
    let fingerprint = fingerprint.to_string();
    // return
    fingerprint
}
