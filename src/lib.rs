/*
    Lightweight Windows named pipe bindings for Rust
    (c) alberkamyu, 2026

    src/lib.rs
*/

use std::os::windows::ffi::OsStrExt;

use crate::pipe::{CloseHandle, CreateNamedPipeW, GetLastError, MaxInstances, PipeAccess, PipeError, PipeReadMode, PipeType, PipeWaitMode, ReadFile, WriteFile, access_flags};
use crate::wintypes::{DWORD, HANDLE, INVALID_HANDLE_VALUE, LPCVOID, LPVOID};

pub(crate) mod wintypes;
pub mod pipe;

fn to_wide_null(s: &str) -> Vec<u16> {
    std::ffi::OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

pub struct PipeConfig {
    pub name: String,
    pub access: PipeAccess,
    pub pipe_type: PipeType,
    pub read_mode: PipeReadMode,
    pub wait_mode: PipeWaitMode,
    pub max_instances: MaxInstances,
    pub out_buffer_size: u32,
    pub in_buffer_size: u32,
    pub default_timeout: u32,
}

impl PipeConfig {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            access: PipeAccess::Duplex,
            pipe_type: PipeType::Byte,
            read_mode: PipeReadMode::Byte,
            wait_mode: PipeWaitMode::Wait,
            max_instances: MaxInstances::Unlimited,
            out_buffer_size: 4096,
            in_buffer_size: 4096,
            default_timeout: 0,
        }
    }
}

pub struct NamedPipe {
    handle: HANDLE
}

impl Drop for NamedPipe {
    fn drop(&mut self) {
        unsafe {
            if self.handle != INVALID_HANDLE_VALUE {
                CloseHandle(self.handle);
            }
        }
    }
}

impl NamedPipe {
    pub fn create(config: PipeConfig) -> Result<Self, PipeError> {
        let name = to_wide_null(&config.name);

        let handle = unsafe {
            CreateNamedPipeW(
                name.as_ptr(),
                config.access.as_dword() | access_flags::FIRST_INSTANCE,
                config.pipe_type.as_dword() | config.read_mode.as_dword() | config.wait_mode.as_dword(),
                config.max_instances.as_dword(),
                config.out_buffer_size,
                config.in_buffer_size,
                config.default_timeout,
                std::ptr::null_mut(),
            )
        };

        if handle == INVALID_HANDLE_VALUE {
            return Err(PipeError::from(unsafe { GetLastError() }));
        }

        Ok(Self { handle })
    }

    pub fn write(&self, data: &[u8]) -> Result<usize, PipeError> {
        let mut written: DWORD = 0;

        let ok = unsafe {
            WriteFile(
                self.handle,
                data.as_ptr() as LPCVOID,
                data.len() as DWORD,
                &mut written,
                std::ptr::null_mut(),
            )
        };

        if ok == 0 {
            return Err(PipeError::from(unsafe { GetLastError() }));
        }

        Ok(written as usize)
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<usize, PipeError> {
        let mut read: DWORD = 0;

        let ok = unsafe {
            ReadFile(
                self.handle,
                buf.as_mut_ptr() as LPVOID,
                buf.len() as DWORD,
                &mut read,
                std::ptr::null_mut(),
            )
        };

        if ok == 0 {
            return Err(PipeError::from(unsafe { GetLastError() }));
        }

        Ok(read as usize)
    }

    pub fn wait_for_client(&self) -> Result<(), PipeError> {
        let ok = unsafe { 
            crate::pipe::ConnectNamedPipe(self.handle, std::ptr::null_mut()) 
        };

        if ok == 0 {
            let err = unsafe { crate::pipe::GetLastError() };
            let pipe_err = PipeError::from(err);

            if let PipeError::PipeConnected = pipe_err {
                return Ok(());
            }

            return Err(pipe_err);
        }

        Ok(())
    }

    pub fn disconnect(&self) -> Result<(), PipeError> {
        let ok = unsafe { 
            crate::pipe::DisconnectNamedPipe(self.handle) 
        };

        if ok == 0 {
            return Err(PipeError::from(unsafe { crate::pipe::GetLastError() }));
        }

        Ok(())
    }
}

unsafe impl Send for NamedPipe {}
unsafe impl Sync for NamedPipe {}