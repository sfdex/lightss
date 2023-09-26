use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::{net, thread};
use crate::cipher::Cipher;
use crate::cipher::password::EncodePassword;
use crate::secure_socket::SecureSocket;

pub struct LssServer {
    socket: SecureSocket,
}

impl LssServer {
    pub fn new(password: EncodePassword, local: SocketAddrV4) -> Self {
        let cipher = Cipher::new_symmetric(password);
        let socket = SecureSocket::new_server(cipher, local);
        Self { socket }
    }

    pub fn run(local: LssServer) {
        let listener = TcpListener::bind(local.socket.listen_addr).unwrap();
        let en = Arc::new(Mutex::new(local));
        loop {
            let r = Arc::clone(&en);
            let (stream, addr) = listener.accept().unwrap();
            thread::spawn(move || {
                let s = r.lock().unwrap().socket;
                Self::handle_conn(s, stream);
            });

            // m.join().unwrap();
        }
    }

    fn handle_conn(socket: SecureSocket, mut encrypted_stream: TcpStream) {
        let mut buf = [0u8; 3];
        encrypted_stream.read_exact(&mut buf).unwrap();

        // Ver
        if buf[0] != 5 {
            println!("Not socks5 (stage 1)");
            return;
        }

        let handshake_resp = [5, 0];
        encrypted_stream.write_all(handshake_resp.as_slice()).unwrap();


        let mut buf = vec![0; 1024];
        encrypted_stream.read_exact(&mut buf).unwrap();

        // Ver
        if buf[0] != 5 {
            println!("Not socks5 (stage 2)");
            return;
        }

        // Command
        if buf[1] != 1 {
            panic!("Not CONNECT command")
        }

        // Atyp
        if buf[3] != 1 {
            panic!("Not ip addr")
        }

        // Ip
        let ip = Ipv4Addr::new(buf[4], buf[5], buf[6], buf[7]);
        let mut pbo = [0u8; 2];
        pbo.copy_from_slice(&buf[8..10]);
        let port = u16::from_be_bytes(pbo);
        let ip = SocketAddrV4::new(ip, port);

        let request_resp = [5, 0, 0, 1, buf[4], buf[5], buf[6], buf[7], buf[8], buf[9]];
        encrypted_stream.write_all(request_resp.as_slice()).unwrap();


        let mut encrypted_clone = encrypted_stream.try_clone().unwrap();

        let mut remote_stream = TcpStream::connect(ip).expect(&format!("Dial remote error: {:?}", ip));
        let mut remote_clone = remote_stream.try_clone().unwrap();

        thread::spawn(move || {
            socket.decrypt_incoming(&mut remote_clone, &mut encrypted_clone).unwrap();
        });

        socket.encrypt_outgoing(&mut encrypted_stream, &mut remote_stream).unwrap();
    }
}