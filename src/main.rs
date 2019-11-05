mod idle_time;
mod active_window;

struct Info {
	idle: u32,
	window: active_window::WindowInfo,
}

fn main() {
	use std::thread;
	use std::time::Duration;
	use std::sync::mpsc::{
		channel,
		Receiver,
		Sender,
	};

	let ( tx, rx ): ( Sender<Info>, Receiver<Info> ) = channel();

	thread::spawn( move || {
		loop {
			tx.send( Info{
				idle: idle_time::get_idle_time(),
				window: active_window::get_active_window_info(),
			} ).unwrap();
			thread::sleep( Duration::from_millis( 1000 ) );
		}
	} );

	loop {
		if let _info = rx.recv() {
			let info = _info.unwrap();
			println!("Last input was {:#?} milliseconds ago", info.idle);
			println!( "Active window:\n\tTitle: {}\n\tPath: {}", info.window.name, info.window.path );
		}
		thread::sleep( Duration::from_millis( 10 ) );
	}

}
