use std::env;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::thread;

fn check_port(addr: String) {
    let timeout = Duration::from_secs(1);

    let socket = addr.to_socket_addrs().unwrap().next();

    if let Some(sock) = socket {
        match TcpStream::connect_timeout(&sock, timeout) {
            Ok(_) => println!("[OPEN]  {}", addr),
            Err(_) => println!("[CLOSED] {}", addr),
        }
    } else {
        println!("[INVALID] {}", addr);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage:");
        println!("cargo run -- <host> <port1,port2,...>");
        return;
    }

    let host = &args[1];
    let ports: Vec<&str> = args[2].split(',').collect();

    let mut handles = vec![];

    for port in ports {
        let addr = format!("{}:{}", host, port);

        let handle = thread::spawn(move || {
            check_port(addr);
        });

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
}