pub fn log(s: &str) {
    unsafe { platform::_log(s.as_ptr() as usize, s.len() as usize) }
}

mod platform {
    extern "C" {
        #[link(name = "_log")]
        pub fn _log(s: usize, len: usize);
    }
}
