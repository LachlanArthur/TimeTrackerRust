use std::ptr::null_mut;
use winapi::{
	shared::{
		windef::HWND,
		minwindef::{
			DWORD,
			MAX_PATH,
		},
	},
	um::{
		handleapi::CloseHandle,
		processthreadsapi::OpenProcess,
		psapi::GetModuleFileNameExW,
		winnt::{
			PROCESS_QUERY_INFORMATION,
			PROCESS_VM_READ,
		},
		winuser::{
			GetForegroundWindow,
			GetWindowTextLengthW,
			GetWindowTextW,
			GetWindowThreadProcessId
		},
	},
};

pub struct WindowInfo {
	pub name: String,
	pub path: String,
	pub id: winapi::shared::minwindef::DWORD,
}

pub fn get_foreground_window() -> HWND {
	return unsafe { GetForegroundWindow() };
}

pub fn get_window_title( window: HWND ) -> String {
	use std::ffi::OsString;
	use std::os::windows::ffi::OsStringExt;

	let length: i32 = unsafe { GetWindowTextLengthW( window ) };
	let mut title = vec![ 0u16; ( length + 1 ) as usize ];
	unsafe {
		GetWindowTextW( window, title.as_mut_ptr(), length + 1 );
	}
	title.truncate( length as usize );
	OsString::from_wide( &title ).to_string_lossy().into_owned()
}

pub fn get_active_window_info() -> WindowInfo {
	let active_window = get_foreground_window();

	let mut path = Vec::with_capacity( MAX_PATH );
	let mut process_id: DWORD;

	unsafe {
		process_id = std::mem::uninitialized();
		GetWindowThreadProcessId( active_window, &mut process_id );

		let handle = OpenProcess(
			PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
			0,
			process_id,
		);

		let length = GetModuleFileNameExW(
			handle,
			null_mut(),
			path.as_mut_ptr(),
			MAX_PATH as u32
		);

		CloseHandle( handle );

		path.set_len( length as usize );
	}

	return WindowInfo {
		id: process_id,
		path: String::from_utf16( &path ).unwrap(),
		name: get_window_title( active_window ),
	};
}
