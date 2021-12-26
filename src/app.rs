use std::{
	io::{
		self,
		BufRead,
		Read,
		Write,
	},
	net::TcpStream,
	sync::mpsc::{
		self,
		Sender,
	},
	thread,
};

use bincode::ErrorKind;

use crate::{
	client::{
		save_file,
		send_file,
		send_msg,
	},
	encryption::Cipher,
	proto::{
		Header,
		HeaderKind,
	},
	session::{
		Command,
		Event,
	},
};

pub fn start<K: Cipher>(mut con: TcpStream, key: K) {
	let (tx, rx) = mpsc::channel();
	start_reading(
		con.try_clone().expect("failed to split the tcp stream"),
		tx.clone(),
	);
	start_stdin(tx);

	while let Ok(ev) = rx.recv() {
		match ev {
			Event::Msg(mut bytes) => {
				println!("peer(encrypted): {}", String::from_utf8_lossy(&bytes));
				key.decrypt_in_place(&mut bytes);
				println!("peer(plaintext): {}", String::from_utf8_lossy(&bytes));
			}
			Event::File {
				name,
				compressed,
				mut data,
			} => {
				println!("decrypting the file...");
				key.decrypt_in_place(&mut data);
				match save_file(&name, compressed, &data) {
					Err(e) => {
						eprintln!("error saving the file: {}", e);
						eprintln!("this could mean the key was incorrect");
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
					println!("{}: file doesn't exist or is a directory", path.display());
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
	thread::spawn(move || {
		let stdin = io::stdin();
		print!("command> ");
		io::stdout().flush().unwrap();
		for res in stdin.lock().lines() {
			if let Ok(s) = res {
				match s.trim().parse::<Command>() {
					Ok(cmd) => tx
						.send(Event::Command(cmd))
						.unwrap_or_else(|_| std::process::exit(0)),
					Err(e) => eprintln!("error: {}", e),
				};
			}
			print!("command> ");
			io::stdout().flush().unwrap();
		}
	});
}

fn start_reading(mut con: TcpStream, tx: Sender<Event>) {
	thread::spawn(move || {
		loop {
			match bincode::deserialize_from::<_, Header>(&mut con) {
				Err(e) => match *e {
					ErrorKind::Io(_) => break,
					_ => {
						eprintln!("clients are out of sync; app needs to be restarted");
						std::process::exit(0);
					}
				},
				Ok(Header { kind, len }) => {
					// read len amount of bytes from the connection.
					if let HeaderKind::File { name, .. } = &kind {
						println!("peer sent a file ({})... downloading", &name);
					}
					let mut buf = vec![0_u8; len];
					if con.read_exact(&mut buf[..]).is_err() {
						break;
					}
					match kind {
						HeaderKind::Msg => tx.send(Event::Msg(buf)),
						HeaderKind::File { name, compressed } => tx.send(Event::File {
							data: buf,
							name,
							compressed,
						}),
					}
					.unwrap_or_else(|_| {
						eprintln!("peer terminated the connection... exiting");
						std::process::exit(0);
					});
				}
			}
		}
		eprintln!("peer terminated the connection... exiting");
		std::process::exit(0);
	});
}
