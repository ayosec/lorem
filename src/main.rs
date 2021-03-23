//! CLI program to generate random text.

mod options;

use std::io::{self, Write};

use options::CountType;

fn main() {
    let options = match options::get() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    // Build words generator.
    let mut chain = lipsum::MarkovChain::new();

    // TODO change learn from options.source
    chain.learn(lipsum::LOREM_IPSUM);
    chain.learn(lipsum::LIBER_PRIMUS);

    // Use buffered output.
    let stdout_handle = io::stdout();
    let mut stdout = io::BufWriter::new(stdout_handle.lock());

    macro_rules! w {
        ($($tt:tt)*) => {
            if write!(&mut stdout, $($tt)*).is_err() {
                return;
            }
        }
    }

    let mut count = 0;
    let mut sentence_start = true;

    for word in chain.iter() {
        let num_chars = word.chars().count();
        count += match options.count_type {
            CountType::Chars => num_chars,
            CountType::Words => 1,
        };

        if count > options.count {
            break;
        }

        // TODO wrap content

        if sentence_start {
            // Convert the first letter in the sentence to uppercase.
            let mut chars = word.chars();
            if let Some(c) = chars.next() {
                w!("{}", c.to_uppercase());
                for c in chars {
                    w!("{}", c);
                }
            }
        } else {
            w!(" {}", word);
        }

        sentence_start = word.ends_with('.');
        if sentence_start {
            w!(" ");
        }
    }

    if !sentence_start {
        w!(".");
    }

    w!("\n");
}
