use super::phantom_data::phantom_data;
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
pub struct Display(ptr::NonNull<xlib::Display>);

impl Display {
    #[inline]
    pub unsafe fn from_raw(display: *mut xlib::Display) -> Option<Self> {
        ptr::NonNull::new(display).map(Display)
    }

    #[inline]
    pub unsafe fn as_raw_ref(&self) -> &xlib::Display {
        self.0.as_ref()
    }

    #[inline]
    pub unsafe fn as_raw_mut(&mut self) -> &mut xlib::Display {
        self.0.as_mut()
    }
}

#[repr(transparent)]
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

pub fn default_screen(display: &mut Display) -> c_int {
    unsafe { xlib::XDefaultScreen(display.as_raw_mut()) }
}

pub fn root_window(display: &mut Display, screen: c_int) -> xlib::Window {
    unsafe { xlib::XRootWindow(display.as_raw_mut(), screen) }
}

pub fn intern_atom<T: AsRef<ffi::CStr> + ?Sized>(
    display: &mut xlib::Display,
    atom_name: &T,
    create_if_exists: bool,
) -> Option<xlib::Atom> {
    unsafe {
        match xlib::XInternAtom(
            display,
            atom_name.as_ref().as_ptr(),
            create_if_exists.into_xlib(),
        ) {
            0 => None,
            x => Some(x),
        }
    }
}
