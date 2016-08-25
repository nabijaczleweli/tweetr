#[macro_use]
extern crate clap;

mod options;

pub use options::Options;


fn main() {
    let opts = Options::parse();
    println!("{:#?}", opts);
}
