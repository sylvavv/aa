fn main() {
    unsafe {
        let dmi_info = aa::get_dmi_info().unwrap();

        println!("{:#X?}", dmi_info);
    }
}
