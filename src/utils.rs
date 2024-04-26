use std::ptr::null_mut;

use winapi::{
    shared::minwindef::LPARAM,
    um::{
        libloaderapi::GetModuleHandleA,
        winuser::{GetForegroundWindow, LoadIconA, SendMessageA, ICON_BIG, ICON_SMALL, WM_SETICON},
    },
};

pub fn load_icon() {
    unsafe {
        let hwnd = GetForegroundWindow();
        let icon_handle = LoadIconA(
            GetModuleHandleA(null_mut()),
            "app-icon.ico\0".as_ptr() as *const i8,
        );
        SendMessageA(hwnd, WM_SETICON, ICON_SMALL as usize, icon_handle as LPARAM);
        SendMessageA(hwnd, WM_SETICON, ICON_BIG as usize, icon_handle as LPARAM);
    }
}
