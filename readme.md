# Kinko

> CAA - Lab 02: Encrypted vault
>
> Author: Doran Kayoumi
> Teacher: Alexandre Duc
> Assistant. Nathan Séville
>
> Note: This project was done as a laboratory for the CAA class at the HEIG-VD



<p align="center">
<img src="docs/cover.jpg" width="500">
</p>
<p align="center">
  <img src="https://forthebadge.com/images/badges/built-with-love.svg" alt="version">
  <img src="https://forthebadge.com/images/badges/made-with-rust.svg" alt="platform">
  <img src="https://forthebadge.com/images/badges/makes-people-smile.svg" alt="platform">
  <img src="https://forthebadge.com/images/badges/built-by-hipsters.svg" alt="platform">
</p>

## Legend

Here's the legend for all the following diagrams explaining the architecture.

<p align="center">
<img src="docs/legend.png" width="500">
</p>

## Architecture

The architecture of `Kinko`  follows a **Client-Server** model with a (imaginary) 1.2/1.3 connection between them.

To make sure that the server knows nothing about the data the users upload to the vault, all of the key generation and file encryption/decryption will be done client side. The only cryptography the server will be doing is to validate the tags sent by the client.

### Client-Server communication

All of the requests made between the client and the server will have attached a tag of a session token (see Authentication chapter for more info on the token). The tag will be a **HMAC** of the session token using a **shared secret** as the key.

The shared secret is a derivation of the user' password done with **argon2id**.

### Authentication

The authentication will be done using a simple challenge response protocol. Once a user authenticates her/him self, the client will send the **name** of the user that's trying to login. The server will send back a **challenge** and the **salt** needed to (re)compute the shared secret. The client will (re)compute the shared secret, calculate the tag of the **challenge** and send it to the server. The server will check if the tag received is valid, if it is, the server will send back a **session token** (and an encrypted list of the names of all the files owned by the user).

<p align="center">
<img src="docs/authentication.png" width="500">
</p>

> Note: Forgot to add the the list of files returned..oops

### File upload

When a user uploads a file to the vault, it's first encrypted then sent along side the key used for the encryption (which is also encrypted).

<p align="center">
<img src="docs/file_upload.png" width="500">
</p>

#### Encryption

The file encryption will be done using **AES-GCM** and to guarantee longterm security, the keys will have a length of **256 bits**. Since the key is also stored on the server, it will also be encrypted, but this this using **ECIES**. The reason for using asymmetric cryptography for the key encryption is to simplify file sharing. **ECIES** was chosen to avoid having to also store huge keys (since we want to have long term security).

> Note: A new key is generated for each new file being uploaded

<p align="center">
<img src="docs/file_encryption.png" width="500">
</p>

> (1) file encryption
>
> (2) key encryption

### File download

For the download, the user gives the name of the file she/he wants and the server will send back the **encrypted file** and **encrypted key**.

> Note: ofc the server will check its the user requesting the file actually owns it.

<p align="center">
<img src="docs/file_download.png" width="500">
</p>

#### Decryption

The decryption is pretty straight forward. The symmetric key is decrypted using the users private key and then the file is decrypted using the decrypted symmetric key.

<p align="center">
<img src="docs/file_decryption.png" width="500">
</p>

### File sharing

To share a file, the only thing to do is to encrypt the files encryption key with the public key of the user we want to share the file with. 

