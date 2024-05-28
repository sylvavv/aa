mod ffi;
mod types;

pub type AnyResult<T> = Result<T, Box<dyn ::std::error::Error>>;

#[allow(non_snake_case, non_camel_case_types)]
#[repr(u32)]
pub enum ClipboardFormats {
    CF_TEXT = 1,
    CF_BITMAP = 2,
    CF_METAFILEPICT = 3,
    CF_SYLK = 4,
    CF_DIF = 5,
    CF_TIFF = 6,
    CF_OEMTEXT = 7,
    CF_DIB = 8,
    CF_PALETTE = 9,
    CF_PENDATA = 10,
    CF_RIFF = 11,
    CF_WAVE = 12,
    CF_UNICODETEXT = 13,
    CF_ENHMETAFILE = 14,

    CF_HDROP = 15,
    CF_LOCALE = 16,

    CF_DIBV5 = 17,
}

pub unsafe fn get_system_info() -> types::SystemInfo {
    let mut system_info: types::SystemInfo = ::core::mem::zeroed::<types::SystemInfo>();

    ffi::GetSystemInfo(&mut system_info);

    system_info
}

unsafe fn get_string_by_dmi(dm_header: *const ffi::DmiHeader, mut index: u8) -> AnyResult<String> {
    if index == 0 {
        return Err(format!("[{}:{}]", file!(), line!()).into());
    }

    let mut base_address: *const i8 = dm_header.cast::<i8>().add(dm_header.read().length as usize);

    while index > 1 && base_address.read() != 0 {
        let strlen = ::std::ffi::CStr::from_ptr(base_address).to_str()?.len();

        base_address = base_address.add(strlen + 1);

        index -= 1;
    }

    if base_address.read() == 0 {
        return Err(format!("[{}:{}]", file!(), line!()).into());
    }

    let strlen: usize = ::std::ffi::CStr::from_ptr(base_address).to_str()?.len();

    let sm_data: Vec<u8> =
        ::std::slice::from_raw_parts(base_address.cast::<u8>(), strlen + 1).to_vec();

    let sm_cstring: ::std::ffi::CString = ::std::ffi::CString::from_vec_with_nul(sm_data)?;

    let result: String = sm_cstring.to_str()?.trim_end_matches('\0').to_string();

    Ok(result)
}

