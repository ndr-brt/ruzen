use rosc::OscPacket;
use std::sync::mpsc::Sender;
use std::net::UdpSocket;

pub struct OscServer {
    address: &'static str,
}

impl OscServer {
    pub fn new(address: &'static str) -> Self {
        OscServer { address }
    }

    pub fn listen_forever(&self, osc_sink: Sender<OscPacket>) {
        let sock = UdpSocket::bind(self.address).unwrap();
        let mut buf = [0u8; rosc::decoder::MTU];

        loop {
            match sock.recv_from(&mut buf) {
                Ok((size, addr)) => {
                    println!("Received packet with size {} from: {}", size, addr);
                    match rosc::decoder::decode(&buf[..size]) {
                        Ok(packet) => { osc_sink.send(packet); },
                        Err(e) => println!("Cannot decode osc packet; {:?}", e)
                    }
                }
                Err(e) => {
                    println!("Error receiving from socket: {}", e);
                    break;
                }
            }
        }
    }
}