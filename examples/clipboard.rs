fn main() {
    let bytes = "Hello!\0".encode_utf16().collect::<Vec<u16>>();

    unsafe {
        aa::set_clipboard_data(
            bytes.as_ptr().cast(),
            bytes.len() * 2,
            aa::ClipboardFormats::CF_UNICODETEXT,
        )
        .unwrap();
    }
}
