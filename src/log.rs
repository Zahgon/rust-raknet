use std::sync::atomic::{AtomicU8, Ordering};

pub static ENABLE_RAKNET_LOG: AtomicU8 = AtomicU8::new(0);

pub fn enable_raknet_log(flag: u8) { panic!("STUB: not implemented") }

#[macro_export]
macro_rules! raknet_log_debug {
    ($($arg:tt)*) => ({
        if $crate::log::ENABLE_RAKNET_LOG.load(std::sync::atomic::Ordering::Relaxed) & 1 != 0 {
            let now = $crate::utils::cur_timestamp_millis();
            let msg = format!($($arg)*);
            println!("debug - {} - {} - {}" , now , "raknet" , msg);
        }
    })
}

#[macro_export]
macro_rules! raknet_log_error {
    ($($arg:tt)*) => ({
        if $crate::log::ENABLE_RAKNET_LOG.load(std::sync::atomic::Ordering::Relaxed) & 2 != 0 {
            let now = $crate::utils::cur_timestamp_millis();
            let msg = format!($($arg)*);
            println!("error - {} - {} - {}" , now , "raknet" , msg);
        }
    })
}

#[macro_export]
macro_rules! raknet_log_info {
    ($($arg:tt)*) => ({
        if $crate::log::ENABLE_RAKNET_LOG.load(std::sync::atomic::Ordering::Relaxed) & 4 != 0 {
            let now = $crate::utils::cur_timestamp_millis();
            let msg = format!($($arg)*);
            println!("info - {} - {} - {}" , now , "raknet" , msg);
        }
    })
}
