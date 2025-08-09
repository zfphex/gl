// #![windows_subsystem = "windows"]
use core::ffi::c_void;
use glow::HasContext;
use window::*;

#[link(name = "Kernel32")]
unsafe extern "system" {
    fn LoadLibraryA(name: *const u8) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const i8) -> *mut c_void;
}

fn main() {
    let window = create_window("gl2", 0, 0, 800, 600, WindowStyle::DEFAULT);

    // Load OpenGL function pointers using wglGetProcAddress with opengl32 fallback
    let opengl32 = unsafe { LoadLibraryA(b"opengl32.dll\0".as_ptr()) };
    assert!(!opengl32.is_null());

    let gl = unsafe {
        glow::Context::from_loader_function(|symbol_name| {
            let p = window.get_wgl_proc_address(symbol_name);
            if !p.is_null() {
                p
            } else {
                (GetProcAddress(opengl32, symbol_name.as_ptr() as *const i8)) as *const c_void
            }
        })
    };
    unsafe { gl.clear_color(0.1, 0.2, 0.3, 1.0) };

    loop {
        match window.event() {
            Some(Event::Quit | Event::Input(window::Key::Escape, _)) => break,
            Some(Event::Input(key, modifiers)) => println!("{:?} {:?}", key, modifiers),
            _ => {}
        }
        unsafe { gl.clear(glow::COLOR_BUFFER_BIT) };
        window.swap_buffers();
    }
}
