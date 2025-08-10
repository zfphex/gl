use glow::*;
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

    // gl::load_with(|name| {
    //     let name = std::ffi::CString::new(name).unwrap();
    //     let p = unsafe { wglGetProcAddress(name.as_ptr()) };
    //     if !p.is_null() {
    //         p
    //     } else {
    //         unsafe { (GetProcAddress(opengl32, name.as_ptr())) as *const c_void }
    //     }
    // });
}

fn load_glow() -> Context {
    let opengl32 = unsafe { LoadLibraryA(b"opengl32.dll\0".as_ptr()) };
    assert!(!opengl32.is_null());
    unsafe {
        Context::from_loader_function_cstr(|name| {
            // let name = std::ffi::CString::new(name).unwrap();
            let p = wglGetProcAddress(name.as_ptr());
            if !p.is_null() {
                p
            } else {
                (GetProcAddress(opengl32, name.as_ptr())) as *const c_void
            }
        })
    }
}

fn main() {
    unsafe {
        defer_results!();
        profile!();

        let window = create_window("gl2", 0, 0, 800, 600, WindowStyle::DEFAULT);
        let gl = load_glow();
        let version = gl.version();
        println!("OpenGL version: {}.{}", version.major, version.minor);

        // unsafe {
        //     gl::ClearColor(0.1, 0.2, 0.3, 1.0);
        // }

        let texture = gl.create_named_texture(glow::TEXTURE_2D).unwrap();
        gl.texture_storage_2d(texture, 1, RGBA8, 800, 600);
        gl.texture_parameter_i32(texture, TEXTURE_MIN_FILTER, LINEAR as i32);
        gl.texture_parameter_i32(texture, TEXTURE_MAG_FILTER, LINEAR as i32);

        // Bindless texture handle
        // Glow does not support bindless textures... :(
        // tex_handle = gl.get_texture_handle_arb(output_tex);
        // gl.make_texture_handle_resident_arb(tex_handle);

        loop {
            match window.event() {
                Some(Event::Quit | Event::Input(window::Key::Escape, _)) => break,
                Some(Event::Input(key, modifiers)) => println!("{:?} {:?}", key, modifiers),
                _ => {}
            }
            // unsafe { gl.clear(glow::COLOR_BUFFER_BIT) };
            // unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
            window.swap_buffers();
            // return;
        }
    }
}
