use std::net::{SocketAddrV4, TcpListener, TcpStream};
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
        let socket = SecureSocket::new(cipher, local, remote);
        Self { socket }
    }

    pub fn listen(&self){
        let listener = TcpListener::bind(&self.socket.listen_addr).unwrap();
        loop {
            let (stream,addr) = listener.accept().unwrap();
            thread::spawn(||{
                let socket: SecureSocket = self.socket;
                Self::handle_conn(socket, stream);
            });
        }
    }

    fn handle_conn(socket:&SecureSocket, stream:TcpStream){

    }
}