use std::io::{Read, Write};
use std::io::Result;
use std::net::{SocketAddrV4, TcpStream};
use std::str::FromStr;
use crate::cipher::Cipher;
use crate::cipher::password::EncodePassword;

const BUFF_SIZE: usize = 1024;

// #[derive(Copy, Clone)]
#[derive(Copy)]
pub struct SecureSocket {
    cipher: Cipher,
    pub listen_addr: SocketAddrV4,
    pub remote_addr: SocketAddrV4,
}

impl Clone for SecureSocket {
    fn clone(&self) -> Self {
        *self
    }
}

impl SecureSocket {
    pub fn new_local(cipher: Cipher, listen_addr: SocketAddrV4, remote_addr: SocketAddrV4) -> Self {
        Self { cipher, listen_addr, remote_addr }
    }

    pub fn new_server(cipher: Cipher, listen_addr: SocketAddrV4) -> Self {
        Self { cipher, listen_addr, remote_addr: SocketAddrV4::from_str("0.0.0.0:1080").unwrap() }
    }

    pub fn encrypt_outgoing(&self, plain_stream: &mut TcpStream, encrypted_stream: &mut TcpStream) -> Result<()> {
        let mut buf = vec![0; BUFF_SIZE];
        loop {
            let r = plain_stream.read(&mut buf);
            let mut bytes = match r {
                Ok(0) => { return Ok(()); }
                Ok(n) => { &buf[..n] }
                Err(err) => { return Err(err); }
            };

            self.cipher.encode(&mut bytes);
            if let Err(err) = encrypted_stream.write(&mut bytes) {
                return Err(err);
            }
        }
    }

    pub fn decrypt_incoming(&self, encrypted_stream: &mut TcpStream, plain_stream: &mut TcpStream) -> Result<()> {
        let mut buf = vec![0; BUFF_SIZE];
        loop {
            let r = encrypted_stream.read(&mut buf);
            let mut bytes = match r {
                Ok(0) => { return Ok(()); }
                Ok(n) => { &buf[..n] }
                Err(err) => { return Err(err); }
            };
            self.cipher.decode(&mut bytes);
            if let Err(err) = plain_stream.write_all(bytes) {
                return Err(err);
            }
        }
    }

    pub fn dial_remote(&self) -> Result<TcpStream> {
        TcpStream::connect(&self.remote_addr)
    }
}


