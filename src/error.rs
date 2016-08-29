use std::io::Write;


/// Enum representing all possible values the application can fail.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Error {
    /// No errors occured, everything executed correctly.
    NoError,
    /// The specified file would need to be overriden but was not specified to.
    OverrideNoForce(String),
}

impl Error {
    /// Get the executable exit value from an `Error` instance.
    pub fn print_error<W: Write>(&self, err_out: &mut W) {
        match *self {
            Error::NoError => (),
            Error::OverrideNoForce(ref fname) => {
                writeln!(err_out, "File \"{}\" was not overriden to prevent data loss.", fname).unwrap();
                writeln!(err_out, "Pass --force to override it.").unwrap();
            },
        }
    }

    /// Get the executable exit value from an `Error` instance.
    pub fn exit_value(&self) -> i32 {
        match *self {
            Error::NoError => 0,
            Error::OverrideNoForce(_) => 1,
        }
    }
}
