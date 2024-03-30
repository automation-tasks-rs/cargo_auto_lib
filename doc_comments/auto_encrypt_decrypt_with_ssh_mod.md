[//]: # (auto_md_to_doc_comments segment start A)

How to save a GitHub TOKEN securely inside a file?

GitHub TOKEN is used by GitHub API to gain access (authentication and authorization) to your GitHub.  
A TOKEN is a secret just like a password and it must be protected.  
If the TOKEN is leaked, a mal-intentioned can gain access to everything, using the API.  
Never store TOKENS in plain text anywhere!

The TOKEN must be encrypted before storing it.  

## SSH keys

We already use SSH keys to connect to GitHub. And we use ssh-agent for easy work with SSH identities.  
I want to use the SSH key inside ssh-agent to encrypt the TOKEN and save it in a file.

The easy solution: encrypt with the Public key and then decrypt with the Private key.  
Problem: ssh-agent can only sign a message with the private key. Nothing more.  
It cannot decrypt with private key, because it would be a security risk.

The security is based on the assumption that only the owner of the[]SSHprivate key can sign the message.  
The user already uses theSSHprivate key and it uses ssh-agent to connect over SSH to GitHub.  
So the user already knows how important are SSH private keys and to keep them secure.

This could work also for other TOKENS, not just GitHub.

Encryption

1. ssh-agent must contain the chosen SSH identity. Use ssh-add for this.  
2. Create a random message that will be used only by this code. It is not a secret.  
3. Sign this message. This will become the password for symmetric encryption. It is a secret.  
4. Input the token interactively in hidden input. It is a secret.  
5. Use the password to symmetric encrypt the token.  
6. Zeroize the token and password.  
7. Save the message and the encrypted token in a json file.

Decryption

1. ssh-agent must contain the SSH identity. Use ssh-add for this.  
2. Read the json file, get the ssh_identity_file_path, message and the encrypted token.  
3. Find the right identity inside ssh-agent.  
4. Sign the message. This will become the password for symmetric decryption. It is a secret.  
5. Use this password to symmetric decrypt the token.  
6. Get the token.  
7. Zeroize the token and password.

[//]: # (auto_md_to_doc_comments segment end A)
