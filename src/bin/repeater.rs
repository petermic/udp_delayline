use std::net::{UdpSocket,SocketAddr};

fn main()
{
    let mut args = std::env::args().skip(1);
    let home_addr = args.next();
    let line_addr = args.next();
    if home_addr.is_some() && line_addr.is_some(){
        let home_address = home_addr.unwrap();
        let line_address = line_addr.unwrap();
        let socket = UdpSocket::bind(home_address.parse::<SocketAddr>().expect("parse failed"))
            .expect("UDP home bind failed");
        loop {
            let mut buffer = [0; 128];
            let laddr = line_address.clone();
            match socket.recv_from(&mut buffer) {
                Ok((nbytes,src_address)) => {
                    println!("received '{}' ({} bytes) from {}",
                    std::str::from_utf8(&buffer[0..nbytes]).expect("UTF8 parse error"),nbytes,src_address);
                    socket.send_to(&buffer[0..nbytes],laddr).expect("send failed");
                }
                Err(e) => {
                    println!("{:?}",e);
                }
            }
        }
    }
    else {
        println!("Usage: ./repeater [home address] [line address]");
    }
}
