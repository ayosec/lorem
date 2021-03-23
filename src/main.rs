//! CLI program to generate random text.

mod options;

fn main() {
    let options = match options::get() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    dbg!(options);
}
