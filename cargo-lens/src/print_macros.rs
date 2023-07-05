#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug_socket")]
        std::eprint!($($arg)*);
    };
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug_socket")]
        std::eprintln!($($arg)*);
    };
}
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug_socket")]
        std::eprint!($($arg)*);
    };
}

#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug_socket")]
        std::eprintln!($($arg)*);
    };
}
