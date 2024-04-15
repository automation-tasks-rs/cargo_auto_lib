// auto_crates_io_mod.rs

//! push versions to crates.io

// bring trait into scope
use secrecy::ExposeSecret;

use crate::BLUE;
use crate::RED;
use crate::RESET;
use crate::YELLOW;

/// Publish to crates.io
///
/// Encrypt/decrypt the crates.io token with the GitHub SSH key.
/// Then call the `cargo publish --token token` command.
/// Never show the secret token anywhere.
pub fn publish_to_crates_io_with_secret_token() {
    let token = check_or_get_crates_io_token().unwrap();
    // don't show the token to the user
    println!("    {YELLOW}cargo publish with token{RESET}");
    let shell_command = format!("cargo publish --token {}", token.expose_secret());
    let status = std::process::Command::new("sh").arg("-c").arg(shell_command).spawn().unwrap().wait().unwrap();
    let exit_code = status.code().expect(&format!("{RED}Error: publish to crates.io error. {RESET}"));
    if exit_code != 0 {
        eprintln!("{RED}Error: publish to crates.io error {exit_code}. {RESET}");
        std::process::exit(1);
    }
}

/// Ask user interactively to type the token
fn check_or_get_crates_io_token() -> Option<secrecy::SecretString> {
    println!(
        r#"
    {YELLOW}The token is required to publish to crates.io.
    You can generate the token at https://crates.io/settings/tokens.
    The token is a secret just like a password, use it with caution.{RESET}
    "#
    );
    let token_is_a_secret = secrecy::SecretString::from(inquire::Password::new(&format!("{BLUE}Enter the crates.io API token:{RESET}")).without_confirmation().prompt().unwrap());
    // return
    Some(token_is_a_secret)
}
