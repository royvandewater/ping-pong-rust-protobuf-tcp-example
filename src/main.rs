mod protos;

use std::{
    net::UdpSocket,
    thread::{self, JoinHandle},
};

use protobuf::Message;
use protos::generated::pingpong::{Ping, Pong};

const SERVER_ADDR: &str = "127.0.0.1:3000";
const CLIENT_ADDR: &str = "127.0.0.1:3001";

fn main() {
    println!("Hello, world!");

    let server_handle = start_server();
    let client_handle = start_client();

    server_handle.join().unwrap();
    client_handle.join().unwrap();
}

fn start_client() -> JoinHandle<()> {
    println!("C: Starting... ");
    let socket = UdpSocket::bind(CLIENT_ADDR).unwrap();
    println!("C: Connected.");

    thread::spawn(move || {
        let mut index = 0;
        let mut send_buf = Vec::with_capacity(1024);
        let mut recv_buf = [0; 1024];

        loop {
            index += 1;
            if index > 100 {
                socket.send_to(&[0x04], SERVER_ADDR).unwrap();
                return;
            }

            let ping = new_ping(index);
            println!("C: Sending Ping! ({})", ping.index());

            ping.write_to_vec(&mut send_buf).unwrap();
            socket.send_to(&send_buf, SERVER_ADDR).unwrap();

            loop {
                let num_read = socket.recv(&mut recv_buf).unwrap();
                if num_read == 0 {
                    continue;
                }

                let bytes = &mut recv_buf[..num_read];
                let pong = Pong::parse_from_bytes(&bytes).unwrap();
                println!("C: Received Pong! ({})", pong.index());
                break;
            }
        }
    })
}

fn start_server() -> JoinHandle<()> {
    println!("S: Starting... ");
    let socket = UdpSocket::bind("127.0.0.1:3000").unwrap();
    println!("S: Started.");

    thread::spawn(move || {
        let mut buf = [0; 1024];

        loop {
            let (num_read, addr) = socket.recv_from(&mut buf).unwrap();
            if num_read == 0 {
                continue;
            }
            let bytes = &mut buf[..num_read];
            if bytes[0] == 0x04 {
                println!("S: Received shutdown signal.");
                return;
            }
            // println!("S: Received bytes: {:?}", bytes);

            let ping = Ping::parse_from_bytes(&bytes).unwrap();
            println!("S: Received Ping! ({})", ping.index());

            let pong = new_pong(ping.index());
            println!("S: Sending Pong! ({})", pong.index());

            let send_buf = pong.write_to_bytes().unwrap();
            socket.send_to(&send_buf, &addr).unwrap();
        }
    })
}

fn new_ping(index: i32) -> Ping {
    let mut ping = Ping::new();
    ping.set_index(index);
    ping
}

fn new_pong(index: i32) -> Pong {
    let mut pong = Pong::new();
    pong.set_index(index);
    pong
}
