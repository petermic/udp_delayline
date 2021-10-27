use std::net::{UdpSocket,SocketAddr};

fn main() {
    let mut args = std::env::args().skip(1);
    let addr = args.next();
    let data = args.next();
    let mut src = args.next();
    if src.is_none() {
        src = Some(str::to_string("127.0.0.1:5555"));
    }
    if addr.is_some() && data.is_some() {
        let address = addr.unwrap();
        let datagram = data.unwrap();
        let source = src.unwrap();
        let socket = UdpSocket::bind(source.parse::<SocketAddr>().expect("UDP parse failed"))
            .expect("UDP socket creation failed");
        socket.send_to(datagram.as_bytes(),address).expect("send failed");
    }
    else {
        println!("Usage: ./injector [address] [data] [source address (opt.)]");
    }
}
