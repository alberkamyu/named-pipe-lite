/*
    Lightweight Windows named pipe bindings for Rust
    (c) alberkamyu, 2026

    src/wintypes.rs
*/

#![allow(non_camel_case_types, dead_code)]

use std::os::raw::{c_void, c_char, c_ushort, c_ulong, c_int, c_short, c_uint};

pub(crate) type BOOL = c_int;
pub(crate) type ULONG = c_ulong;
pub(crate) type ULONG_PTR = usize;
pub(crate) type SHORT = c_short;
pub(crate) type WORD = c_ushort;
pub(crate) type DWORD = ULONG;
pub(crate) type DWORD_PTR = ULONG_PTR;
pub(crate) type LPDWORD = *mut DWORD;
pub(crate) type VOID = c_void;
pub(crate) type PVOID = *mut VOID;
pub(crate) type LPVOID = PVOID;
pub(crate) type LPCVOID = *const VOID;
pub(crate) type UINT = c_uint;
pub(crate) type UINT_PTR = usize;
pub(crate) type LPCCH = *const c_char;
pub(crate) type LPWSTR = *mut u16;
pub(crate) type LPCWSTR = *const u16;
pub(crate) type WCHAR = u16;
pub(crate) type SIZE_T = ULONG_PTR;
pub(crate) type LARGE_INTEGER = i64;
pub(crate) type errno_t = c_int;
pub(crate) type HANDLE = PVOID;
pub(crate) type HWND = *mut c_void;
pub(crate) type SOCKET = UINT_PTR;
pub(crate) const INVALID_HANDLE_VALUE: HANDLE = usize::MAX as HANDLE;
pub(crate) type LPOVERLAPPED = *mut c_void;
pub(crate) type LPSECURITY_ATTRIBUTES = *mut c_void;