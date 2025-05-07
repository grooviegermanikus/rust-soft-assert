
#[macro_export]
macro_rules! soft_assert {
    ($cond:expr $(,)?) => {
        if !$cond {
            log::warn!(
                "Soft assert failed: {}\nStack trace:\n{:?}",
                stringify!($cond),
                std::backtrace::Backtrace::capture()
            );
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        if !$cond {
            log::warn!(
                "Soft assert failed: {}\nStack trace:\n{:?}",
                format_args!($($arg)+),
                std::backtrace::Backtrace::capture()
            );
        }
    };
}


#[test]
fn test_soft() {
    let _ = tracing_subscriber::fmt::try_init();
    
    soft_assert!(false, "This is a soft assert");


}