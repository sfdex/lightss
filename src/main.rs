use std::env;
use std::net::SocketAddrV4;
use std::str::FromStr;
use ligntss::local::LssLocal;
use ligntss::server::LssServer;

const PASSWORD: [u8; 256] = [0u8; 256]; // Your password

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    if args.len() < 2 {
        println!("Running lss local");
        let local_addr = &format!("127.0.0.1:{}", 1090);
        let server_addr = "";
        let local = LssLocal::new(PASSWORD, SocketAddrV4::from_str(local_addr).unwrap(), SocketAddrV4::from_str(server_addr).unwrap());
        LssLocal::run(local);
        return;
    } else {
        println!("Running lss server");
        let local_addr = &format!("127.0.0.1:{}", 8765);
        let server = LssServer::new(PASSWORD, SocketAddrV4::from_str(local_addr).unwrap());
        LssServer::run(server);
    }
}
