use std::ptr::null_mut;
use winapi::{
	shared::{
		windef,
		minwindef,
	},
	um::{
		handleapi,
		processthreadsapi,
		psapi,
		winnt,
		winuser,
	},
};

pub struct WindowInfo {
	pub name: String,
	pub path: String,
	pub id: minwindef::DWORD,
}

pub fn get_foreground_window() -> windef::HWND {
	return unsafe { winuser::GetForegroundWindow() };
}

pub fn get_window_title( window: windef::HWND ) -> String {
	use std::ffi::OsString;
	use std::os::windows::ffi::OsStringExt;

	let length: i32 = unsafe { winuser::GetWindowTextLengthW( window ) };
	let mut title = vec![ 0u16; ( length + 1 ) as usize ];
	unsafe {
		winuser::GetWindowTextW( window, title.as_mut_ptr(), length + 1 );
	}
	title.truncate( length as usize );
	return OsString::from_wide( &title ).to_string_lossy().into_owned();
}

pub fn get_active_window_info() -> WindowInfo {
	let active_window = get_foreground_window();

	let mut path = Vec::with_capacity( minwindef::MAX_PATH );
	let mut process_id: minwindef::DWORD;

	unsafe {
		process_id = std::mem::MaybeUninit::uninit().assume_init();
		winuser::GetWindowThreadProcessId( active_window, &mut process_id );

		let handle = processthreadsapi::OpenProcess(
			winnt::PROCESS_QUERY_INFORMATION | winnt::PROCESS_VM_READ,
			0,
			process_id,
		);

		let length = psapi::GetModuleFileNameExW(
			handle,
			null_mut(),
			path.as_mut_ptr(),
			minwindef::MAX_PATH as u32
		);

		handleapi::CloseHandle( handle );

		path.set_len( length as usize );
	}

	return WindowInfo {
		id: process_id,
		path: String::from_utf16( &path ).unwrap(),
		name: get_window_title( active_window ),
	};
}
