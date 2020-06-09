pub fn get_idle_time() -> u32 {
	use std::mem::size_of;
	use winapi::um::{
		sysinfoapi,
		winuser,
	};

	let mut last_input = winuser::LASTINPUTINFO {
		cbSize: size_of::<winuser::LASTINPUTINFO>() as u32,
		dwTime: 0,
	};

	assert!( unsafe { winuser::GetLastInputInfo( &mut last_input ) } != 0 );

	let tick_count: u32 = unsafe { sysinfoapi::GetTickCount() };

	return tick_count - last_input.dwTime;
}
