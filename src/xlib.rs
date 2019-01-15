use super::copy_ref::*;
use super::force_mut::*;
use super::phantom_data::*;
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
        unsafe { self.0.as_ref() }
    }

    #[inline]
    pub fn as_mut(&mut self) -> &mut xlib::Display {
        // Safe because the lifetime of the reference is bound to the lifetime of self.
        unsafe { self.0.as_mut() }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            match xlib::XCloseDisplay(self.as_mut()) {
                0 => {}
                x => {
                    panic!("Failed to close display: {}", x);
                }
            }
        }
    }
}

unsafe impl super::copy_ref::Elegible for Display {
    type Raw = ptr::NonNull<xlib::Display>;

    fn as_raw(&self) -> &Self::Raw {
        &self.0
    }
}

#[derive(Debug)]
pub struct ScreenNumber<'d> {
    screen_number: c_int,
    display: CopyRef<'d, Display>,
}

impl<'d> ScreenNumber<'d> {
    #[inline]
    pub unsafe fn from_raw(display: &'d Display, screen_number: c_int) -> Self {
        ScreenNumber {
            screen_number,
            display: CopyRef::new(display),
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

pub fn open<F: FnOnce(&Display)>(f: F) {
    let display = open_display_default().unwrap();

    let display = &*ClosureDrop::new(display, |display| {
        close_display(display).unwrap();
    });

    f(display);
}

pub struct ClosureDrop<T, F: FnOnce(T)>(Option<(T, F)>);

impl<T, F: FnOnce(T)> ClosureDrop<T, F> {
    #[inline]
    fn new(t: T, f: F) -> Self {
        ClosureDrop(Some((t, f)))
    }
}

impl<T, F: FnOnce(T)> std::ops::Deref for ClosureDrop<T, F> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.0.as_ref().take().unwrap().0
    }
}

impl<T, F: FnOnce(T)> Drop for ClosureDrop<T, F> {
    #[inline]
    fn drop(&mut self) {
        let (t, f) = self.0.take().unwrap();
        f(t)
    }
}

pub fn open_display_default() -> Option<Display> {
    unsafe { Display::from_raw(xlib::XOpenDisplay(ptr::null())) }
}

pub fn open_display<T: AsRef<ffi::CStr>>(display_name: &T) -> Option<Display> {
    unsafe { Display::from_raw(xlib::XOpenDisplay(display_name.as_ref().as_ptr())) }
}

pub fn close_display(mut display: Display) -> Result<(), num::NonZeroU32> {
    unsafe {
        num::NonZeroU32::new(xlib::XCloseDisplay(display.as_mut()) as u32).map_or(Ok(()), Err)
    }
}

pub fn default_screen(display: &Display) -> ScreenNumber {
    unsafe { ScreenNumber::from_raw(display, xlib::XDefaultScreen(display.as_ref().force_mut())) }
}

pub fn root_window(display: &Display, screen_number: &ScreenNumber) -> xlib::Window {
    unsafe { xlib::XRootWindow(display.as_ref().force_mut(), screen_number.as_raw()) }
}

pub fn white_pixel(display: &Display, screen_number: &ScreenNumber) -> c_ulong {
    unsafe { xlib::XWhitePixel(display.as_ref().force_mut(), screen_number.as_raw()) }
}

pub fn create_window<'d>(
    display: &'d Display,
    parent: xlib::Window,
    x: c_int,
    y: c_int,
    width: c_uint,
    height: c_uint,
    border_width: c_uint,
    depth: c_int,
    class: c_uint,
    visual: *mut xlib::Visual,
    valuemask: c_ulong,
    attributes: *mut xlib::XSetWindowAttributes,
) -> Option<Window<'d>> {
    unsafe {
        let raw = xlib::XCreateWindow(
            display.as_ref().force_mut(),
            parent,
            x,
            y,
            width,
            height,
            border_width,
            depth,
            class,
            visual,
            valuemask,
            attributes,
        );
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

// xlib_sys::XStoreName(
//     display.as_ref().force_mut(),
//     window.as_raw(),
//     static_cstr!("X11 Lab").as_ptr() as *mut c_char,
// );
