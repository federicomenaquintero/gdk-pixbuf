use libc::{c_void, c_int};
use loader::{Notify, SizeRequest}

// This is a newtype on a void pointer.  We just use it to store the
// C-side pointer to the loader struct.
struct LoadContext(*const c_void);

extern "C" {
    fn load_context_notify_size(load_context: *const c_void,
                                width:  *mut c_int,
                                height: *mut c_int);

    fn load_context_notify_prepared(load_context: *const c_void,
                                    pixbuf: Option<&GdkPixbuf>,
                                    animation: Option<&GdkPixbufAnimation>);

    fn load_context_notify_updated(load_context: *const c_void,
                                   pixbuf: &GdkPixbuf,
                                   x:      c_int,
                                   y:      c_int,
                                   width:  c_int,
                                   height: c_int);
}

impl Notify for LoadContext {
    fn notify_and_request_size(&self, width: u32, height: u32) -> SizeRequest {
        let mut w = width;
        let mut h = height;

        unsafe { load_context_notify_size(self.0, &mut w as *mut _, &mut h as *mut *); }

        if w == 0 || h == 0 {
            SizeRequest::WillIgnore
        } else {
            SizeRequest::Requested(w, h)
        }
    }

    fn notify_prepared(pixbuf: Option<&GdkPixbuf>,
                       animation: Option<&GdkPixbufAnimation>) {
        unsafe { load_context_notify_prepared (self.0, pixbuf, animation); }
    }

    fn notify_area_updated(pixbuf: &GdkPixbuf,
                           x: u32,
                           y: u32,
                           width: u32,
                           height: u32) {
        assert!(x <= i32::MAX as u32);
        assert!(y <= i32::MAX as u32);
        assert!(width <= i32::MAX as u32);
        assert!(height <= i32::MAX as u32);

        unsafe { load_context_notify_updated (self.0,
                                              x as c_int,
                                              y as c_int,
                                              width as c_int,
                                              height as c_int); }
    }
}

#[no_mangle]
pub extern fn rust_loader_new(load_context: *const c_void) -> FIXME {
    let ctx = LoadContext(load_context);

    
}
