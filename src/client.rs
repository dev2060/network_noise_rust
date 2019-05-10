use std::net::{TcpStream, SocketAddr};
use std::thread;
use std::io::{Write, Read, stdin, BufWriter, BufReader};
use std::thread::JoinHandle;
use std::process::exit; //immediately terminate the current process; exit code is passed to the underlying OS to further handling by other process
use std::time::Duration;
use crate::utils;

pub fn start(connect_to: &str, duration: u32, bitrate: u32) -> Result<(), std::io::Error> {
	let stream = TcpStream::connect(connect_to)?; // question mark operator is required to propagate error to function caller
	let stream_clone = stream.try_clone()?;  // references to the same stream that original stream. Also I guess we do not need thread safe here
	let packages_thread: JoinHandle<_> = thread::spawn(move || { // "move" closure allows use data from main thread to another thread
		if let Err(err) = send_packages(&stream_clone, &duration, &bitrate) {
			eprintln!("Error while client running: {}", err);
			exit(0);
		}
		else {
			exit(1); // Everything works ok. Exit client mode
		}
	});

	user_interface();
	//packages_thread.join().expect("Could not join on packages thread"); // main thread waits until another thread ends
	// otherwise the new thread will be stopped when the main thread ends, whether or not it has finished running
	Ok(())
}

fn send_packages(stream: &TcpStream, duration: &u32, bitrate: &u32) -> Result<(), std::io::Error> {
    let mut writer = BufWriter::new(stream);

	let bitrate1 = *bitrate as f32 / utils::BITS_IN_BYTE as f32;
	let byets_per_interval: f32 = bitrate1 * utils::INTERVAL_OF_BROADCAST_IN_MILLS as f32 / utils::MILLS_IN_ONE_SECOND as f32 / 2.0f32;
	let byets_per_interval = byets_per_interval as u32;
	let message = (0..byets_per_interval).map(|_| "d").collect::<String>(); // fills with "d" as value with bytes_per_interval as length, then converts to Sring

	let dur = utils::MILLS_IN_ONE_SECOND * duration;
	let time_begin = utils::get_current_time_in_mills();
	let mut time_started: i64 = time_begin;
	let mut time_ended: i64 = time_begin;

	let mut i = 0;

	loop {
		i += 1;
		println!("{}", i);
		let mut bytes_to_broacast = byets_per_interval;
		while bytes_to_broacast > 0 {
			let sent_result = write!(writer, "{}", message);
			writer.flush()?;
			if let Ok(result) = sent_result {;}
			else {
				return Err(std::io::Error::new(std::io::ErrorKind::Other, "failed to send data"));
			}
			bytes_to_broacast -= byets_per_interval;
			
		}
		
        time_started = time_ended;
        time_ended = utils::get_current_time_in_mills();
        let time_elapsed = time_ended - time_begin;
        if time_elapsed >= dur as i64 {
        	break;
        }
        let time_remain = utils::INTERVAL_OF_BROADCAST_IN_MILLS as i64 - (time_ended - time_started); 
        if (time_remain > 0) {
        	thread::sleep(Duration::from_millis(time_remain as u64));
        }
	}

	//println!("on thread");
	println!("[Client_message] Data broadcast is finished with success");
	Ok(())
}

fn user_interface() -> Result<(), std::io::Error> {
	let mut buf = String::new();
	println!("[Client_message] You started in client mode.\n[Client_message] To exit just enter: \"quit\"\n");

	loop {
		buf.clear();
		let _ = stdin().read_line(&mut buf);
		if buf.starts_with("quit") {
			break;
		} 
	}

	Ok(())
}