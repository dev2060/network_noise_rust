use std::net::{TcpStream, TcpListener, SocketAddr};
use std::io::{Read, BufReader, BufRead};
use std::thread;
use std::thread::JoinHandle;

use crate::utils;

pub fn start(bind_to: &str) -> Result<(), std::io::Error> {
	let tcp_listener = match TcpListener::bind(bind_to) {
		Ok(listener) => listener,
		Err(e) => { panic!("Can not start tcp listener: {}", e); }
	};
	println!("[Server_message] It is listening on {}. You can run another instance in client mode to interact", bind_to);

	match tcp_listener.accept() { // we need to listen just for one client here 
		Ok((stream, addr)) => { 
			println!("[Server_message] New client connection from: [{}]", addr);
			handle_client(stream);
		},
		Err(e) => eprintln!("Could not get a client: {}", e)
	}

	Ok(())
}

fn on_client_payload(stream: &TcpStream) -> Result<(), std::io::Error> {
    let mut buf_reader = BufReader::new(stream);
    let mut payload_buffer = [0; 256]; // fills with zeros from 0 to 255
	let mut total_bytes_received: usize = 0;
	let time_started: i64 = utils::get_current_time_in_mills();

	loop {
        let payload_len = buf_reader.read(&mut payload_buffer)?;
        total_bytes_received += payload_len;
        if payload_len == 0 { // means no incoming bytes
            break;
        }

	}
	
	let time_elapsed: i64 = utils::get_current_time_in_mills() - time_started;
	let bps = utils::get_bitrate(total_bytes_received as u64, time_elapsed);
	println!("Total bytes received: [{}]", total_bytes_received * 8);
	println!("Average bitrate: [{}] bits per second", bps);
	println!("Time elapsed: [{}] seconds", time_elapsed as f32/1000f32);
	
	// Ok(())
	Err(std::io::Error::new(std::io::ErrorKind::Other, "msg_len == 0")) // it breakes when buffer result is empty (zero bytes), this means stream is closed by user
}

fn handle_client(stream: TcpStream) -> Result<(), std::io::Error> {
    let stream_clone = stream.try_clone()?; // "try_clone?"gives a reference to the same stream that "stream" references

    let client_thread: JoinHandle<_> = thread::spawn(move || {
		match on_client_payload(&stream_clone) {
			Err(err) => eprintln!("[Server_warning] Client is disconnected: {}", err),
			Ok(_) => {;}
		}
    });
    client_thread.join().expect("Could not join on clients thread");

    Ok(())
}