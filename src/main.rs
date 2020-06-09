mod idle_time;
mod active_window;

use serde::Serialize;

extern crate chrono;
extern crate dirs;
extern crate signal_hook;

struct Info {
	idle: u32,
	window: active_window::WindowInfo,
}

#[derive(Serialize, Debug, Clone)]
enum InfoStatus {
	Idle( u32 ),
	Active { title: String, path: String },
}

#[derive(Serialize, Debug, Clone)]
struct InfoJson {
	date: chrono::NaiveDateTime,
	status: InfoStatus,
}

fn main() {
	use std::thread;
	use std::time::Duration;
	use std::sync::mpsc::{
		channel,
		Receiver,
		Sender,
	};
	use std::io::prelude::*;

	let polling_rate: u32 = 1000;
	let idle_timeout: u32 = 1000 * 60;
	let path = dirs::home_dir().expect( "Could not find home dir" ).join( "TimeTracker" );
	let filename = chrono::Local::now().naive_local().format( "%Y-%m-%d.ndjson" ).to_string();

	std::fs::create_dir_all( path.to_owned() )
		.expect( "Failed to create log path" );

	let file = std::fs::OpenOptions::new()
		.append(true)
		.create(true)
		.open( path.join( filename ) )
		.expect( "Failed to open the log file" );

	let writer = std::io::LineWriter::new( file );

	let ( tx, rx ): ( Sender<Info>, Receiver<Info> ) = channel();

	thread::spawn( move || {
		loop {
			tx.send( Info{
				idle: idle_time::get_idle_time(),
				window: active_window::get_active_window_info(),
			} ).unwrap();
			thread::sleep( Duration::from_millis( polling_rate.into() ) );
		}
	} );

	let mut last_info = InfoJson{
		date: chrono::Local::now().naive_local(),
		status: InfoStatus::Idle( 0 ),
	};

	// // TODO: Handle termination
	// unsafe { signal_hook::register( signal_hook::SIGTERM, || {
	// 	// Write last info
	// 	writeln!( writer.get_ref(), "{}", serde_json::to_string( &( last_info.clone() ) ).unwrap() )
	// 		.expect( "Failed to write to log file" );
	// } ) };

	loop {
		let info = rx.recv().unwrap();
		let is_idle = info.idle > idle_timeout;

		let info = InfoJson{
			date: chrono::Local::now().naive_local(),
			status: if is_idle {
				InfoStatus::Idle( info.idle - idle_timeout )
			} else {
				InfoStatus::Active{
					title: info.window.name,
					path: info.window.path,
				}
			},
		};

		let status_changed = match ( last_info.clone().status, info.clone().status ) {
			// Check both title and path
			( InfoStatus::Active { title: last_title, path: last_path }, InfoStatus::Active { title: active_title, path: active_path } ) => last_title != active_title || last_path != active_path,
			// Idle is always unchanged
			( InfoStatus::Idle( _ ), InfoStatus::Idle( _ ) ) => false,
			// Different types
			_ => true,
		};

		if status_changed {

			// Immediately output last idle info before becoming active
			match last_info.clone().status {
				InfoStatus::Idle( _ ) => {
					writeln!( writer.get_ref(), "{}", serde_json::to_string( &( last_info.clone() ) ).unwrap() )
						.expect( "Failed to write to log file" );
				},
				_ => {},
			}

			writeln!( writer.get_ref(), "{}", serde_json::to_string( &( info.clone() ) ).unwrap() )
				.expect( "Failed to write to log file" );
		}

		last_info = info.clone();

		thread::sleep( Duration::from_millis( polling_rate.into() ) );
	}

}