pub unsafe fn get_dmi_info() -> AnyResult<types::DmiInformation> {
    let signature = u32::from_be_bytes(*b"RSMB");

    let buf_size: u32 = ffi::GetSystemFirmwareTable(signature, 0, ::core::ptr::null_mut(), 0);

    let mut buffer: Vec<u8> = vec![0; buf_size as usize];

    let return_length = ffi::GetSystemFirmwareTable(signature, 0, buffer.as_mut_ptr(), buf_size);

    if return_length == 0 {
        return Err(::std::io::Error::last_os_error().into());
    }

    if return_length > buf_size {
        return Err(format!("[{}:{}]", file!(), line!()).into());
    }

    let smb: ffi::RawSMBIOSData = ffi::RawSMBIOSData {
        used20_calling_method: buffer[0],
        smbiosmajor_version: buffer[1],
        smbiosminor_version: buffer[2],
        dmi_revision: buffer[3],
        length: u32::from_ne_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
        smbiostable_data: buffer[8..].to_vec(),
    };

    let mut dmi_info: types::DmiInformation = types::DmiInformation::default();

    let mut uuid: [u8; 16] = [0; 16];

    let mut sm_data: *const u8 = smb.smbiostable_data.as_ptr();

    let mut once_flag: bool = false;

    while sm_data < smb.smbiostable_data.as_ptr().add(smb.length as usize) {
        let dmi_header: *const ffi::DmiHeader = sm_data.cast();

        if dmi_header.read().length < 4 {
            break;
        }

        if dmi_header.read().type_ == 0 && once_flag == false {
            if let Ok(bios_vendor) = get_string_by_dmi(dmi_header, sm_data.offset(0x4).read()) {
                dmi_info.bios_vendor = bios_vendor;
            }

            if let Ok(bios_version) = get_string_by_dmi(dmi_header, sm_data.offset(0x5).read()) {
                dmi_info.bios_version = bios_version;
            }

            if let Ok(bios_release_date) = get_string_by_dmi(dmi_header, sm_data.offset(0x8).read())
            {
                dmi_info.bios_release_date = bios_release_date;
            }

            if sm_data.offset(0x16).read() != 0xFF && sm_data.offset(0x17).read() != 0xFF {
                dmi_info.bios_embedded_controller_firmware_version = format!(
                    "{}.{}",
                    sm_data.offset(0x16).read(),
                    sm_data.offset(0x17).read()
                );
            }

            once_flag = true;
        }

        if dmi_header.read().type_ == 0x01 && dmi_header.read().length >= 0x19 {
            if let Ok(manufacturer) = get_string_by_dmi(dmi_header, sm_data.offset(0x4).read()) {
                dmi_info.system_manufacturer = manufacturer;
            }

            if let Ok(product) = get_string_by_dmi(dmi_header, sm_data.offset(0x5).read()) {
                dmi_info.system_product = product;
            }

            if let Ok(version) = get_string_by_dmi(dmi_header, sm_data.offset(0x6).read()) {
                dmi_info.system_version = version;
            }

            if let Ok(serial_number) = get_string_by_dmi(dmi_header, sm_data.offset(0x7).read()) {
                dmi_info.system_serial_number = serial_number;
            }

            if let Ok(sku_number) = get_string_by_dmi(dmi_header, sm_data.offset(0x19).read()) {
                dmi_info.system_sku_number = sku_number;
            }

            if let Ok(family) = get_string_by_dmi(dmi_header, sm_data.offset(0x1A).read()) {
                dmi_info.system_family = family;
            }

            sm_data = sm_data.add(0x8);

            let mut all_zero: bool = true;

            let mut all_one: bool = true;

            let mut i: isize = 0;

            while i < 16 && (all_zero || all_one) {
                if sm_data.offset(i).read() != 0x00 {
                    all_zero = false;
                }

                if sm_data.offset(i).read() != 0xFF {
                    all_one = false;
                }

                i += 1;
            }

            if !all_zero && !all_one {
                for i in 0..4 {
                    uuid[i] = sm_data.add(i).read();
                }

                uuid[5] = sm_data.offset(5).read();

                uuid[4] = sm_data.offset(4).read();

                uuid[7] = sm_data.offset(7).read();

                uuid[6] = sm_data.offset(6).read();

                for j in 8..16 {
                    uuid[j] = sm_data.add(j).read();
                }

                let mut uuid_string: String = String::new();

                for i in 0..16 {
                    uuid_string.push_str(format!("{:02X}", uuid[i]).as_str());

                    if (i + 1) % 4 == 0 && i != 15 {
                        uuid_string.push('-');
                    }
                }

                let mut guid: [u8; 16] = uuid;

                guid[0] = uuid[3];

                guid[1] = uuid[2];

                guid[2] = uuid[1];

                guid[3] = uuid[0];

                guid[4] = uuid[5];

                guid[5] = uuid[4];

                guid[6] = uuid[7];

                guid[7] = uuid[6];

                dmi_info.system_uuid = (uuid, uuid_string);

                let mut guid_string: String = String::new();

                for i in 0..16 {
                    guid_string.push_str(format!("{:02X}", guid[i]).as_str());

                    if i == 3 {
                        guid_string.push('-');
                    }

                    if i % 2 == 1 && i < 10 && i > 4 {
                        guid_string.push('-');
                    }
                }

                dmi_info.system_guid = (guid, guid_string);
            }

            break;
        }

        let mut next: *const u8 = sm_data.add(dmi_header.read().length as usize);

        while next < smb.smbiostable_data.as_ptr().add(smb.length as usize)
            && (next.offset(0).read() != 0 || next.offset(1).read() != 0)
        {
            next = next.add(1);
        }

        sm_data = next.add(2);
    }

    Ok(dmi_info)
}

