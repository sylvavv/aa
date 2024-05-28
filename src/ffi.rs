#[link(name = "kernel32")]
extern "system" {

    pub(crate) fn GetSystemInfo(lpSystemInfo: *mut crate::types::SystemInfo);

    pub(crate) fn GetSystemFirmwareTable(
        FirmwareTableProviderSignature: u32,
        FirmwareTableID: u32,
        pFirmwareTableBuffer: *mut u8,
        BufferSize: u32,
    ) -> u32;

    // pub(crate) fn AllocConsole() -> i32;

    // pub(crate) fn FreeConsole() -> i32;

    // pub(crate) fn SetConsoleMode(hConsoleHandle: *mut ::core::ffi::c_void, dwMode: u32) -> i32;

    // pub(crate) fn GetStdHandle(nStdHandle: u32) -> *mut ::core::ffi::c_void;

    // pub(crate) fn GetConsoleMode(hConsoleHandle: *mut ::core::ffi::c_void, lpMode: *mut u32)
    //     -> i32;

    pub(crate) fn GlobalAlloc(uFlags: u32, dwBytes: usize) -> *mut ::core::ffi::c_void;

    pub(crate) fn GlobalLock(hMem: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;

    pub(crate) fn GlobalUnlock(hMem: *mut ::core::ffi::c_void) -> i32;

    // pub(crate) fn GlobalFree(hMem: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;

    pub(crate) fn GlobalSize(hMem: *mut ::core::ffi::c_void) -> usize;

}

#[link(name = "user32")]
extern "system" {

    pub(crate) fn OpenClipboard(hWndNewOwner: *mut ::core::ffi::c_void) -> i32;

    pub(crate) fn EmptyClipboard() -> i32;

    pub(crate) fn CloseClipboard() -> i32;

    pub(crate) fn GetClipboardData(uFormat: u32) -> *mut ::core::ffi::c_void;

    pub(crate) fn SetClipboardData(
        uFormat: u32,
        hMem: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]

pub(crate) struct RawSMBIOSData {
    pub(crate) used20_calling_method: u8,
    pub(crate) smbiosmajor_version: u8,
    pub(crate) smbiosminor_version: u8,
    pub(crate) dmi_revision: u8,
    pub(crate) length: u32,
    pub(crate) smbiostable_data: Vec<u8>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]

pub(crate) struct DmiHeader {
    pub(crate) type_: u8,
    pub(crate) length: u8,
    pub(crate) handle: u16,
}
