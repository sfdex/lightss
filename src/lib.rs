mod cipher;
mod secure_socket;
pub mod local;
pub mod server;

pub fn init(){
    cipher::init()
}