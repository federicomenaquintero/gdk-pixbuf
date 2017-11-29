Rust loaders for gdk-pixbuf
===========================

Image loaders for gdk-pixbuf have been historically written in C.
We would like to move all the loaders to Rust, so that they can be
written in a fast, safe language.

# Goals

* Allow writing gdk-pixbuf modules in Rust.

* No unsafe code in module implementations.

* Support the progressive loading API only; deprecate the
  non-progressive API.

* Allow easy testing of an individual loader outside of gdk-pixbuf's
  infrastructure for modules, e.g. for fuzz testing individual loaders.

# Rough plan

## C entry points for the module

Gdk-pixbuf modules are shared libraries that export two functions,
`fill_vtable` and `fill_info`.

The first entry point, `fill_vtable`, is a function that takes a
`GdkPixbufModule` structure, and is supposed to fill in the
`->begin_load`, etc. fields.  Other fields are initialized by the
gdk-pixbuf library itself at the program's runtime.

The other entry point is `fill_info`.  This doesn't get called within
user programs.  Instead, it is called by `gdk-pixbuf-query-loaders` at
installation time.  This function takes an uninitialized
`GdkPixbufFormat` structure and fills in details like file extensions,
header magic, and names of MIME types.

Those two structs, `GdkPixbufModule` and `GdkPixbufFormat`, are
rather... C-ish.  To avoid a lot of clunky Rust code to manipulate
them, we will instead have some minimal C code that can initialize
those structures in an implementation of `fill_vtable` and
`fill_info`.  That C code will reference Rust code for the actual
method implementations, or to get the `GdkPixbufFormat`'s details.

Currently the entry points are in `gdk-pixbuf/rust-module.c`.  Each
image loader written in Rust will have this file as part of its
sources, since it is where the `fill_vtable` and `fill_info` entry
points are defined and made visible to the linker.

## Minimal Rust wrapper for the internal module API

### Module methods

A GdkPixbufModule, when it implements the progerssive loading API,
must provide implementations of the methods `begin_load`,
`load_increment`, and `stop_load`.  We provide a `trait Loader` in
`gdk-pixbuf/rust/src/loader.rs` with these methods.  There will be at
least two impls of this trait:  one to be called by the C-to-Rust
wrapper code in the module, and one to be used in the test harness.
This way we should be able to implement things like fuzz testing for
individual loaders, without having all the gdk-pixbuf module machinery
around.

### Notifications from the loader

A gdk-pixbuf module's API entry point is its `begin_load` method.
This function gets passed three callbacks, `size_func`,
`prepared_func`, and `updated_func`, which the module is supposed to
call at various points while loading an image.  To avoid `unsafe`
calls to these C callbacks, we will provide Rust wrappers for them.

Currently those wrappers are represented as a `trait Notify` in
`gdk-pixbuf/rust/src/loader.rs`.  We will probably end up with two
impls for that trait:  one for the actual C callbacks at runtime, and
one for the test harness.  The test harness will assert things like
"`size_func` was indeed called while reading an image file's header",
without having to use the whole gdk-pixbuf module machinery.

We also try to make some ad-hoc concepts in the C API more idiomatic
for Rust.  For example, if `size_func` returns 0 for either of the
width/height of an image, the loader is supposed to exit early and
avoid parsing the rest of the image (i.e. the caller was just
interested in fetching the image size, not the actual pixels).  We
represent this with `enum SizeRequest`, which can be either an
explicitly-requested size from the calling code, or `WillIgnore`.

Currently the Rust-to-C wrappers for notifications are in
`gdk-pixbuf/rust/src/pixbuf-module.rs`.

## Minimal Rust wrapper for GdkPixbuf objects

Here we don't have the `gdk-pixbuf` crate available from gtk-rs, since
we are in the low-level gdk-pixbuf implementation itself.  In loaders,
we still need to be able to do things like

* Create an empty pixbuf once we know the image size.

* Obtain access to the pixbuf's pixel buffer, preferably safely.

* Return errors that will be translated to GError.

The wrapper for these is in `gdk-pixbuf/rust/src/utils.rs`.  Here we
wrap basic functions like `gdk_pixbuf_new()` and
`gdk_pixbuf_get_pixels()`.

### Accessing pixels safely

An idiomatic, safe way to represent the pixel buffer of a GdkPixbuf is
to obtain a Rust `&mut [u8]` slice for the buffer.  We therefore have
these:

```rust
impl GdkPixbuf {
    pub fn get_pixels_mut(&self) -> &mut [u8] { ... }
    
    pub fn get_row_mut(&self, usize row) -> &mut [u8] { ... }
}
```

The first one obtains a mutable slice into the whole pixel buffer.
The second one obtains a mutable slice into a single row.  We will see
if it is possible to have iterators without superfluous bounds checks
for these constructs.

