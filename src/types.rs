#[repr(C)]
#[derive(Clone, Copy)]
pub struct SystemInfo {
    pub dummy_union: SystemInfoDummyUnion,
    pub dw_page_size: u32,
    pub lp_minimum_application_address: *mut ::core::ffi::c_void,
    pub lp_maximum_application_address: *mut ::core::ffi::c_void,
    pub dw_active_processor_mask: usize,
    pub dw_number_of_processors: u32,
    pub dw_processor_type: u32,
    pub dw_allocation_granularity: u32,
    pub w_processor_level: u16,
    pub w_processor_revision: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union SystemInfoDummyUnion {
    pub dw_oem_id: u32,
    pub dummy_struct: SystemInfoDummyStruct,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemInfoDummyStruct {
    pub w_processor_architecture: u16,
    pub w_reserved: u16,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]

pub struct DmiInformation {
    pub bios_version: String,
    pub bios_release_date: String,
    pub bios_vendor: String,
    pub bios_embedded_controller_firmware_version: String,

    pub system_manufacturer: String,
    pub system_product: String,
    pub system_version: String,
    pub system_serial_number: String,
    pub system_uuid: ([u8; 16], String),
    pub system_guid: ([u8; 16], String),
    pub system_sku_number: String,
    pub system_family: String,
}
