pub use core::ffi::c_void;

pub type size_t = usize;
pub type ssize_t = isize;

#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
pub type c_char = u8;
#[cfg(target_arch = "x86_64")]
pub type c_char = i8;

pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;

#[cfg(target_pointer_width = "32")]
pub type c_long = i32;
#[cfg(target_pointer_width = "64")]
pub type c_long = i64;

#[cfg(target_pointer_width = "32")]
pub type c_ulong = u32;
#[cfg(target_pointer_width = "64")]
pub type c_ulong = u64;

pub type c_longlong = i64;
pub type c_ulonglong = u64;

pub type c_uint8_t = u8;
pub type c_uint16_t = u16;
pub type c_uint32_t = u32;
pub type c_uint64_t = u64;

pub type c_int8_t = i8;
pub type c_int16_t = i16;
pub type c_int32_t = i32;
pub type c_int64_t = i64;

pub const STDOUT_FILENO: ::c_int = 1;
pub const STDERR_FILENO: ::c_int = 2;

extern "C" {
    pub fn calloc(nobj: size_t, size: size_t) -> *mut c_void;
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn realloc(p: *mut c_void, size: size_t) -> *mut c_void;
    pub fn free(p: *mut c_void);
    pub fn memalign(align: ::size_t, size: ::size_t) -> *mut ::c_void;
    pub fn posix_memalign(memptr: *mut *mut ::c_void, align: ::size_t, size: ::size_t) -> ::c_int;
    pub fn write(fd: ::c_int, buf: *const ::c_void, count: ::size_t) -> ::ssize_t;
    pub fn writev(fd: ::c_int, iov: *const ::iovec, iovcnt: ::c_int) -> ::ssize_t;
}

s! {
    pub struct iovec {
        pub iov_base: *mut ::c_void,
        pub iov_len: ::size_t,
    }
}
