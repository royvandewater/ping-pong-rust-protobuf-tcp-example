# Ping Pong Rust Protobuf TCP Example

Example of how to use the `protobuf` library to send & read messages over a native Rust TCP stream.

## To Run

```shell
cargo run
```

It'll spin up a TCPListener on port 3000 in one thread, then connect to it in another thread. The client will send a Ping message to the server, which will respond with a Pong message. Both the client & server print what they send & receive. This repeats 100 times, then the program exits.


## UDP

There's also [a branch demonstrating how to send & receive protobuf messages via UDP](https://github.com/royvandewater/ping-pong-rust-protobuf-tcp-example/tree/udp).


