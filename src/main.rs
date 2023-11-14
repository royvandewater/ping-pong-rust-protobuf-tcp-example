mod protos;

use std::{
    net::{TcpListener, TcpStream},
    thread::{self, JoinHandle},
};

use protobuf::{CodedInputStream, Message};
use protos::generated::pingpong::Ping;

use crate::protos::generated::pingpong::Pong;

fn main() {
    println!("Hello, world!");

    let server_handle = start_server();
    let client_handle = start_client();

    server_handle.join().unwrap();
    client_handle.join().unwrap();
}

fn start_client() -> JoinHandle<()> {
    println!("C: Starting... ");
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();
    println!("C: Connected.");

    thread::spawn(move || {
        let mut index = 0;
        let mut incoming_stream = stream.try_clone().unwrap();
        let mut incoming = CodedInputStream::new(&mut incoming_stream);

        loop {
            index += 1;

            if index > 100 {
                println!("C: Disconnecting");
                return;
            }

            let ping = new_ping(index);
            println!("C: Sending Ping! ({})", ping.index());
            ping.write_length_delimited_to_writer(&mut stream).unwrap();

            let pong: Pong = incoming.read_message().unwrap();
            println!("C: Received Pong! ({})", pong.index());
        }
    })
}

fn new_ping(index: i32) -> Ping {
    let mut ping = Ping::new();
    ping.set_index(index);
    ping
}

fn start_server() -> JoinHandle<()> {
    println!("S: Starting... ");
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("S: Started.");

    thread::spawn(move || {
        for stream_result in listener.incoming() {
            let mut stream = stream_result.unwrap();

            let mut incoming_stream = stream.try_clone().unwrap();
            let mut incoming = CodedInputStream::new(&mut incoming_stream);

            loop {
                if incoming.eof().unwrap() {
                    println!("S: Client disconnected.");
                    return;
                }
                let ping: Ping = incoming.read_message().unwrap();
                println!("S: Received Ping! ({})", ping.index());

                let mut pong = Pong::new();
                pong.set_index(ping.index());
                println!("S: Sending Pong! ({})", pong.index());
                pong.write_length_delimited_to_writer(&mut stream).unwrap();
            }
        }
    })
}
