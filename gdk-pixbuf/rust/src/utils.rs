use std::{slice, ptr, i32};
use libc::{c_int, c_void, size_t};

use glib_sys::{gboolean};
use glib::translate::*;

// This mirrors GdkPixbufError
pub enum ErrorKind {
    /* image data hosed */
    CorruptImage,
    /* no mem to load image */
    InsufficientMemory,
    /* bad option passed to save routine */
    BadOption,
    /* unsupported image type (sort of an ENOSYS) */
    UnknownType,
    /* unsupported operation (load, save) for image type */
    UnsupportedOperation,
    Failed,
    IncompleteAnimation,
}

pub struct Error {
    pub kind: ErrorKind,
    pub msg: String
}

#[repr(C)]
pub enum GdkColorspace {
    RGB
}

extern "C" {
    fn gdk_pixbuf_new(colorspace:      GdkColorspace,
                      has_alpha:       gboolean,
                      bits_per_sample: c_int,
                      width:           c_int,
                      height:          c_int) -> *mut c_void;

    fn gdk_pixbuf_get_width(pixbuf: *mut c_void) -> c_int;
    fn gdk_pixbuf_get_height(pixbuf: *mut c_void) -> c_int;
    fn gdk_pixbuf_get_rowstride(pixbuf: *mut c_void) -> c_int;
    fn gdk_pixbuf_get_byte_length(pixbuf: *mut c_void) -> size_t;

    fn gdk_pixbuf_get_pixels(pixbuf: *mut c_void) -> *mut u8;
}

// Just a newtype over a *GdkPixbuf
pub struct GdkPixbuf(*mut c_void);

impl GdkPixbuf {
    pub fn new(has_alpha: bool,
               width:     u32,
               height:    u32) -> Result<GdkPixbuf, Error> {
        assert!(width > 0);
        assert!(height > 0);
        assert!(width <= i32::MAX as u32);
        assert!(height <= i32::MAX as u32);

        let raw_pixbuf = unsafe { gdk_pixbuf_new (GdkColorspace::RGB,
                                                  has_alpha.to_glib(),
                                                  8,
                                                  width as c_int,
                                                  height as c_int) };

        if raw_pixbuf.is_null() {
            Err(Error {
                kind: ErrorKind::InsufficientMemory,
                msg: format!("Cannot allocate memory for image")  // FIXME: i18n
            })
        } else {
            let pixbuf = GdkPixbuf(raw_pixbuf);
            let pixels = unsafe { gdk_pixbuf_get_pixels(raw_pixbuf) };
            assert!(!pixels.is_null());

            let byte_length = pixbuf.get_byte_length();

            // that memory comes uninitialized out of g_try_malloc_n()
            unsafe { ptr::write_bytes(pixels, 0, byte_length); }

            Ok(pixbuf)
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        let w = unsafe { gdk_pixbuf_get_width(self.0) };
        let h = unsafe { gdk_pixbuf_get_height(self.0) };

        assert!(w >= 0);
        assert!(h >= 0);

        (w as u32, h as u32)
    }

    pub fn get_rowstride(&self) -> usize {
        let rowstride = unsafe { gdk_pixbuf_get_rowstride(self.0) };

        assert!(rowstride > 0);

        rowstride as usize
    }

    pub fn get_byte_length(&self) -> usize {
        let byte_length = unsafe { gdk_pixbuf_get_byte_length(self.0) };

        assert!(byte_length > 0);

        byte_length as usize
    }

    pub fn get_pixels_mut(&self) -> &mut [u8] {
        let pixels = unsafe { gdk_pixbuf_get_pixels(self.0) };
        let byte_length = self.get_byte_length();

        assert!(!pixels.is_null());

        unsafe { slice::from_raw_parts_mut(pixels, byte_length) }
    }

    pub fn get_row_mut(&self, usize row) -> &mut [u8] {
        let (_, height) = self.get_size();
        assert!(row < height);

        let pixels = unsafe { gdk_pixbuf_get_pixels(self.0) };
        let byte_length = self.get_byte_length();
        let rowstride = self.get_rowstride();

        assert!(!pixels.is_null());

        let row_start_ofs = row * rowstride;

        let row_start = pixels.offset(row_start_ofs);
        let row_len = byte_length - row_start_ofs;

        unsafe { slice::from_raw_parts_mut(row_start, row_len) }
    }
}
