//https://docs.rs/terminal-clipboard/latest/terminal_clipboard/
//https://docs.rs/clap/latest/clap/index.html

use clap::Parser;
use terminal_clipboard;
use std::net::TcpStream;
use std::io::{self, Write};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
    ///Please specify port as well
    #[arg(short, long)]
    ip: String,

    ///Change text inside of clipboard
    #[arg(short, long)]
    text: String,
}

fn main() {
    let args = Args::parse();

    terminal_clipboard::set_string(args.text).unwrap();
    let mut clipboard = terminal_clipboard::get_string().unwrap();

    println!("Sending to IP address: {}", args.ip);
    println!("Text in clipboard: {}", clipboard);

    send_clip(args.ip, clipboard);
}

fn send_clip(ip: String, clipb: String)
{
    let mut stream = TcpStream::connect(ip);
    stream.expect("Could not do thing").write(clipb.as_bytes());
}
