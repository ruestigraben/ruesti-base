#[cfg(feature = "ruesti_test")]
mod test {
    use log::*;

    #[no_mangle]
    pub fn __test() {
        info!("Hello world");
    }
}
