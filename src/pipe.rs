/*
    Lightweight Windows named pipe bindings for Rust
    (c) alberkamyu, 2026

    src/pipe.rs
*/

use std::fmt;
use crate::wintypes::{DWORD, HANDLE, LPCWSTR, LPSECURITY_ATTRIBUTES, LPVOID, LPDWORD, BOOL, LPOVERLAPPED, LPCVOID};

#[link(name = "kernel32")]
unsafe extern "system" {
    pub fn CreateNamedPipeW(
        lpName: LPCWSTR,
        dwOpenMode: DWORD,
        dwPipeMode: DWORD,
        nMaxInstances: DWORD,
        nOutBufferSize: DWORD,
        nInBufferSize: DWORD,
        nDefaultTimeOut: DWORD,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES
    ) -> HANDLE;

    pub fn ConnectNamedPipe(
        hNamedPipe: HANDLE,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;

    pub fn DisconnectNamedPipe(
        hNamedPipe: HANDLE,
    ) -> BOOL;

    pub fn CloseHandle(
        hObject: HANDLE,
    ) -> BOOL;

    pub fn WriteFile(
        hFile: HANDLE,
        lpBuffer: LPCVOID,
        nNumberOfBytesToWrite: DWORD,
        lpNumberOfBytesWritten: LPDWORD,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;

    pub fn ReadFile(
        hFile: HANDLE,
        lpBuffer: LPVOID,
        nNumberOfBytesToRead: DWORD,
        lpNumberOfBytesRead: LPDWORD,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;

    pub fn GetLastError() -> DWORD;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipeType {
    Byte,    // 0x00000000
    Message, // 0x00000004
}

impl PipeType {
    pub fn as_dword(&self) -> DWORD {
        match self {
            PipeType::Byte => 0x00000000,
            PipeType::Message => 0x00000004,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipeReadMode {
    Byte,    // 0x00000000
    Message, // 0x00000002
}

impl PipeReadMode {
    pub fn as_dword(&self) -> DWORD {
        match self {
            PipeReadMode::Byte => 0x00000000,
            PipeReadMode::Message => 0x00000002,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipeWaitMode {
    Wait,    // 0x00000000
    NoWait,  // 0x00000001
}

impl PipeWaitMode {
    pub fn as_dword(&self) -> DWORD {
        match self {
            PipeWaitMode::Wait => 0x00000000,
            PipeWaitMode::NoWait => 0x00000001,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaxInstances {
    Unlimited, // 255
    Specific(u32),
}

impl MaxInstances {
    pub fn as_dword(&self) -> DWORD {
        match self {
            MaxInstances::Unlimited => 255,
            MaxInstances::Specific(n) => *n,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipeAccess {
    Inbound,  // 0x00000001
    Outbound, // 0x00000002
    Duplex,   // 0x00000003
}

impl PipeAccess {
    pub fn as_dword(&self) -> DWORD {
        match self {
            PipeAccess::Inbound => 0x00000001,
            PipeAccess::Outbound => 0x00000002,
            PipeAccess::Duplex => 0x00000003,
        }
    }
}

#[allow(unused)]
pub mod access_flags {
    use super::DWORD;
    pub const OVERLAPPED: DWORD = 0x40000000;
    pub const FIRST_INSTANCE: DWORD = 0x00080000;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipeError {
    PipeConnected,      // 535
    BrokenPipe,         // 109
    MoreData,           // 234
    NoData,             // 232
    PipeBusy,           // 231
    PipeNotConnected,   // 233
    IoPending,          // 997
    Unknown(DWORD),
}

impl From<DWORD> for PipeError {
    fn from(code: DWORD) -> Self {
        match code {
            535 => PipeError::PipeConnected,
            109 => PipeError::BrokenPipe,
            234 => PipeError::MoreData,
            232 => PipeError::NoData,
            231 => PipeError::PipeBusy,
            233 => PipeError::PipeNotConnected,
            997 => PipeError::IoPending,
            _ => PipeError::Unknown(code),
        }
    }
}

impl fmt::Display for PipeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pipe Error: {:?}", self)
    }
}

impl std::error::Error for PipeError {}