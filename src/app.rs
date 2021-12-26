use std::{
	sync::mpsc,
	net::TcpStream,
};

pub fn start(mut con: TcpStream, key: impl Cipher) {
	let (tx, rx) = mpsc::channel();
	start_reading(con.try_clone().expect("failed to split the tcp stream"), tx.clone());
	start_stdin(tx);
	
	while let Ok(ev) = rx.recv() {
		match ev {
			Event::Msg(mut bytes) => {
				println!("peer(encrypted): {}", bytes.to_string_lossy());
				key.decrypt_in_place(&mut bytes);
				println!("peer(plaintext): {}", bytes.to_string_lossy());
			}
			Event::File{name, compressed, mut data} => {
				println!("peer sent a file (filename: {}, compressed: {})", &name, compressed);
				println!("decrypting the file...");
				key.decrypt_in_place(&mut data);
				match save_file(&name, compressed, &data) {
					Err(e) => {
						println!("error saving the file: {}", e);
					println!("this could mean the key was incorrect");
				}
				Ok(path) => println!("saved the file to {}", path.display()),
				};
			}
			Event::Command(Command::Msg(msg)) => {
				if send_msg(&mut con, &msg, &key).is_err() {
					break;
				}
			}
			Event::Command(Command::File(path)) => {
				if !path.is_file() {
					println!("{}: file doesn't exist or is a directory", &data);
					continue;
				}
				println!("compressing and sending the file...");
				if send_file(&mut con, &path, &key).is_err() {
					break;
				}
				println!("sent {}", path.display());
			}
		}
	}
	
	println!("peer terminated the connection... exiting");
}

fn start_stdin(tx: Sender<Event>) {
	let stdin = io::stdin();
	print!("command> ");
	io::stdout().flush().unwrap();
	for res in stdin.lock().lines() {
		if let Ok(s) = res {
			match s.trim().parse::<Command>() {
				Ok(cmd) => tx.send(Event::Command(cmd)).unwrap_or_else(|_| std::process::exit(0)),
				Err(e) => eprintln!("error: {}", e),
			};
		}
		print!("command> ");
	io::stdout().flush().unwrap();
	}
}
