use std::net::{SocketAddrV4, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::cipher::Cipher;
use crate::cipher::password::EncodePassword;
use crate::secure_socket::SecureSocket;

pub struct LssLocal {
    socket: SecureSocket,
}

impl LssLocal {
    pub fn new(password: EncodePassword, local: SocketAddrV4, remote: SocketAddrV4) -> Self {
        let cipher = Cipher::new_symmetric(password);
        let socket = SecureSocket::new_local(cipher, local, remote);
        Self { socket }
    }

    pub fn run(local: LssLocal) {
        let listener = TcpListener::bind(local.socket.listen_addr).unwrap();
        let en = Arc::new(Mutex::new(local));
        loop {
            let r = Arc::clone(&en);
            let (stream, addr) = listener.accept().unwrap();
            thread::spawn(move|| {
                let s = r.lock().unwrap().socket;
                Self::handle_conn(s, stream);
            });

            // m.join().unwrap();
        }
    }

    fn handle_conn(socket: SecureSocket, mut local_stream: TcpStream) {
        let mut local_clone = local_stream.try_clone().unwrap();

        let mut remote_stream = socket.dial_remote().unwrap();
        let mut remote_clone = remote_stream.try_clone().unwrap();

        thread::spawn(move||{
            socket.encrypt_outgoing(&mut local_stream, &mut remote_stream).unwrap();
        });

        socket.decrypt_incoming(&mut remote_clone, &mut local_clone).unwrap();
    }
}