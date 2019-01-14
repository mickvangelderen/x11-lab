#![feature(proc_macro_hygiene)]

use static_cstr::static_cstr;
use std::ffi;
use std::mem;
use std::os::raw::*;
use std::ptr;
use x11::xlib;

fn main() {
    let start = std::time::Instant::now();
    unsafe {
        chaos();
    }
    println!("{:?}", start.elapsed());
}

trait IntoBool {
    fn into_xlib(self) -> xlib::Bool;
}

impl IntoBool for bool {
    #[inline]
    fn into_xlib(self) -> xlib::Bool {
        match self {
            false => xlib::False,
            true => xlib::True,
        }
    }
}

// TODO: For how long is the pointer valid?
// TODO: We can alias the pointer by opening the display twice :'(
unsafe fn open_display<T: AsRef<ffi::CStr> + ?Sized>(
    display_name: Option<&T>,
) -> Option<&mut xlib::Display> {
    let ptr = xlib::XOpenDisplay(match display_name {
        Some(x) => x.as_ref().as_ptr(),
        None => ptr::null(),
    });
    if ptr.is_null() {
        None
    } else {
        Some(&mut *ptr)
    }
}

unsafe fn default_screen(display: &mut xlib::Display) -> c_int {
    xlib::XDefaultScreen(display)
}

unsafe fn root_window(display: &mut xlib::Display, screen: c_int) -> xlib::Window {
    xlib::XRootWindow(display, screen)
}

unsafe fn intern_atom<T: AsRef<ffi::CStr> + ?Sized>(
    display: &mut xlib::Display,
    atom_name: &T,
    create_if_exists: bool,
) -> Option<xlib::Atom> {
    match xlib::XInternAtom(display, atom_name.as_ref().as_ptr(), create_if_exists.into_xlib()) {
        0 => None,
        x => Some(x),
    }
}

unsafe fn chaos() {
    let display = open_display::<ffi::CStr>(None).unwrap();

    let screen = default_screen(display);
    let root = root_window(display, screen);

    let mut attributes: xlib::XSetWindowAttributes = mem::uninitialized();
    attributes.background_pixel = xlib::XWhitePixel(display, screen);

    let window = xlib::XCreateWindow(
        display,
        root,
        0,
        0,
        400,
        300,
        0,
        0,
        xlib::InputOutput as c_uint,
        ptr::null_mut(),
        xlib::CWBackPixel,
        &mut attributes,
    );

    xlib::XStoreName(
        display,
        window,
        static_cstr!("X11 Lab").as_ptr() as *mut c_char,
    );

    let wm_protocols = intern_atom(display, static_cstr!("WM_PROTOCOLS"), true).unwrap();
    let wm_delete_window = intern_atom(display, static_cstr!("WM_DELETE_WINDOW"), true).unwrap();

    let mut protocols = [wm_delete_window];
    xlib::XSetWMProtocols(
        display,
        window,
        protocols.as_mut_ptr(),
        protocols.len() as c_int,
    );

    xlib::XMapWindow(display, window);

    // Main loop.
    let mut event: xlib::XEvent = mem::uninitialized();

    'main: loop {
        xlib::XNextEvent(display, &mut event);

        match event.get_type() {
            xlib::ClientMessage => {
                let xclient = xlib::XClientMessageEvent::from(event);

                if xclient.message_type == wm_protocols && xclient.format == 32 {
                    let protocol = xclient.data.get_long(0) as xlib::Atom;

                    if protocol == wm_delete_window {
                        break 'main;
                    }
                }
            }
            _ => {}
        }
    }
}
