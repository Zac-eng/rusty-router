use std::net::SocketAddrV6;
use std::io::{Read, Write};
use socket2::{Socket, Domain, Type, Protocol};
mod crypto;
mod socket;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let socket = Socket::new(Domain::IPV6, Type::STREAM, Some(Protocol::MPTCP))?;
//     socket.set_reuse_address(true)?;
//     let addr = SocketAddrV6::new("fe80::625c:ce34:15fd:c2ca".parse().unwrap(), 8080, 0, 0);
//     socket.bind(&addr.into())?;
//     let dst = SocketAddrV6::new("fe80::2cf1:b884:abc:8b74".parse().unwrap(), 8080, 0, 0);
//     while socket.connect(&dst.into()).is_err() {
//         continue;
//     }
//     socket.send(b"this is a test")?;
//     Ok(())
// }


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = Socket::new(Domain::IPV6, Type::STREAM, Some(Protocol::MPTCP))?;
    socket.set_reuse_address(true)?;
    let addr = SocketAddrV6::new("fe80::2cf1:b884:abc:8b74".parse().unwrap(), 8080, 0, 0);
    socket.bind(&addr.into())?;
    socket.listen(5)?;
    let (mut client_socket, client_addr) = socket.accept()?;
    println!("Accepted connection from: {:?}", client_addr);
    let mut buffer = [0; 1024];
    let bytes_read = client_socket.read(&mut buffer)?;
    println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
    Ok(())
}
