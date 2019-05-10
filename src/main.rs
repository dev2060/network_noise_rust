use std::env;
use clap::{Arg, App, SubCommand};

mod utils;
mod server;
mod client;

fn main() {
    let ip = "0.0.0.0";
    let port_to_bind = "8085";
    let duration_default = "10000";//10 seconds
    let bitrate_default = "20";

    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
    	panic!("Wrong arguments. Please try again."); //we have to explicitly set start mode that is why command key is required to specify start mode as well as port key to specify port
    }

    let matches = App::new("NNGENERATOR")
                          .version("0.1.0")
                          .author("Vitali Grabovski <vetal2060@gmail.com>")
                          .arg(Arg::with_name("server")
                               .short("s")
                               .long("server")
                               .help("Start in Server mode.")
                               .multiple(false)
                               .takes_value(false))
                          .arg(Arg::with_name("port")
                               .short("p")
                               .long("port")
                               .required(false)
                               .help("Set either port to bind in server mode or port to connect in client mode. (Example: -p 5677)")
                               .multiple(false)
                               .takes_value(true))
                          .arg(Arg::with_name("client")
                               .short("c")
                               .long("client")
                               .multiple(false)
                               .help("Start in Client mode."))
                          .arg(Arg::with_name("duration")
                               .short("d")
                               .long("duration")
                               //.multiple(false)
                               .takes_value(true)
                               .help("Number of seconds to noise generation."))
                          .arg(Arg::with_name("bitrate")
                               .short("b")
                               .long("bitrate")
                               //.multiple(false)
                               .takes_value(true)
                               .help("Number of Bitrate of noise generation."))
                          .get_matches();

    // Gets a values for options if supplied by user, or defaults in some cases
    let is_server = matches.is_present("server");
    let is_client = matches.is_present("client");
    let is_port_attached = matches.is_present("port");
    let is_duration_attached = matches.is_present("duration");
    let is_bitrate_attached = matches.is_present("bitrate");

    //let port = if (port > 80 && port < 65535) {port} else {port_to_bind};
    let port = matches.value_of("port").unwrap_or(&port_to_bind);
    let duration = matches.value_of("duration").unwrap_or(&duration_default);
    let duration: u32 = duration.parse().expect("Number required"); // panics with a custom message
    let bitrate = matches.value_of("bitrate").unwrap_or(&bitrate_default);
    let bitrate: u32 = bitrate.parse().expect("Number required");

    if is_server && !is_port_attached {
    	panic!("You have to specify port (example: -p 5677) while starting server mode. Please try again")
    }

    if is_client && !(is_port_attached && is_duration_attached && is_bitrate_attached) {
        eprintln!("You have to specify port (example: -p 5677), duration (example: -d 10), bitrate (example: -b 20) of server where you want to connect. It will continue with default configuration.\n");
    }

    let sock_addr = format!("{}:{}", ip, port);

    if is_server {
    	match server::start(&sock_addr[..]) {
    		Ok(_) => {},
    		Err(e) => { panic!("Server error happened while server starting: {}", e); }
    	}
    }
    else if is_client {
    	match client::start(&sock_addr[..], duration, bitrate) {
     		Ok(_) => (),
    		Err(e) => { panic!("Client error happened: {}", e); } //I guess this is unreachable   		
    	}
    }

}
