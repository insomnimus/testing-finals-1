mod app;
mod client;
mod encryption;
mod error;
mod proto;
mod session;

use std::net::{
	TcpListener,
	TcpStream,
	ToSocketAddrs,
};

use clap::{
	arg,
	App,
	Arg,
};
use encryption::{
	Cipher,
	KeyedXor,
};
pub use error::{
	Error,
	Result,
};

// static _TEST_MODE: AtomicBool = AtomicBool::new(false);

fn build_app() -> App<'static> {
	App::new("simple-chat")
		.about("Yazılım Sınama final projesi.")
		// .arg(Arg::new("test-mode").hidden(true).long("__test_mode"))
		.arg(arg!(-k --key <PASSWORD> "Mesajlaşmada kullanılacak şifre."))
		.subcommand(
			App::new("listen")
				.about("Karşı tarafın bağlanmasını bekle.")
				.arg(
					Arg::new("address")
						.required(true)
						.help("Dinlenecek ip adresi. Örneğin localhost:8000 ya da 127.0.0.1:1234")
						.validator(|s| {
							s.to_socket_addrs()
								.map(|_| {})
								.map_err(|e| format!("girilen adres geçerli değil: {}", e))
						}),
				),
		)
		.subcommand(
			App::new("connect")
				.about("Karşı tarafa bağlan.")
				.visible_alias("dial")
				.arg(
					Arg::new("address")
						.required(true)
						.help("Bağlanılacak adres. Örneğin localhost:8000 ya da 192.168.0.245")
						.validator(|s| {
							s.to_socket_addrs()
								.map(|_| {})
								.map_err(|e| format!("girilen adres geçerli değil: {}", e))
						}),
				),
		)
}

fn run_connect<A: ToSocketAddrs, K: Cipher>(addr: A, key: K) -> Result<()> {
	#[cfg(not(blackbox_tests))]
	println!("connecting to the remote");
	let stream = TcpStream::connect(addr)?;
	#[cfg(not(blackbox_tests))]
	println!("connected");
	app::start(stream, key);
	Ok(())
}

fn run_listen<A: ToSocketAddrs + std::fmt::Debug, K: Cipher>(addr: A, key: K) -> Result<()> {
	#[cfg(not(blackbox_tests))]
	println!("listening on {:?}", &addr);
	let listener = TcpListener::bind(addr)?;
	let con = listener.incoming().next().unwrap()?;
	#[cfg(not(blackbox_tests))]
	println!("remote connected");
	app::start(con, key);
	Ok(())
}

fn main() {
	let m = build_app().get_matches();
	// _TEST_MODE.store(m.is_present("test-mode"), Ordering::Relaxed);

	let key = m.value_of("key").map(KeyedXor::new).unwrap();
	match m.subcommand().unwrap() {
		("listen", m) => {
			let addr = m.value_of("address").unwrap();
			run_listen(addr, key)
		}
		("connect", m) => {
			let addr = m.value_of("address").unwrap();
			run_connect(addr, key)
		}
		_ => panic!("unreachable code reached"),
	}
	.unwrap_or_else(|e| {
		eprintln!("error: {}", e);
		std::process::exit(2);
	});
}