pub unsafe fn set_clipboard_data(
    bytes_p: *const u8,
    num_bytes: usize,
    cf: ClipboardFormats,
) -> AnyResult<()> {
    if ffi::OpenClipboard(::core::ptr::null_mut()) == 0 {
        return Err(::std::io::Error::last_os_error().into());
    }

    if ffi::EmptyClipboard() == 0 {
        if ffi::CloseClipboard() == 0 {
            return Err(::std::io::Error::last_os_error().into());
        }

        return Err(::std::io::Error::last_os_error().into());
    }

    let global_allocted_mem: *mut ::core::ffi::c_void = ffi::GlobalAlloc(0x2, num_bytes);

    if global_allocted_mem.is_null() {
        if ffi::CloseClipboard() == 0 {
            return Err(::std::io::Error::last_os_error().into());
        }

        return Err(::std::io::Error::last_os_error().into());
    }

    let global_locked_mem = ffi::GlobalLock(global_allocted_mem);

    if global_locked_mem.is_null() {
        if ffi::CloseClipboard() == 0 {
            return Err(::std::io::Error::last_os_error().into());
        }

        return Err(::std::io::Error::last_os_error().into());
    }

    ::std::ptr::copy_nonoverlapping(bytes_p, global_locked_mem.cast(), num_bytes);

    ffi::GlobalUnlock(global_allocted_mem);

    if ffi::SetClipboardData(cf as u32, global_locked_mem).is_null() {
        if ffi::CloseClipboard() == 0 {
            return Err(::std::io::Error::last_os_error().into());
        }

        return Err(::std::io::Error::last_os_error().into());
    };

    if ffi::CloseClipboard() == 0 {
        return Err(::std::io::Error::last_os_error().into());
    }

    Ok(())
}

pub unsafe fn get_clipboard_data<T>(cf: ClipboardFormats) -> AnyResult<Vec<T>> {
    if ffi::OpenClipboard(::core::ptr::null_mut()) == 0 {
        return Err(::std::io::Error::last_os_error().into());
    }

    let clip_board_data_handle = ffi::GetClipboardData(cf as u32);

    if clip_board_data_handle.is_null() {
        if ffi::CloseClipboard() == 0 {
            return Err(::std::io::Error::last_os_error().into());
        }

        return Err(::std::io::Error::last_os_error().into());
    }

    let global_locked_mem: *mut ::core::ffi::c_void = ffi::GlobalLock(clip_board_data_handle);

    if global_locked_mem.is_null() {
        if ffi::CloseClipboard() == 0 {
            return Err(::std::io::Error::last_os_error().into());
        }

        return Err(::std::io::Error::last_os_error().into());
    }

    let data_len = ffi::GlobalSize(clip_board_data_handle) as usize / ::core::mem::size_of::<T>();

    if data_len == 0 {
        if ffi::CloseClipboard() == 0 {
            return Err(::std::io::Error::last_os_error().into());
        }

        return Err(::std::io::Error::last_os_error().into());
    }

    let mut data: Vec<T> = Vec::with_capacity(data_len);

    data.set_len(data_len);

    std::ptr::copy_nonoverlapping(global_locked_mem.cast(), data.as_mut_ptr(), data_len);

    if ffi::GlobalUnlock(global_locked_mem) == 0 {
        if ffi::CloseClipboard() == 0 {
            return Err(::std::io::Error::last_os_error().into());
        }

        return Err(::std::io::Error::last_os_error().into());
    }

    if ffi::CloseClipboard() == 0 {
        return Err(::std::io::Error::last_os_error().into());
    }

    Ok(data)
}
