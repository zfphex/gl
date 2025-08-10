#![allow(unused)]
// #![windows_subsystem = "windows"]
use mini::{defer_results, profile};
use window::*;

use core::ffi::c_void;

#[link(name = "Kernel32")]
unsafe extern "system" {
    fn LoadLibraryA(name: *const u8) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const i8) -> *mut c_void;
}

fn load_opengl() {
    let opengl32 = unsafe { LoadLibraryA(b"opengl32.dll\0".as_ptr()) };
    assert!(!opengl32.is_null());

    gl::load_with(|name| {
        let name = std::ffi::CString::new(name).unwrap();
        let p = unsafe { wglGetProcAddress(name.as_ptr()) };
        if !p.is_null() {
            p
        } else {
            unsafe { (GetProcAddress(opengl32, name.as_ptr())) as *const c_void }
        }
    });
}

fn main() {
    unsafe {
        let window = create_window("gl2", 0, 0, 800, 600, WindowStyle::DEFAULT);
        load_opengl();

        gl::ClearColor(0.1, 0.2, 0.3, 1.0);

        loop {
            match window.event() {
                Some(Event::Quit | Event::Input(window::Key::Escape, _)) => break,
                Some(Event::Input(key, modifiers)) => println!("{:?} {:?}", key, modifiers),
                _ => {}
            }

            unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
            window.swap_buffers();
        }
    }
}
