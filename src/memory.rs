use std::ffi::c_void;

use windows::Win32::{Foundation::HANDLE, System::Threading::{PROCESS_ALL_ACCESS}};

pub struct ProcessMemory {
    bytes: Vec<u8>,
    virtual_address: usize,
    pid: u32,
    handle: HANDLE
}

impl ProcessMemory {
    pub fn new(virtual_address: usize, capacity: usize, pid: u32) -> ProcessMemory {
        let handle = unsafe { windows::Win32::System::Threading::OpenProcess(PROCESS_ALL_ACCESS, false, pid)
            .expect("Could not open process.") };

        ProcessMemory {
            bytes: vec![0; capacity],
            pid: pid,
            virtual_address: virtual_address,
            handle: handle
        }
    }

    pub fn read(&mut self) -> &[u8] {
        unsafe {
            let _ = windows::Win32::System::Diagnostics::Debug::ReadProcessMemory(
                self.handle, 
                self.virtual_address as *const c_void, 
                self.bytes.as_mut_ptr() as *mut c_void,
                self.bytes.len(), 
                None
            );
        }

        &self.bytes
    }

    pub fn write(&mut self, physical_address: usize, write_bytes: &[u8]) {
        let write_address = physical_address + self.virtual_address;
        unsafe {
            let res = windows::Win32::System::Diagnostics::Debug::WriteProcessMemory(
                self.handle, 
                write_address as *const c_void, 
                write_bytes.as_ptr() as *mut c_void, 
                write_bytes.len(), 
                None
            );

            if let Err(e) = res {
                println!("Res: {}", e);
            }
        }
    }
}

impl Drop for ProcessMemory {
    fn drop(&mut self) {
        if !self.handle.is_invalid() {
            unsafe {
                let _ = windows::Win32::Foundation::CloseHandle(self.handle);
            }
        }
    }
}