use clap::Parser;
use terminal_clipboard;
use std::net::TcpStream;
use std::net::TcpListener;
use std::io::{self, Write};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
    ///IP and port to bind too. Format: 1.1.1.1:69
    #[arg(short, long)]
    ip_port: String,

    ///Used to only accept communication from a certain IP
    #[arg(short, long)]
    access_ip: String,
}

fn main() {

    let args = Args::parse();

    let listener = TcpListener::bind(args.ip_port).expect("CANT DO");

    for stream in listener.incoming() 
    {
        match stream {
            Ok(stream) => {
                let peer_ip = stream.peer_addr().unwrap().ip();
                let access_ip = match args.access_ip.parse::<std::net::IpAddr>() {
                    Ok(ip) => ip,
                    Err(_) => {
                        println!("Invalid access IP address specified.");
                        return;
                    }
                };
                if peer_ip == access_ip{
                    handle_client(stream);
                } else {
                    println!("Connection from {} not allowed.", peer_ip);
                    // Optionally, you can send a rejection message to the client here.
                }
            }
            Err(e) => { println!("Connection failed"); }
        }
    }
}

fn handle_client(stream: TcpStream)
{
    //Code here
}


