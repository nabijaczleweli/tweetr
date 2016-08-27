extern crate egg_mode;
#[macro_use]
extern crate clap;

pub mod options;


fn main() {
    let opts = options::Options::parse();
    println!("{:#?}", opts);
}
