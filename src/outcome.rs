use std::io::Write;


/// Enum representing all possible values the application can fail.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Outcome {
    /// No errors occured, everything executed correctly.
    NoError,
    /// The specified file would need to be overriden but was not specified to.
    OverrideNoForce(String),
}

impl Outcome {
    /// Get the executable exit value from an `Outcome` instance.
    pub fn print_error<W: Write>(&self, err_out: &mut W) {
        match *self {
            Outcome::NoError => (),
            Outcome::OverrideNoForce(ref fname) => {
                writeln!(err_out, "File \"{}\" was not overriden to prevent data loss.", fname).unwrap();
                writeln!(err_out, "Pass --force to override it.").unwrap();
            }
        }
    }

    /// Get the executable exit value from an `Outcome` instance.
    pub fn exit_value(&self) -> i32 {
        match *self {
            Outcome::NoError => 0,
            Outcome::OverrideNoForce(_) => 1,
        }
    }
}
