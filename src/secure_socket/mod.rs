use std::io::{Read, Write};
use std::io::Result;
use std::net::{SocketAddrV4, TcpStream};
use crate::cipher::Cipher;

const BUFF_SIZE: usize = 1024;

pub struct SecureSocket {
    cipher: Cipher,
    listen_addr: SocketAddrV4,
    remote_addr: SocketAddrV4,
}

impl SecureSocket {
    pub fn encrypt_copy(&self, plain_stream: &mut TcpStream, encrypted_stream: &mut TcpStream) -> Result<usize> {
        let mut buf = vec![0; BUFF_SIZE];
        loop {
            let r = plain_stream.read(&mut buf);
            if let Err(err) = r {
                return Err(err);
            }
            self.cipher.encode(&mut buf);

            let n = r.ok().unwrap();

            self.cipher.encode(&mut buf[..n]);
            if let Err(err) = encrypted_stream.write(&mut buf[..n]) {
                return Err(err);
            }
        }
    }

    pub fn decrypt_copy(&self, encrypted_stream: &mut TcpStream, plain_stream: &mut TcpStream) -> Result<()> {
        let mut buf = vec![0; BUFF_SIZE];
        loop {
            let r = encrypted_stream.read(&mut buf);
            if let Err(err) = r {
                return Err(err);
            }
            let n = r.ok().unwrap();
            self.cipher.decode(&mut buf[..n]);
            if let Err(err) = plain_stream.write_all(&buf[..n]) {
                return Err(err);
            }
        }
    }

    pub fn dial_remote(&self) -> Result<TcpStream>{
        TcpStream::connect(format!("{}:{}",&self.remote_addr.ip(),&self.remote_addr.port()))
    }
}


