// ============================================================================
//  Mission 1: one error type for the whole library (Chapter 9)
// ============================================================================

use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum RagError {
    // Reading a file or a directory failed. We keep the original `io::Error` inside.
    Io(io::Error),
    // A file had the wrong format. `file` and `line` say where, `reason` says what was wrong.
    Parse {
        file: String,
        line: usize,
        reason: String,
    },
    // The directory exists but has no `.md` files in it.
    EmptyCorpus(PathBuf),
}

// MISSION 1a: how the error looks to a human, e.g. when `main` prints `error: {e}`.
//   Io(e)                      -> "io error: <e>"
//   Parse { file, line, reason } -> "<file>:<line>: <reason>"
//   EmptyCorpus(dir)           -> "no .md files in <dir>"   (use `dir.display()`)
impl fmt::Display for RagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RagError::Io(e) => write!(f, "io error: {}", e),
            RagError::Parse { file, line, reason } => write!(f, "{}:{}: {}", file, line, reason),
            RagError::EmptyCorpus(dir) => write!(f, "no .md files in {}", dir.display()),
        }
    }
}

// MISSION 1b: let `?` turn an `io::Error` into a `RagError` automatically.
impl From<io::Error> for RagError {
    fn from(e: io::Error) -> RagError {
        RagError::Io(e)
    }
}

// Already done, but read it: this marks `RagError` as a standard error type.
// `std::error::Error` has default implementations for all of its methods (Chapter 10.2,
// "Default Implementations"), and it requires `Debug + Display`, which we have once Mission 1a
// is done. So the whole impl is empty.
impl std::error::Error for RagError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mission_01_display() {
        let e = RagError::Parse {
            file: String::from("queries.tsv"),
            line: 3,
            reason: String::from("missing tab"),
        };
        assert_eq!(e.to_string(), "queries.tsv:3: missing tab");

        let e = RagError::EmptyCorpus(PathBuf::from("notes"));
        assert_eq!(e.to_string(), "no .md files in notes");

        let e = RagError::Io(io::Error::new(io::ErrorKind::NotFound, "gone"));
        assert_eq!(e.to_string(), "io error: gone");
    }

    #[test]
    fn mission_01_question_mark_converts_io_errors() {
        fn read_missing() -> Result<String, RagError> {
            let text = std::fs::read_to_string("this/file/does/not/exist.md")?;
            Ok(text)
        }
        match read_missing() {
            Err(RagError::Io(e)) => assert_eq!(e.kind(), io::ErrorKind::NotFound),
            other => panic!("expected RagError::Io, got {other:?}"),
        }
    }

    #[test]
    fn mission_01_is_a_std_error() {
        // A generic function with a trait bound (Chapter 10.2). It only compiles if
        // `RagError` implements `std::error::Error`.
        fn assert_is_error<E: std::error::Error>(_: &E) {}
        assert_is_error(&RagError::EmptyCorpus(PathBuf::from(".")));
    }
}
