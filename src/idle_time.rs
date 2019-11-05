pub fn get_idle_time() -> u32 {
	use std::mem::size_of;
	use winapi::um::{
		sysinfoapi::GetTickCount,
		winuser::{
			GetLastInputInfo,
			LASTINPUTINFO,
		},
	};

	let mut last_input = LASTINPUTINFO {
		cbSize: size_of::<LASTINPUTINFO>() as u32,
		dwTime: 0,
	};

	assert!( unsafe { GetLastInputInfo( &mut last_input ) } != 0 );

	let tick_count: u32 = unsafe { GetTickCount() };

	return tick_count - last_input.dwTime;
}
