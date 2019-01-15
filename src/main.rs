#![feature(proc_macro_hygiene)]

use self::force_mut::*;
use static_cstr::static_cstr;
use std::mem;
use std::os::raw::*;
use std::ptr;
use x11::xlib as xlib_sys;

mod force_mut;
mod phantom_data;
// NOTE(mickvangelderen): Declared as pub to get rid of unused fn.
pub mod xlib;

fn main() {
    let start = std::time::Instant::now();
    unsafe {
        chaos();
    }
    println!("{:?}", start.elapsed());
}

unsafe fn chaos() {
    let mut display = xlib::open_display_default().unwrap();

    let screen_number = xlib::default_screen(&display);

    let root = xlib::root_window(&display, &screen_number);

    let mut attributes: xlib_sys::XSetWindowAttributes = mem::uninitialized();
    attributes.background_pixel = xlib::white_pixel(&display, &screen_number);

    let window = xlib::create_window(
        &display,
        root,
        0,
        0,
        400,
        300,
        0,
        0,
        xlib_sys::InputOutput as c_uint,
        ptr::null_mut(),
        xlib_sys::CWBackPixel,
        &mut attributes,
    ).unwrap();

    xlib_sys::XStoreName(
        display.as_ref().force_mut(),
        window.as_raw(),
        static_cstr!("X11 Lab").as_ptr() as *mut c_char,
    );

    let wm_protocols =
        xlib::intern_atom(&display, static_cstr!("WM_PROTOCOLS"), true).unwrap();
    let wm_delete_window =
        xlib::intern_atom(&display, static_cstr!("WM_DELETE_WINDOW"), true).unwrap();

    let mut protocols = [wm_delete_window];
    xlib_sys::XSetWMProtocols(
        display.as_ref().force_mut(),
        window.as_raw(),
        protocols.as_mut_ptr(),
        protocols.len() as c_int,
    );

    xlib_sys::XMapWindow(display.as_ref().force_mut(), window.as_raw());

    // Main loop.
    let mut event: xlib_sys::XEvent = mem::uninitialized();

    'main: loop {
        xlib_sys::XNextEvent(display.as_mut(), &mut event);

        match event.get_type() {
            xlib_sys::ClientMessage => {
                let xclient = xlib_sys::XClientMessageEvent::from(event);

                if xclient.message_type == wm_protocols && xclient.format == 32 {
                    let protocol = xclient.data.get_long(0) as xlib_sys::Atom;

                    if protocol == wm_delete_window {
                        break 'main;
                    }
                }
            }
            _ => {}
        }
    }
}
