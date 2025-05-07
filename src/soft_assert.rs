#[macro_export]
macro_rules! soft_assert {
    ($cond:expr $(,)?) => {
        if !$cond {
            log::error!(
                "Soft assert failed: {}\nStack trace:\n{}",
                stringify!($cond),
                std::backtrace::Backtrace::capture().to_string()
            );
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        if !$cond {

            log::error!(
                "Soft assert failed: {}\nStack trace:\n{}",
                format_args!($($arg)+),
                std::backtrace::Backtrace::capture().to_string()
            );
        }
    };
}

#[macro_export]
macro_rules! soft_assert_eq {
    ($left:expr, $right:expr $(,)?) => {
        if $left != $right {
            log::error!(
                "Soft assert failed: {} != {}\nStack trace:\n{}",
                stringify!($left),
                stringify!($right),
                std::backtrace::Backtrace::capture().to_string()
            );
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        if $left != $right {
            log::error!(
                "Soft assert failed: {} != {}\nMessage: {}\nStack trace:\n{}",
                stringify!($left),
                stringify!($right),
                format_args!($($arg)+),
                std::backtrace::Backtrace::capture().to_string()
            );
        }
    };
}

#[macro_export]
macro_rules! soft_assert_ne {
    ($left:expr, $right:expr $(,)?) => {
        if $left == $right {
            log::error!(
                "Soft assert failed: {} == {}\nStack trace:\n{}",
                stringify!($left),
                stringify!($right),
                std::backtrace::Backtrace::capture().to_string()
            );
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        if $left == $right {
            log::error!(
                "Soft assert failed: {} == {}\nMessage: {}\nStack trace:\n{}",
                stringify!($left),
                stringify!($right),
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
}

#[test]
fn test_soft_with_args() {
    let _ = tracing_subscriber::fmt::fmt().try_init();

    soft_assert!(false, "My fuzzy text with args: {} {}", 42, "foo");
}

#[test]
fn test_soft_assert_eq() {
    let _ = tracing_subscriber::fmt::fmt().try_init();

    // This should log an error because 1 != 2
    soft_assert_eq!(1, 2, "Values are not equal: {} and {}", 1, 2);

    // This should not log an error because 1 == 1
    soft_assert_eq!(1, 1);
}

#[test]
fn test_soft_assert_ne() {
    let _ = tracing_subscriber::fmt::fmt().try_init();

    // This should log an error because 1 == 1
    soft_assert_ne!(1, 1, "Values are equal: {} and {}", 1, 1);

    // This should not log an error because 1 != 2
    soft_assert_ne!(1, 2);
}
