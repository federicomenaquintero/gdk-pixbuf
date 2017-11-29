use libc::{c_int as gint};
use glib;
use utils::{Error};

pub trait Loader {
    fn begin_load(ctx: &LoadContext) -> Result<Self, Error>;

    fn load_increment(&self, bytes: &[u8]) -> Result<(), Error>;

    fn stop_load(&self) -> Result<(), Error>;
}

pub enum SizeRequest {
    Requested(u32, u32), // width, height
    WillIgnore // exit early; the caller just needed the size but doesn't need the image
}

pub trait Notify {
    fn notify_and_request_size(width: u32, height: u32) -> SizeRequest;

    fn notify_prepared(pixbuf: Option<&GdkPixbuf>,
                       animation: Option<&GdkPixbufAnimation>);

    fn notify_area_updated(pixbuf: &GdkPixbuf,
                           x: u32,
                           y: u32,
                           width: u32,
                           height: u32);
}

