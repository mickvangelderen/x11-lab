use super::phantom_data::*;
use super::force_mut::*;
use std::ffi;
use std::marker;
use std::num;
use std::os::raw::*;
use std::ptr;
use x11::xlib;

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

#[repr(transparent)]
#[derive(Debug)]
pub struct Display(ptr::NonNull<xlib::Display>);

impl Display {
    #[inline]
    pub unsafe fn from_raw(display: *mut xlib::Display) -> Option<Self> {
        ptr::NonNull::new(display).map(Display)
    }

    #[inline]
    pub fn as_ref(&self) -> &xlib::Display {
        // Safe because the lifetime of the reference is bound to the lifetime of self.
        unsafe {
            self.0.as_ref()
        }
    }

    #[inline]
    pub fn as_mut(&mut self) -> &mut xlib::Display {
        // Safe because the lifetime of the reference is bound to the lifetime of self.
        unsafe {
            self.0.as_mut()
        }
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct ScreenNumber<'d> {
    screen_number: c_int,
    _display: marker::PhantomData<&'d Display>,
}

impl<'d> ScreenNumber<'d> {
    #[inline]
    pub unsafe fn from_raw(display: &'d Display, screen_number: c_int) -> Self {
        ScreenNumber {
            screen_number,
            _display: phantom_data(display),
        }
    }

    #[inline]
    pub fn as_raw(&self) -> c_int {
        self.screen_number
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Window<'d> {
    window: num::NonZeroU64,
    _display: marker::PhantomData<&'d Display>,
}

impl<'d> Window<'d> {
    #[inline]
    pub unsafe fn from_raw(display: &'d Display, window: xlib::Window) -> Option<Self> {
        num::NonZeroU64::new(window).map(|window| Window {
            window,
            _display: phantom_data(display),
        })
    }

    #[inline]
    pub fn as_raw(&self) -> xlib::Window {
        self.window.get()
    }
}

pub fn open_display_default() -> Option<Display> {
    unsafe { Display::from_raw(xlib::XOpenDisplay(ptr::null())) }
}

pub fn open_display<T: AsRef<ffi::CStr>>(display_name: &T) -> Option<Display> {
    unsafe { Display::from_raw(xlib::XOpenDisplay(display_name.as_ref().as_ptr())) }
}

pub fn default_screen(display: &Display) -> ScreenNumber {
    unsafe {
        ScreenNumber::from_raw(
            display,
            xlib::XDefaultScreen(display.as_ref().force_mut())
        )
    }
}

pub fn root_window(display: &Display, screen_number: &ScreenNumber) -> xlib::Window {
    unsafe { xlib::XRootWindow(display.as_ref().force_mut(), screen_number.as_raw()) }
}

pub fn white_pixel(display: &Display, screen_number: &ScreenNumber) -> c_ulong {
    unsafe { xlib::XWhitePixel(display.as_ref().force_mut(), screen_number.as_raw()) }
}

pub fn create_window<'d>(display: &'d Display, parent: xlib::Window, x: c_int, y: c_int, width: c_uint, height: c_uint, border_width: c_uint, depth: c_int, class: c_uint, visual: *mut xlib::Visual, valuemask: c_ulong, attributes: *mut xlib::XSetWindowAttributes) -> Option<Window<'d>> {
    unsafe {
        let raw = xlib::XCreateWindow(display.as_ref().force_mut(), parent, x, y, width, height, border_width, depth, class, visual, valuemask, attributes);
        Window::from_raw(display, raw)
    }
}

pub fn intern_atom<T: AsRef<ffi::CStr> + ?Sized>(
    display: &Display,
    atom_name: &T,
    create_if_exists: bool,
) -> Option<xlib::Atom> {
    unsafe {
        match xlib::XInternAtom(
            display.as_ref().force_mut(),
            atom_name.as_ref().as_ptr(),
            create_if_exists.into_xlib(),
        ) {
            0 => None,
            x => Some(x),
        }
    }
}
