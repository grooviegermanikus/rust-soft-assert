
#[macro_export]
macro_rules! soft_assert {
    ($cond:expr $(,)?) => {
        if !$cond {
            log::warn!(
                "Soft assert failed: {}\nStack trace:\n{}",
                stringify!($cond),
                std::backtrace::Backtrace::capture().to_string()
            );
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        if !$cond {

            log::warn!(
                "Soft assert failed: {}\nStack trace:\n{}",
                format_args!($($arg)+),
                std::backtrace::Backtrace::capture().to_string()
            );
        }
    };
}



#[test]
fn test_soft() {
    let _ = tracing_subscriber::fmt::fmt().try_init();

    soft_assert!(false, "My fuzzy text");
    soft_assert!(false, "My fuzzy text with args: {} {}", 42, "foo");

}