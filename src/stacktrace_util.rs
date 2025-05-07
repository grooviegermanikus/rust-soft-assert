use log::{debug, log, Level};
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::thread::ThreadId;

pub struct Stracktrace {
    pub frames: Vec<Frame>,
    // simple tagging of stacktrace e.g. 'JuCPL' - use for grepping
    pub hash: u64,
}

pub struct Frame {
    pub method: String,
    pub filename: String,
    pub line_no: u32,
}

pub struct ThreadInfo {
    pub thread_id: ThreadId,
    pub name: String,
}

#[derive(Debug)]
pub enum BacktrackError {
    NoStartFrame,
    NoDebugSymbols,
}

impl Display for ThreadInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO fix format "main:ThreadId(1)" -> how to deal with numeric thread id?
        write!(f, "{}:{:?}", self.name, self.thread_id)
    }
}

impl Display for BacktrackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BacktrackError::NoStartFrame => write!(f, "Start Frame not found!"),
            BacktrackError::NoDebugSymbols => {
                write!(f, "No debug symbols! Did you build in release mode?")
            }
        }
    }
}

impl std::error::Error for BacktrackError {}

/// Returns a list of stack frames starting with innermost frame.
pub fn backtrack_frame(fn_skip_frame: fn(&str) -> bool) -> Result<Stracktrace, BacktrackError> {
    const FRAMES_LIMIT: usize = 99;

    let mut started = false;
    let mut stop = false;
    let mut symbols = 0;
    let mut hasher = DefaultHasher::new();

    // ordering: inside out
    let mut frames: Vec<Frame> = vec![];

    backtrace::trace(|frame| {
        backtrace::resolve_frame(frame, |symbol| {
            // note: values are None for release build
            // sample output:
            // Symbol { name: backtrace::backtrace::trace_unsynchronized::hc02a5cecd085adce,
            //   addr: 0x100001b2a, filename: ".../.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.67/src/backtrace/mod.rs", lineno: 66 }

            if stop {
                return;
            }

            if symbol.filename().is_none() {
                return;
            }

            symbols += 1;

            if frames.len() > FRAMES_LIMIT {
                stop = true;
                return;
            }

            // /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/rt.rs
            if symbol
                .filename()
                .unwrap()
                .starts_with(PathBuf::from("/rustc"))
            {
                stop = true;
                return;
            }

            // symbol.name looks like this "rust_basics::debugging_lock_newtype::backtrack::h1cb6032f9b10548c"
            let symbol_name = symbol.name().unwrap().to_string();
            // module_path is "rust_soft_assert::stacktrace_util"

            if !symbol_name.starts_with("backtrace::backtrace::")
                && !fn_skip_frame(symbol_name.as_str())
            {
                started = true;
                // do not return to catch the current frame
            }

            if !started {
                return;
            }

            let frame = Frame {
                method: symbol.name().unwrap().to_string(),
                filename: symbol
                    .filename()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                line_no: symbol.lineno().unwrap(),
            };

            // hash frame data
            hasher.write(frame.method.as_bytes());
            hasher.write_i32(0x2A66ED); // random separator
            hasher.write(frame.filename.as_bytes());
            hasher.write_i32(0x2A66ED); // random separator
            hasher.write_u32(frame.line_no);
            hasher.write_i32(0xF122ED); // random separator

            frames.push(frame);
        });

        !stop
    });

    if !started {
        if symbols == 0 {
            // detected implicitly by checking frames
            return Err(BacktrackError::NoDebugSymbols);
        } else {
            return Err(BacktrackError::NoStartFrame);
        }
    } else {
        let hash = hasher.finish() as u64;
        return Ok(Stracktrace { frames, hash });
    }
}

fn debug_frames(frames: &Result<Vec<Frame>, BacktrackError>) {
    for frame in frames.as_ref().unwrap() {
        println!("\t>{}:{}:{}", frame.filename, frame.method, frame.line_no);
    }
}

pub fn log_frames(level: Level, msg: &str, stacktrace: &Stracktrace) {
    log!(level, " |->\t{}:", msg);
    for frame in &stacktrace.frames {
        log!(
            level,
            " |->\t  {}!{}:{}",
            frame.filename,
            frame.method,
            frame.line_no
        );
    }
}

pub fn get_current_stracktrace() -> Result<Stracktrace, BacktrackError> {
    // covers:
    // rust_soft_assert::debugging_locks::
    // rust_soft_assert::stacktrace_util::
    const OMIT_FRAME_SUFFIX1: &str = "rust_soft_assert:";
    // <rust_soft_assert::debugging_locks::RwLockWrapped<T> as core::default::Default>::default::haed7701ba5f48aa2:97
    const OMIT_FRAME_SUFFIX2: &str = "<rust_soft_assert:";
    backtrack_frame(|symbol_name| {
        symbol_name.starts_with(OMIT_FRAME_SUFFIX1) || symbol_name.starts_with(OMIT_FRAME_SUFFIX2)
    })
}

#[derive(Debug, Clone)]
pub struct AllocationTracker {}

impl AllocationTracker {
    pub fn new() -> Self {
        let stracktrace = get_current_stracktrace().unwrap();
        log_frames(Level::Info, "AllocationTracker::new", &stracktrace);
        AllocationTracker {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stacktrace_from_method() {
        let _ = tracing_subscriber::fmt::try_init();

        let stacktrace = caller_function().unwrap();
        log_frames(Level::Info, "stacktrace_from_method", &stacktrace);
        assert!(
            stacktrace
                .frames
                .get(0)
                .unwrap()
                .method
                .starts_with("rust_soft_assert::stacktrace_util::tests::caller_function::h"),
            "method name: {}",
            stacktrace.frames.get(0).unwrap().method
        );
    }

    fn caller_function() -> Result<Stracktrace, BacktrackError> {
        backtrack_frame(|symbol_name| !symbol_name.contains("::caller_function"))
    }
}
