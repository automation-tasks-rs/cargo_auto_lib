// auto_shell_mod.rs

use crate::{
    error_mod::LibError,
    public_api_mod::{RED, RESET, YELLOW},
    ResultWithLibError,
};

/// similar to std::process::Output, but with i32 and Strings for easier work
#[derive(Debug)]
pub struct ShellOutput {
    /// The status (exit code) of the process.
    pub status: i32,
    /// The string that the process wrote to stdout.
    pub stdout: String,
    /// The string that the process wrote to stderr.
    pub stderr: String,
}

/// Run one shell command with static str
///
/// We trust the "developer" that he will not make "command injection" in his own code.
/// The problem that must be sanitized is always "user input".
/// Exit task execution if the command has Exit Status != 0.
/// A panic on this location means nothing. I want to panic in the caller location.
pub fn run_shell_command_static(shell_command: &'static str) -> ResultWithLibError<()> {
    if !shell_command.starts_with("echo ") && !shell_command.starts_with("printf ") {
        println!("    {YELLOW}$ {shell_command}{RESET}");
    }
    let status = std::process::Command::new("sh").arg("-c").arg(shell_command).spawn().unwrap().wait().unwrap();
    let exit_code = status.code().expect(&format!("{RED}Error. {RESET}"));
    if exit_code != 0 {
        return Err(LibError::ErrorFromString(format!("{RED}Error: run_shell_command {}. {RESET}", exit_code)));
    }
    Ok(())
}

/// Shell command builder with simple but limited sanitizer
///
/// The limited sanitization will panic if the value contains double quotes.
/// Command injections attack is possible because the shell command mixes executable code and data in a single string.
/// The attacker could format the "user input" data in a way that it transforms it into "executable code".
/// A true sanitization is hard to do in software. It would mean to understand all the intricacies of bash syntax?!
/// Another solution is to create a complex object model to have every command and data separated. Too complicated and developer unfriendly.
/// Instead here we take that the developer is a trusted person and he knows how to create the template correctly,
/// so that the placeholders are always de-facto delimited with double-quote inside the shell command.
/// This avoids the problem of injection of any other symbol except double-quotes.
/// The injection of double quote would finish the double-quote data and open the door tho write executable code.
/// It would be very complicated to check if "escaped double quotes" are or not correct in the context of the template.
/// So I don't allow them at all. This covers the vast majority of simple use cases.
/// Placeholders are delimited with curly brackets.
pub struct ShellCommandLimitedDoubleQuotesSanitizer {
    template: String,
}
impl crate::ShellCommandLimitedDoubleQuotesSanitizerTrait for ShellCommandLimitedDoubleQuotesSanitizer {
    /// Template for the shell command with placeholders
    ///
    /// The limited sanitization will panic if the value contains double quotes.
    /// Placeholders are delimited with curly brackets.
    /// The developer must be super careful to write the template correctly.
    /// The placeholders must be inside a block delimited with double quotes.
    /// In a way that only an injection of a double quote can cause problems.
    /// There is no software check of the correctness of the template.
    fn new(template: &str) -> Self {
        // just a quick check that there are double quotes in the template, that the developer didn't forget about it.
        if !template.contains("\"") {
            panic!("{RED}The template must contain double quotes around placeholders because otherwise it is susceptible to command injection in shell command.{RESET}")
        }
        ShellCommandLimitedDoubleQuotesSanitizer { template: template.to_string() }
    }
    /// Replace placeholders with the value
    ///
    /// The limited sanitization will panic if the value contains double quotes.
    /// Enter the placeholder parameter delimited with curly brackets.
    /// It would be very complicated to check if "escaped double quotes" are or not correct in the context of the template.
    /// So I don't allow them at all. This covers the vast majority of simple use cases.
    fn replace_placeholder_forbidden_double_quotes(&mut self, placeholder: &str, value: &str) {
        if value.contains("\"") {
            panic!("{RED}The {placeholder} must not contain a double quote because it could create a command injection in shell command.{RESET}")
        }
        self.template = self.template.replace(placeholder, value);
    }

    /// Run the sanitized command with no additional checks
    fn run(&self) {
        run_shell_command(&self.template);
    }
}

/// Run one shell command
///
/// Exit task execution if the command has Exit Status != 0.
/// TODO: vulnerable to command injection
pub fn run_shell_command(shell_command: &str) {
    if !shell_command.starts_with("echo ") && !shell_command.starts_with("printf ") {
        println!("    {YELLOW}$ {shell_command}{RESET}");
    }
    let status = std::process::Command::new("sh").arg("-c").arg(shell_command).spawn().unwrap().wait().unwrap();
    let exit_code = status.code().expect(&format!("{RED}Error. {RESET}"));
    if exit_code != 0 {
        panic!("{RED}Error: run_shell_command {}. {RESET}", exit_code);
    }
}

/// Run one shell command and return ShellOutput {exit_status, stdout, stderr}
///
/// TODO: vulnerable to command injection
pub fn run_shell_command_output(shell_command: &str) -> ShellOutput {
    if !shell_command.starts_with("echo ") && !shell_command.starts_with("printf ") {
        println!("   {YELLOW} $ {shell_command}{RESET}");
    }
    let output = std::process::Command::new("sh").arg("-c").arg(shell_command).output().unwrap();
    // return
    ShellOutput {
        status: output.status.code().unwrap(),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    }
}

/// Run one shell command and return true if success
///
/// TODO: vulnerable to command injection
pub fn run_shell_command_success(shell_command: &str) -> bool {
    if !shell_command.starts_with("echo ") && !shell_command.starts_with("printf ") {
        println!("    {YELLOW}$ {shell_command}{RESET}");
    }
    let status = std::process::Command::new("sh").arg("-c").arg(shell_command).status().unwrap();
    // return
    status.success()
}
